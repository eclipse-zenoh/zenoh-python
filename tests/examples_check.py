# Copyright (c) 2017, 2022 ZettaScale Technology Inc.
import sys
import time
import uuid
from glob import glob
from os import getpgid, killpg, path
from signal import SIGINT, SIGKILL
from subprocess import PIPE, Popen, TimeoutExpired

import fixtures

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
    def __init__(self, p, args=None, basedir=None, timeout=30) -> None:
        if args is None:
            args = []
        if basedir is None:
            basedir = examples
        self.name = p
        self.timeout = timeout
        print(f"starting {self.name}")
        self.process: Popen = Popen(
            ["python3", path.join(basedir, p), *args],
            stdout=PIPE,
            stderr=PIPE,
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


def test_z_bytes():
    """Test z_bytes."""
    z_bytes = Pyrun("z_bytes.py")
    if sys.version_info >= (3, 9):
        if error := z_bytes.status():
            z_bytes.dbg()
            z_bytes.errors.append(error)

    assert not z_bytes.errors


def test_z_info_z_scout():
    z_info = Pyrun("z_info.py")
    z_scout = Pyrun("z_scout.py")
    if error := z_info.status():
        z_info.dbg()
        z_info.errors.append(error)

    if error := z_scout.status():
        z_scout.dbg()
        z_scout.errors.append(error)

    assert not z_info.errors
    assert not z_scout.errors


def test_z_get_z_queryable():
    """Test z_get & z_queryable"""
    key = f"demo/example/query/{uuid.uuid4().hex}"
    z_queryable = Pyrun("z_queryable.py", [f"-k={key}"])
    time.sleep(3)
    ## z_get: Able to get reply from queryable
    z_get = Pyrun("z_get.py", [f"-s={key}"])
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


def test_z_querier_z_queryable():
    """Test z_querier & z_queryable"""
    key = f"demo/example/querier/{uuid.uuid4().hex}"
    z_queryable = Pyrun("z_queryable.py", [f"-k={key}"])
    time.sleep(3)
    ## z_querier: Able to get reply from queryable
    z_querier = Pyrun("z_querier.py", [f"-s={key}", "-p=value"])
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


def test_z_storage_z_sub():
    """Test z_storage & z_sub."""
    key = f"demo/example/storage/{uuid.uuid4().hex}"
    pub_key = f"{key}/pub"
    z_storage = Pyrun("z_storage.py", ["--key", f"{key}/**"])
    z_sub = Pyrun("z_sub.py", ["--key", f"{key}/**"])
    time.sleep(3)
    ## z_put: Put one message (to storage & sub)
    z_put = Pyrun("z_put.py", ["--key", key])
    time.sleep(1)
    ## z_pub: Put two messages (to storage & sub)
    pub = Pyrun("z_pub.py", ["--key", pub_key, "--iter=2"])
    time.sleep(1)
    z_get = Pyrun("z_get.py", [f"-s={key}"])
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

    z_delete = Pyrun("z_delete.py", ["--key", key])
    if error := z_delete.status():
        z_delete.dbg()
        z_delete.errors.append(error)
    time.sleep(1)

    ## z_get: Unable to get put from storage
    z_get = Pyrun("z_get.py", [f"-s={key}"])
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


def test_z_pull_z_sub_queued():
    """Test z_pull & z_sub_queued."""
    ## Run z_pull and z_sub_queued
    key = f"demo/example/pull/{uuid.uuid4().hex}"
    sub_queued = Pyrun("z_sub_queued.py", ["--key", key])
    time.sleep(3)
    # The first poll must happen after the publisher has completed both puts.
    # Use a unique key as well, so unrelated samples cannot overwrite the ring.
    pull = Pyrun("z_pull.py", ["--key", key, "--size=1", "--interval=5"])
    time.sleep(3)
    ## z_pub: Put two messages (to storage & sub)
    pub = Pyrun("z_pub.py", ["--key", key, "--iter=2", "--interval=0"])
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


def test_z_sub_thr_z_pub_thr():
    """Test z_sub_thr & z_pub_thr."""
    key = f"test/thr/{uuid.uuid4().hex}"
    sub_thr = Pyrun("z_sub_thr.py", ["--key", key])
    pub_thr = Pyrun("z_pub_thr.py", ["--key", key, "128"])
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


def test_z_advanced_pub_z_advanced_sub():
    """Test z_advanced_pub & z_advanced_sub."""
    ## Run z_advanced_pub and z_advanced_sub
    key = f"demo/example/advanced/{uuid.uuid4().hex}"
    ## z_advanced_pub: Start publishing messages
    pub = Pyrun("z_advanced_pub.py", ["--key", key, "--history=10"])
    time.sleep(5)  # wait 5 seconds to ensure that we miss few messages
    sub = Pyrun("z_advanced_sub.py", ["--key", f"{key}/**"])
    time.sleep(5)

    if error := pub.interrupt():
        pub.dbg()
        pub.errors.append(error)
    if error := sub.interrupt():
        sub.dbg()
        sub.errors.append(error)

    sub_out = "".join(sub.stdout)
    for i in range(0, 8):
        if not (
            f"Received SampleKind.PUT ('{key}': '[   {i}] Pub from Python!')" in sub_out
        ):
            sub.errors.append(
                f"z_advanced_sub didn't catch the {i}-th z_advanced_pub message"
            )

    assert not pub.errors
    assert not sub.errors


def test_z_pub_shm():
    """Test z_pub_shm."""
    ## Run z_sub
    key = f"demo/example/shm/{uuid.uuid4().hex}"
    sub = Pyrun("z_sub.py", ["--key", f"{key}/**"])
    time.sleep(3)
    ## z_pub: Put two messages (to storage & sub)
    pub = Pyrun("z_pub.py", ["--key", key, "--iter=1", "--interval=0"])
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


def test_docs_examples():
    """Test all docs/examples - run each one and verify no timeout or non-zero exit."""
    example_files = glob(path.join(docs_examples, "*.py"))
    errors = []

    for example_file in example_files:
        example_name = path.basename(example_file)
        print(f"\nTesting docs example: {example_name}")

        example = Pyrun(example_name, basedir=docs_examples)
        if error := example.status():
            example.dbg()
            errors.append(f"{example_name}: {error}")

    assert not errors, f"Docs examples failed: {errors}"
