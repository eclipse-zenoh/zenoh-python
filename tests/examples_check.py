# Copyright (c) 2017, 2022 ZettaScale Technology Inc.
import re
import sys
import time
import uuid
from glob import glob
from os import getpgid, killpg, path
from select import select
from signal import SIGINT, SIGKILL
from subprocess import PIPE, Popen, TimeoutExpired

import fixtures
import pytest
from common.testing import discovery_config, isolated_config, open_listener

import zenoh

# Contributors:
#   ZettaScale Zenoh team, <zenoh@zettascale.tech>
#
# This program and the accompanying materials are made available under the
# terms of the Eclipse Public License 2.0 which is available at
# http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
# which is available at https://www.apache.org/licenses/LICENSE-2.0.

# SPDX-License-Identifier: EPL-2.0 OR Apache-2.0


examples = path.realpath(__file__).split("/tests")[0] + "/examples/"
docs_examples = path.realpath(__file__).split("/tests")[0] + "/docs/examples/"
tab = "\t"
ret = "\r\n"


class Pyrun(fixtures.Fixture):
    def __init__(self, p, args=None, basedir=None, timeout=30, network=None) -> None:
        if args is None:
            args = []
        if basedir is None:
            basedir = examples
        if network is not None:
            args = [
                *args,
                "--mode",
                "client",
                "--no-multicast-scouting",
                "--cfg",
                "scouting/gossip/enabled:false",
                "--cfg",
                "listen/endpoints:[]",
                "--cfg",
                "connect/timeout_ms:5000",
                "--cfg",
                "connect/exit_on_failure:true",
                "--connect",
                network,
            ]
        self.name = p
        self.timeout = timeout
        print(f"starting {self.name}")
        self.process: Popen = Popen(
            [sys.executable, "-u", path.join(basedir, p), *args],
            stdout=PIPE,
            stderr=PIPE,
            # Keep select() in wait_for_output in sync with unread pipe data.
            bufsize=0,
            start_new_session=True,
        )
        self.start = time.time()
        self.end = None
        self.errors = []
        self._stdouts = []
        self._stderrs = []

    def _setUp(self):
        # Always reap the process.  Merely sending SIGINT leaves descendants
        # behind when a test fails or a process does not handle the signal.
        self.addCleanup(self._cleanup)

    def dbg(self):
        self.wait()
        print(f"{self.name} stdout:")
        print(f"{tab}{tab.join(self.stdout)}")
        print(f"{self.name} stderr:")
        print(f"{tab}{tab.join(self.stderr)}")

    def status(self, expecting=0):
        status = self.wait()
        formatted = (
            f"{self.name}: returned {status} (expected {-expecting}) - {self.time:.2}s"
        )
        print(formatted)
        return formatted if status != -expecting else None

    def wait(self):
        try:
            code = self.process.wait(timeout=self.timeout)
        except TimeoutExpired:
            self._interrupt_group()
            try:
                code = self.process.wait(timeout=10)
            except TimeoutExpired:
                self._kill_group()
                code = self.process.wait(timeout=10)
        if self.end is None:
            self.end = time.time()
        return code

    def _interrupt_group(self):
        try:
            killpg(getpgid(self.process.pid), SIGINT)
        except ProcessLookupError:
            pass

    def _kill_group(self):
        try:
            killpg(getpgid(self.process.pid), SIGKILL)
        except ProcessLookupError:
            pass

    def _cleanup(self):
        if self.process.poll() is None:
            self._interrupt_group()
            try:
                self.process.wait(timeout=5)
            except TimeoutExpired:
                self._kill_group()
                self.process.wait(timeout=5)

    def interrupt(self):
        # Send SIGINT to the isolated process group, then reap it.
        self._interrupt_group()
        return self.status(SIGINT)

    def wait_for_output(self, text, timeout=10):
        """Wait until a child emits text, preserving consumed stdout lines."""
        deadline = time.monotonic() + timeout
        while True:
            if any(text in line for line in self._stdouts):
                return

            remaining = deadline - time.monotonic()
            if remaining <= 0:
                raise TimeoutError(f"{self.name} did not emit {text!r}")
            if not select([self.process.stdout], [], [], remaining)[0]:
                raise TimeoutError(f"{self.name} did not emit {text!r}")

            line = self.process.stdout.readline()
            if not line:
                raise RuntimeError(f"{self.name} exited before emitting {text!r}")
            self._stdouts.append(line.decode("utf8"))

    @property
    def stdout(self):
        self._stdouts.extend(
            line.decode("utf8") for line in self.process.stdout.readlines()
        )
        return self._stdouts

    @property
    def stderr(self):
        self._stderrs.extend(
            line.decode("utf8") for line in self.process.stderr.readlines()
        )
        return self._stderrs

    @property
    def time(self):
        return None if self.end is None else (self.end - self.start)


@pytest.fixture
def pyrun(request):
    endpoint = None

    def run(p, args=None, basedir=None, timeout=30, network=True):
        nonlocal endpoint
        if network and endpoint is None:
            config = isolated_config()
            config.insert_json5("mode", '"router"')
            router, endpoint = open_listener(config)
            request.addfinalizer(router.close)
        example = Pyrun(p, args, basedir, timeout, endpoint if network else None)
        request.addfinalizer(example._cleanup)
        return example

    return run


def test_z_bytes(pyrun):
    """Test z_bytes."""
    z_bytes = pyrun("z_bytes.py", network=False)
    if sys.version_info >= (3, 9):
        if error := z_bytes.status():
            z_bytes.dbg()
            z_bytes.errors.append(error)

    assert not z_bytes.errors


def test_z_info_z_scout(pyrun, tmp_path):
    config = discovery_config()
    with zenoh.open(config) as discovery_session:
        config_path = tmp_path / "scouting.json5"
        config_path.write_text(str(config))
        z_info = pyrun("z_info.py")
        z_scout = pyrun("z_scout.py", ["--config", str(config_path)], network=False)
        for example in (z_scout, z_info):
            if error := example.status():
                example.dbg()
                pytest.fail(error)
        output = "".join(z_scout.stdout)
        discovered = set()
        for line in output.splitlines():
            if not line.startswith("Hello"):
                continue
            hello = re.fullmatch(r"Hello \{ zid: ([0-9a-f]+), .* \}", line)
            assert hello is not None, output
            discovered.add(hello[1])
        expected = str(discovery_session.info.zid())
        assert discovered <= {expected}, output
        if expected not in discovered:
            pytest.skip("No Hello received over loopback multicast")


def test_z_get_z_queryable(pyrun):
    """Test z_get & z_queryable"""
    key = f"demo/example/query/{uuid.uuid4().hex}"
    z_queryable = pyrun("z_queryable.py", [f"-k={key}"])
    time.sleep(3)
    ## z_get: Able to get reply from queryable
    z_get = pyrun("z_get.py", [f"-s={key}"])
    if error := z_get.status():
        z_get.dbg()
        z_get.errors.append(error)

    z_queryable.interrupt()

    if not (f"Received ('{key}': 'Queryable from Python!')" in "".join(z_get.stdout)):
        z_get.dbg()
        z_queryable.dbg()
        z_get.errors.append("z_get didn't get a response from z_queryable")
    queryableout = "".join(z_queryable.stdout)
    if not (f"Received Query '{key}'" in queryableout):
        z_queryable.errors.append("z_queryable didn't catch query")
    if any(("z_queryable" in error) for error in z_queryable.errors):
        z_queryable.dbg()

    assert not z_get.errors
    assert not z_queryable.errors


def test_z_querier_z_queryable(pyrun):
    """Test z_querier & z_queryable"""
    key = f"demo/example/querier/{uuid.uuid4().hex}"
    z_queryable = pyrun("z_queryable.py", [f"-k={key}"])
    time.sleep(3)
    ## z_querier: Able to get reply from queryable
    z_querier = pyrun("z_querier.py", [f"-s={key}", "-p=value"])
    time.sleep(5)
    z_queryable.interrupt()
    z_querier.interrupt()

    if not (
        f"Received ('{key}': 'Queryable from Python!')" in "".join(z_querier.stdout)
    ):
        z_querier.dbg()
        z_queryable.dbg()
        z_querier.errors.append("z_querier didn't get a response from z_queryable")
    queryableout = "".join(z_queryable.stdout)
    if not (f"Received Query '{key}' with payload: '[   0] value'" in queryableout):
        z_queryable.errors.append("z_queryable didn't catch query [0]")
    elif not (f"Received Query '{key}' with payload: '[   2] value'" in queryableout):
        z_queryable.errors.append("z_queryable didn't catch query [2]")
    if any(("z_queryable" in error) for error in z_queryable.errors):
        z_queryable.dbg()

    assert not z_querier.errors
    assert not z_queryable.errors


def test_z_storage_z_sub(pyrun):
    """Test z_storage & z_sub."""
    key = f"demo/example/storage/{uuid.uuid4().hex}"
    pub_key = f"{key}/pub"
    z_storage = pyrun("z_storage.py", ["--key", f"{key}/**"])
    z_sub = pyrun("z_sub.py", ["--key", f"{key}/**"])
    time.sleep(3)
    ## z_put: Put one message (to storage & sub)
    z_put = pyrun("z_put.py", ["--key", key])
    time.sleep(1)
    ## z_pub: Put two messages (to storage & sub)
    pub = pyrun("z_pub.py", ["--key", pub_key, "--iter=2"])
    time.sleep(1)
    z_get = pyrun("z_get.py", [f"-s={key}"])
    if error := z_put.status():
        z_put.dbg()
        z_put.errors.append(error)

    if error := z_get.status():
        z_get.dbg()
        z_get.errors.append(error)

    if not (f"Received ('{key}': 'Put from Python!')" in "".join(z_get.stdout)):
        z_get.dbg()
        z_get.errors.append("z_get didn't get a response from z_storage about put")
    if any(("z_get" in error) for error in z_get.errors):
        z_get.dbg()
    time.sleep(1)

    z_delete = pyrun("z_delete.py", ["--key", key])
    if error := z_delete.status():
        z_delete.dbg()
        z_delete.errors.append(error)
    time.sleep(1)

    ## z_get: Unable to get put from storage
    z_get = pyrun("z_get.py", [f"-s={key}"])
    if error := z_get.status():
        z_get.dbg()
        z_get.errors.append(error)
    if f"Received ('{key}': 'Put from Python!')" in "".join(z_get.stdout):
        z_storage.dbg()
        z_get.errors.append(
            "z_get did get a response from z_storage about put after delete"
        )
    if any(("z_get" in error) for error in z_get.errors):
        z_get.dbg()
    time.sleep(1)

    ## z_sub: Should receive put, pub and delete
    if error := z_sub.interrupt():
        z_sub.dbg()
        z_sub.errors.append(error)
    subout = "".join(z_sub.stdout)
    if not (f"Received SampleKind.PUT ('{key}': 'Put from Python!')" in subout):
        z_sub.errors.append("z_sub didn't catch put")
    if not (
        f"Received SampleKind.PUT ('{pub_key}': '[   1] Pub from Python!')" in subout
    ):
        z_sub.errors.append("z_sub didn't catch second z_pub")
    if not (f"Received SampleKind.DELETE ('{key}': '')" in subout):
        z_sub.errors.append("z_sub didn't catch delete")
    if any(("z_sub" in error) for error in z_sub.errors):
        z_sub.dbg()

    ## z_storage: Should receive put, pub, delete, and query
    if error := z_storage.interrupt():
        z_storage.dbg()
        z_storage.errors.append(error)
    storageout = "".join(z_storage.stdout)
    if not (f"Received SampleKind.PUT ('{key}': 'Put from Python!')" in storageout):
        z_storage.errors.append("z_storage didn't catch put")
    if not (
        f"Received SampleKind.PUT ('{pub_key}': '[   1] Pub from Python!')"
        in storageout
    ):
        z_storage.errors.append("z_storage didn't catch second z_pub")
    if not (f"Received SampleKind.DELETE ('{key}': '')" in storageout):
        z_storage.errors.append("z_storage didn't catch delete")
    if not (f"Received Query '{key}'" in storageout):
        z_storage.errors.append("z_storage didn't catch query")
    if any(("z_storage" in error) for error in z_storage.errors):
        z_storage.dbg()

    assert not z_sub.errors
    assert not z_storage.errors
    assert not z_get.errors


def test_z_pull_z_sub_queued(pyrun):
    """Test z_pull & z_sub_queued."""
    ## Run z_pull and z_sub_queued
    key = f"demo/example/pull/{uuid.uuid4().hex}"
    sub_queued = pyrun("z_sub_queued.py", ["--key", key])
    sub_queued.wait_for_output("Press CTRL-C to quit...")
    # Wait for the pull subscriber to finish declaring before publishing.
    # The first poll must happen after the publisher has completed both puts.
    # Use a unique key as well, so unrelated samples cannot overwrite the ring.
    pull = pyrun("z_pull.py", ["--key", key, "--size=1", "--interval=5"])
    pull.wait_for_output("Press CTRL-C to quit...")
    ## z_pub: Put two messages (to storage & sub)
    # z_pub waits before its first put, giving the publisher time to match.
    pub = pyrun("z_pub.py", ["--key", key, "--iter=2", "--interval=1"])
    if error := pub.status():
        pub.dbg()
        pub.errors.append(error)
    ## z_sub_queued: Should receive two messages
    if error := sub_queued.interrupt():
        sub_queued.dbg()
        sub_queued.errors.append(error)
    sub_queued_out = "".join(sub_queued.stdout)
    if not (
        f"Received SampleKind.PUT ('{key}': '[   0] Pub from Python!')"
        in sub_queued_out
    ):
        sub_queued.errors.append("z_sub_queued didn't catch the first z_pub")
    if not (
        f"Received SampleKind.PUT ('{key}': '[   1] Pub from Python!')"
        in sub_queued_out
    ):
        sub_queued.errors.append("z_sub_queued didn't catch the second z_pub")
    if any(("z_sub_queued" in error) for error in sub_queued.errors):
        sub_queued.dbg()
    ## z_pull: Should only receive the last message
    # z_pull polls every 5 seconds; the publisher has completed before the
    # first poll.
    time.sleep(3)
    if error := pull.interrupt():
        pull.dbg()
        pull.errors.append(error)
    pullout = "".join(pull.stdout)
    if f"Received SampleKind.PUT ('{key}': '[   0] Pub from Python!')" in pullout:
        pull.errors.append("z_pull shouldn't catch the old z_pub")
    if not (f"Received SampleKind.PUT ('{key}': '[   1] Pub from Python!')" in pullout):
        pull.errors.append("z_pull didn't catch the last z_pub")
    if any(("z_pull" in error) for error in pull.errors):
        pull.dbg()

    assert not pub.errors
    assert not sub_queued.errors
    assert not pull.errors


def test_z_sub_thr_z_pub_thr(pyrun):
    """Test z_sub_thr & z_pub_thr."""
    key = f"test/thr/{uuid.uuid4().hex}"
    sub_thr = pyrun("z_sub_thr.py", ["--key", key])
    pub_thr = pyrun("z_pub_thr.py", ["--key", key, "128"])
    time.sleep(5)
    # Stop the publisher first.  Leaving a high-rate publisher running while
    # the subscriber callback handler is being torn down can keep the callback
    # thread busy and make shutdown timing-dependent.
    if error := pub_thr.interrupt():
        pub_thr.dbg()
        pub_thr.errors.append(error)
    if error := sub_thr.interrupt():
        sub_thr.dbg()
        sub_thr.errors.append(error)

    assert not sub_thr.errors
    assert not pub_thr.errors


def test_z_advanced_pub_z_advanced_sub(pyrun):
    """Test that z_advanced_sub replays the publisher's cached history."""
    key = f"demo/example/advanced/{uuid.uuid4().hex}"
    pub = pyrun("z_advanced_pub.py", ["--key", key, "--history=32"])
    # The publisher has cached samples 0–3 before it logs that it is sending 4.
    pub.wait_for_output(f"Putting Data ('{key}': '[   4] Pub from Python!')")
    sub = pyrun("z_advanced_sub.py", ["--key", key])
    sub.wait_for_output(f"Received SampleKind.PUT ('{key}': '[   7] Pub from Python!')")

    if error := pub.interrupt():
        pub.dbg()
        pub.errors.append(error)
    if error := sub.interrupt():
        sub.dbg()
        sub.errors.append(error)

    sub_out = "".join(sub.stdout)
    for i in range(8):
        if (
            f"Received SampleKind.PUT ('{key}': '[{i:4d}] Pub from Python!')"
            not in sub_out
        ):
            sub.errors.append(
                f"z_advanced_sub didn't catch the {i}-th z_advanced_pub message"
            )

    assert not pub.errors
    assert not sub.errors, sub_out


def test_z_pub_shm(pyrun):
    """Test z_pub_shm."""
    ## Run z_sub
    key = f"demo/example/shm/{uuid.uuid4().hex}"
    sub = pyrun("z_sub.py", ["--key", f"{key}/**"])
    time.sleep(3)
    ## z_pub: Put two messages (to storage & sub)
    pub = pyrun("z_pub.py", ["--key", key, "--iter=1", "--interval=0"])
    if error := pub.status():
        pub.dbg()
        pub.errors.append(error)
    ## z_sub_queued: Should receive two messages
    if error := sub.interrupt():
        sub.dbg()
        sub.errors.append(error)
    sub_out = "".join(sub.stdout)
    if not (f"Received SampleKind.PUT ('{key}': '[   0] Pub from Python!')" in sub_out):
        sub.errors.append("z_sub_queued didn't catch the first z_pub")

    assert not pub.errors
    assert not sub.errors


@pytest.mark.parametrize(
    "example_file",
    sorted(
        p
        for p in glob(path.join(docs_examples, "*.py"))
        if not path.basename(p).startswith("_")
    ),
    ids=path.basename,
)
def test_docs_examples(pyrun, example_file):
    """Run each documentation example in its own isolated network."""
    example_name = path.basename(example_file)
    example = pyrun(example_name, basedir=docs_examples, network=False)
    code = example.wait()
    if code == 77 and example_name == "scouting.py":
        pytest.skip("No Hello received over loopback multicast")
    if error := example.status():
        example.dbg()
        pytest.fail(error)
