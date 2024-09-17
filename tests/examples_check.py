# Copyright (c) 2017, 2022 ZettaScale Technology Inc.
import sys
import time
from os import getpgid, killpg, path
from signal import SIGINT
from subprocess import PIPE, Popen

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


class Pyrun:
    def __init__(self, p, args=None) -> None:
        if args is None:
            args = []
        self.name = p
        print(f"starting {self.name}")
        self.process: Popen = Popen(
            ["python3", path.join(examples, p), *args],
            stdin=PIPE,
            stdout=PIPE,
            stderr=PIPE,
            start_new_session=True,
        )
        self.start = time.time()
        self.end = None
        self._stdouts = []
        self._stderrs = []

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
        except:
            self.process.kill()
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


errors = []

if sys.version_info >= (3, 9):
    # Test z_bytes
    print("=> Test z_bytes")
    z_bytes = Pyrun("z_bytes.py")
    if error := z_bytes.status():
        z_bytes.dbg()
        errors.append(error)

# Test z_info & z_scout
print("=> Test z_info & z_scout")
info = Pyrun("z_info.py")
if error := info.status():
    info.dbg()
    errors.append(error)
scout = Pyrun("z_scout.py")
if error := scout.status():
    scout.dbg()
    errors.append(error)

# Test z_get & z_queryable
print("=> Test z_get & z_queryable")
## Run z_queryable
queryable = Pyrun("z_queryable.py", ["-k=demo/example/zenoh-python-queryable"])
time.sleep(1)
## z_get: Able to get reply from queryable
get = Pyrun("z_get.py", ["-s=demo/example/zenoh-python-queryable"])
if error := get.status():
    get.dbg()
    errors.append(error)
if not (
    "Received ('demo/example/zenoh-python-queryable': 'Queryable from Python!')"
    in "".join(get.stdout)
):
    get.dbg()
    queryable.dbg()
    errors.append("z_get didn't get a response from z_queryable")
## Stop z_queryable
if error := queryable.interrupt():
    queryable.dbg()
    errors.append(error)
queryableout = "".join(queryable.stdout)
if not ("Received Query 'demo/example/zenoh-python-queryable'" in queryableout):
    errors.append("z_queryable didn't catch query")
if any(("z_queryable" in error) for error in errors):
    queryable.dbg()

# Test z_storage & z_sub
print("=> Test z_storage & z_sub")
storage = Pyrun("z_storage.py")
sub = Pyrun("z_sub.py")
time.sleep(1)
## z_put: Put one message (to storage & sub)
put = Pyrun("z_put.py")
if error := put.status():
    put.dbg()
    errors.append(error)
time.sleep(1)
## z_pub: Put two messages (to storage & sub)
pub = Pyrun("z_pub.py", ["--iter=2"])
time.sleep(4)
## z_get: Able to get put from storage
get = Pyrun("z_get.py", ["-s=demo/example/zenoh-python-put"])
if error := get.status():
    get.dbg()
    errors.append(error)
if not (
    "Received ('demo/example/zenoh-python-put': 'Put from Python!')"
    in "".join(get.stdout)
):
    get.dbg()
    errors.append("z_get didn't get a response from z_storage about put")
if any(("z_get" in error) for error in errors):
    get.dbg()
time.sleep(1)
## z_delete: Delete put in storage
delete = Pyrun("z_delete.py")
if error := delete.status():
    delete.dbg()
    errors.append(error)
time.sleep(1)
## z_get: Unable to get put from storage
get = Pyrun("z_get.py", ["-s=demo/example/zenoh-python-put"])
if error := get.status():
    get.dbg()
    errors.append(error)
if "Received ('demo/example/zenoh-python-put': 'Put from Python!')" in "".join(
    get.stdout
):
    # storage.dbg()
    errors.append("z_get did get a response from z_storage about put after delete")
if any(("z_get" in error) for error in errors):
    get.dbg()
time.sleep(1)
## z_sub: Should receive put, pub and delete
if error := sub.interrupt():
    sub.dbg()
    errors.append(error)
subout = "".join(sub.stdout)
if not (
    "Received SampleKind.PUT ('demo/example/zenoh-python-put': 'Put from Python!')"
    in subout
):
    errors.append("z_sub didn't catch put")
if not (
    "Received SampleKind.PUT ('demo/example/zenoh-python-pub': '[   1] Pub from Python!')"
    in subout
):
    errors.append("z_sub didn't catch second z_pub")
if not ("Received SampleKind.DELETE ('demo/example/zenoh-python-put': '')" in subout):
    errors.append("z_sub didn't catch delete")
if any(("z_sub" in error) for error in errors):
    sub.dbg()
## z_storage: Should receive put, pub, delete, and query
if error := storage.interrupt():
    storage.dbg()
    errors.append(error)
storageout = "".join(storage.stdout)
if not (
    "Received SampleKind.PUT ('demo/example/zenoh-python-put': 'Put from Python!')"
    in storageout
):
    errors.append("z_storage didn't catch put")
if not (
    "Received SampleKind.PUT ('demo/example/zenoh-python-pub': '[   1] Pub from Python!')"
    in storageout
):
    errors.append("z_storage didn't catch second z_pub")
if not (
    "Received SampleKind.DELETE ('demo/example/zenoh-python-put': '')" in storageout
):
    errors.append("z_storage didn't catch delete")
if not ("Received Query 'demo/example/zenoh-python-put'" in storageout):
    errors.append("z_storage didn't catch query")
if any(("z_storage" in error) for error in errors):
    storage.dbg()

# Test z_pull & s_sub_queued
print("=> Test z_pull & z_sub_queued")
## Run z_pull and z_sub_queued
sub_queued = Pyrun("z_sub_queued.py")
time.sleep(1)
pull = Pyrun("z_pull.py", ["--size=1", "--interval=1"])
time.sleep(1)
## z_pub: Put two messages (to storage & sub)
pub = Pyrun("z_pub.py", ["--iter=2", "--interval=0"])
if error := pub.status():
    pub.dbg()
    errors.append(error)
## z_sub_queued: Should receive two messages
if error := sub_queued.interrupt():
    sub_queued.dbg()
    errors.append(error)
sub_queued_out = "".join(sub_queued.stdout)
if not (
    "Received SampleKind.PUT ('demo/example/zenoh-python-pub': '[   0] Pub from Python!')"
    in sub_queued_out
):
    errors.append("z_sub_queued didn't catch the first z_pub")
if not (
    "Received SampleKind.PUT ('demo/example/zenoh-python-pub': '[   1] Pub from Python!')"
    in sub_queued_out
):
    errors.append("z_sub_queued didn't catch the second z_pub")
if any(("z_sub_queued" in error) for error in errors):
    sub_queued.dbg()
## z_pull: Should only receive the last messages
time.sleep(1)
if error := pull.interrupt():
    pull.dbg()
    errors.append(error)
pullout = "".join(pull.stdout)
if (
    "Received SampleKind.PUT ('demo/example/zenoh-python-pub': '[   0] Pub from Python!')"
    in pullout
):
    errors.append("z_pull shouldn't catch the old z_pub")
if not (
    "Received SampleKind.PUT ('demo/example/zenoh-python-pub': '[   1] Pub from Python!')"
    in pullout
):
    errors.append("z_pull didn't catch the last z_pub")
if any(("z_pull" in error) for error in errors):
    pull.dbg()

# Test z_sub_thr & z_pub_thr
print("=> Test z_sub_thr & z_pub_thr")
sub_thr = Pyrun("z_sub_thr.py")
pub_thr = Pyrun("z_pub_thr.py", ["128"])
time.sleep(5)
if error := sub_thr.interrupt():
    sub_thr.dbg()
    errors.append(error)
if error := pub_thr.interrupt():
    pub_thr.dbg()
    errors.append(error)


if len(errors):
    message = f"Found {len(errors)} errors: {(ret+tab) + (ret+tab).join(errors)}"
    raise Exception(message)
else:
    print("Pass examples_check")
