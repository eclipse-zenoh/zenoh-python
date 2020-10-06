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

import sys
import time
import argparse
import zenoh
from zenoh import Zenoh, Value
from zenoh.net import encoding

# --- Command line argument parsing --- --- --- --- --- ---
parser = argparse.ArgumentParser(
    prog='z_sub',
    description='zenoh sub example')
parser.add_argument('size',
                    metavar='PAYLOAD_SIZE',
                    type=int,
                    help='Sets the size of the payload to put.')
parser.add_argument('--mode', '-m', dest='mode',
                    default='peer',
                    choices=['peer', 'client'],
                    type=str,
                    help='The zenoh session mode.')
parser.add_argument('--peer', '-e', dest='peer',
                    metavar='LOCATOR',
                    action='append',
                    type=str,
                    help='Peer locators used to initiate the zenoh session.')
parser.add_argument('--listener', '-l', dest='listener',
                    metavar='LOCATOR',
                    action='append',
                    type=str,
                    help='Locators to listen on.')

args = parser.parse_args()
conf = { "mode": args.mode }
if args.peer is not None:
    conf["peer"] = ",".join(args.peer)
if args.listener is not None:
    conf["listener"] = ",".join(args.listener)
print(type(args.size))
size = args.size

# zenoh code  --- --- --- --- --- --- --- --- --- --- ---
print("Running throughput test for payload of {} bytes".format(size))
data = bytearray()
for i in range(0, size):
    data.append(i % 10)

v = Value.Raw(encoding.NONE, bytes(data))

print("New zenoh...")
zenoh = Zenoh(conf)

print("New workspace...")
workspace = zenoh.workspace()


print('Press Ctrl-C to stop the publisher...')
while True:
    workspace.put('/test/thr', v)
