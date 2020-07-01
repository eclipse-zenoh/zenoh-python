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
import argparse
from zenoh import Zenoh, Selector, Path, Workspace, Encoding, Value

# --- Command line argument parsing --- --- --- --- --- ---
parser = argparse.ArgumentParser(
    prog='z_put_thr', description='Publisher for zenoh throughput example')
parser.add_argument(
    '--size', '-s', dest='size',
    default='256', type=int,
    help='the size in bytes of the payload used for the throughput test')

parser.add_argument(
    '--locator', '-l', dest='locator',
    default=None, type=str,
    help='The locator to be used to boostrap the zenoh session.'
         ' By default dynamic discovery is used')

parser.add_argument(
    '--path', '-p', dest='path',
    default='/zenoh/examples/throughput/data',
    type=str,
    help='the resource used to write throughput data')

args = parser.parse_args()

locator = args.locator
size = args.size
path = args.path

# zenoh code  --- --- --- --- --- --- --- --- --- --- ---
print("Running throughput test for payload of {} bytes".format(size))
data = bytearray()
for i in range(0, size):
    data.append(i % 10)

v = Value(data)

print('Login to Zenoh...')
z = Zenoh.login(locator)
w = z.workspace()

print('Press Ctrl-C to stop the publisher...')
while True:
    w.put(path, v)
