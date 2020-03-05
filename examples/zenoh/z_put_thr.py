# Copyright (c) 2017, 2020 ADLINK Technology Inc.
#
# This program and the accompanying materials are made available under the
# terms of the Eclipse Public License 2.0 which is available at
# http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
# which is available at https://www.apache.org/licenses/LICENSE-2.0.
#
# SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
#
# Contributors:
#   ADLINK zenoh team, <zenoh@adlink-labs.tech>

import time
import sys
from zenoh import Zenoh, Selector, Path, Workspace, Encoding, Value
import argparse

locator = None
if len(sys.argv) < 2:
    print('USAGE:\n\tzn_put_thr <payload-size> [<zenoh-locator>]\n\n')
    sys.exit(-1)

length = int(sys.argv[1])
print("Running throughput test for payload of {} bytes".format(length))
if len(sys.argv) > 2:
    locator = sys.argv[2]

path = '/test/thr'

data = bytearray()
for i in range(0, length):
    data.append(i % 10)
v = Value(data, encoding=Encoding.RAW)

print('Login to Zenoh...')
z = Zenoh.login(locator)
print("Use Workspace on '/'")
w = z.workspace('/')

while True:
    w.put(path, v)
