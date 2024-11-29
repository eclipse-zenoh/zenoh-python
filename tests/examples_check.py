# Copyright (c) 2017, 2022 ZettaScale Technology Inc.
import sys
import time
from os import getpgid, killpg, path
from signal import SIGINT
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
tab = "\t"
ret = "\r\n"


class Pyrun(fixtures.Fixture):
    def __init__(self, p, args=None) -> None:
        if args is None:
            args = []
        self.name = p
        print(f"starting {self.name}")
        self.process: Popen = Popen(
            ["python3", path.join(examples, p), *args],
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
        self.addCleanup(self.process.send_signal, SIGINT)

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
            code = self.process.wait(timeout=10)
        except TimeoutExpired:
            self.process.send_signal(SIGINT)
            code = self.process.wait(timeout=10)
        if self.end is None:
            self.end = time.time()
        return code

    def interrupt(self):
        # send SIGINT to process group
        pgid = getpgid(self.process.pid)
        killpg(pgid, SIGINT)
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
    z_queryable = Pyrun("z_queryable.py", ["-k=demo/example/zenoh-python-queryable"])
    time.sleep(3)
    ## z_get: Able to get reply from queryable
    z_get = Pyrun("z_get.py", ["-s=demo/example/zenoh-python-queryable"])
    if error := z_get.status():
        z_get.dbg()
        z_get.errors.append(error)

    z_queryable.interrupt()

    if not (
        "Received ('demo/example/zenoh-python-queryable': 'Queryable from Python!')"
        in "".join(z_get.stdout)
    ):
        z_get.dbg()
        z_queryable.dbg()
        z_get.errors.append("z_get didn't get a response from z_queryable")
    queryableout = "".join(z_queryable.stdout)
    if not ("Received Query 'demo/example/zenoh-python-queryable'" in queryableout):
        z_queryable.errors.append("z_queryable didn't catch query")
    if any(("z_queryable" in error) for error in z_queryable.errors):
        z_queryable.dbg()

    assert not z_get.errors
    assert not z_queryable.errors


def test_z_querier_z_queryable():
    """Test z_querier & z_queryable"""
    z_queryable = Pyrun("z_queryable.py", ["-k=demo/example/zenoh-python-queryable"])
    time.sleep(3)
    ## z_querier: Able to get reply from queryable
    z_querier = Pyrun(
        "z_querier.py", ["-s=demo/example/zenoh-python-queryable", "-p=value"]
    )
    time.sleep(5)
    z_queryable.interrupt()
    z_querier.interrupt()

    if not (
        "Received ('demo/example/zenoh-python-queryable': 'Queryable from Python!')"
        in "".join(z_querier.stdout)
    ):
        z_querier.dbg()
        z_queryable.dbg()
        z_querier.errors.append("z_querier didn't get a response from z_queryable")
    queryableout = "".join(z_queryable.stdout)
    if not (
        "Received Query 'demo/example/zenoh-python-queryable' with payload: [   0] value"
        in queryableout
    ):
        z_queryable.errors.append("z_queryable didn't catch query [0]")
    elif not (
        "Received Query 'demo/example/zenoh-python-queryable' with payload: [   2] value"
        in queryableout
    ):
        z_queryable.errors.append("z_queryable didn't catch query [2]")
    if any(("z_queryable" in error) for error in z_queryable.errors):
        z_queryable.dbg()

    assert not z_querier.errors
    assert not z_queryable.errors


def test_z_storage_z_sub():
    """Test z_storage & z_sub."""
    z_storage = Pyrun("z_storage.py")
    z_sub = Pyrun("z_sub.py")
    time.sleep(3)
    ## z_put: Put one message (to storage & sub)
    z_put = Pyrun("z_put.py")
    time.sleep(1)
    ## z_pub: Put two messages (to storage & sub)
    pub = Pyrun("z_pub.py", ["--iter=2"])
    time.sleep(1)
    z_get = Pyrun("z_get.py", ["-s=demo/example/zenoh-python-put"])
    if error := z_put.status():
        z_put.dbg()
        z_put.errors.append(error)

    if error := z_get.status():
        z_get.dbg()
        z_get.errors.append(error)

    if not (
        "Received ('demo/example/zenoh-python-put': 'Put from Python!')"
        in "".join(z_get.stdout)
    ):
        z_get.dbg()
        z_get.errors.append("z_get didn't get a response from z_storage about put")
    if any(("z_get" in error) for error in z_get.errors):
        z_get.dbg()
    time.sleep(1)

    z_delete = Pyrun("z_delete.py")
    if error := z_delete.status():
        z_delete.dbg()
        z_delete.errors.append(error)
    time.sleep(1)

    ## z_get: Unable to get put from storage
    z_get = Pyrun("z_get.py", ["-s=demo/example/zenoh-python-put"])
    if error := z_get.status():
        z_get.dbg()
        z_get.errors.append(error)
    if "Received ('demo/example/zenoh-python-put': 'Put from Python!')" in "".join(
        z_get.stdout
    ):
        z_storage.dbg()
        z_get.errors.append(
            "z_get did get a response from z_storage about put after delete"
        )
    if any(("z_get" in error) for error in z_get.errors):
        z_get.dbg()
    time.sleep(1)

    ## z_sub: Should receive put, pub and delete
    if error := z_sub.process.send_signal(SIGINT):
        z_sub.dbg()
        z_sub.errors.append(error)
    subout = "".join(z_sub.stdout)
    if not (
        "Received SampleKind.PUT ('demo/example/zenoh-python-put': 'Put from Python!')"
        in subout
    ):
        z_sub.errors.append("z_sub didn't catch put")
    if not (
        "Received SampleKind.PUT ('demo/example/zenoh-python-pub': '[   1] Pub from Python!')"
        in subout
    ):
        z_sub.errors.append("z_sub didn't catch second z_pub")
    if not (
        "Received SampleKind.DELETE ('demo/example/zenoh-python-put': '')" in subout
    ):
        z_sub.errors.append("z_sub didn't catch delete")
    if any(("z_sub" in error) for error in z_sub.errors):
        z_sub.dbg()

    ## z_storage: Should receive put, pub, delete, and query
    if error := z_storage.process.send_signal(SIGINT):
        z_storage.dbg()
        z_storage.errors.append(error)
    storageout = "".join(z_storage.stdout)
    if not (
        "Received SampleKind.PUT ('demo/example/zenoh-python-put': 'Put from Python!')"
        in storageout
    ):
        z_storage.errors.append("z_storage didn't catch put")
    if not (
        "Received SampleKind.PUT ('demo/example/zenoh-python-pub': '[   1] Pub from Python!')"
        in storageout
    ):
        z_storage.errors.append("z_storage didn't catch second z_pub")
    if not (
        "Received SampleKind.DELETE ('demo/example/zenoh-python-put': '')" in storageout
    ):
        z_storage.errors.append("z_storage didn't catch delete")
    if not ("Received Query 'demo/example/zenoh-python-put'" in storageout):
        z_storage.errors.append("z_storage didn't catch query")
    if any(("z_storage" in error) for error in z_storage.errors):
        z_storage.dbg()

    assert not z_sub.errors
    assert not z_storage.errors
    assert not z_get.errors


def test_z_pull_z_sub_queued():
    """Test z_pull & z_sub_queued."""
    ## Run z_pull and z_sub_queued
    sub_queued = Pyrun("z_sub_queued.py")
    time.sleep(3)
    pull = Pyrun("z_pull.py", ["--size=1", "--interval=1"])
    time.sleep(3)
    ## z_pub: Put two messages (to storage & sub)
    pub = Pyrun("z_pub.py", ["--iter=2", "--interval=0"])
    if error := pub.status():
        pub.dbg()
        pub.errors.append(error)
    ## z_sub_queued: Should receive two messages
    if error := sub_queued.interrupt():
        sub_queued.dbg()
        sub_queued.errors.append(error)
    sub_queued_out = "".join(sub_queued.stdout)
    if not (
        "Received SampleKind.PUT ('demo/example/zenoh-python-pub': '[   0] Pub from Python!')"
        in sub_queued_out
    ):
        sub_queued.errors.append("z_sub_queued didn't catch the first z_pub")
    if not (
        "Received SampleKind.PUT ('demo/example/zenoh-python-pub': '[   1] Pub from Python!')"
        in sub_queued_out
    ):
        sub_queued.errors.append("z_sub_queued didn't catch the second z_pub")
    if any(("z_sub_queued" in error) for error in sub_queued.errors):
        sub_queued.dbg()
    ## z_pull: Should only receive the last messages
    time.sleep(3)
    if error := pull.interrupt():
        pull.dbg()
        pull.errors.append(error)
    pullout = "".join(pull.stdout)
    if (
        "Received SampleKind.PUT ('demo/example/zenoh-python-pub': '[   0] Pub from Python!')"
        in pullout
    ):
        pull.errors.append("z_pull shouldn't catch the old z_pub")
    if not (
        "Received SampleKind.PUT ('demo/example/zenoh-python-pub': '[   1] Pub from Python!')"
        in pullout
    ):
        pull.errors.append("z_pull didn't catch the last z_pub")
    if any(("z_pull" in error) for error in pull.errors):
        pull.dbg()

    assert not pub.errors
    assert not sub_queued.errors
    assert not pull.errors


def test_z_sub_thr_z_pub_thr():
    """Test z_sub_thr & z_pub_thr."""
    sub_thr = Pyrun("z_sub_thr.py")
    pub_thr = Pyrun("z_pub_thr.py", ["128"])
    time.sleep(5)
    if error := sub_thr.interrupt():
        sub_thr.dbg()
        sub_thr.errors.append(error)
    if error := pub_thr.interrupt():
        pub_thr.dbg()
        pub_thr.errors.append(error)

    assert not sub_thr.errors
    assert not pub_thr.errors
