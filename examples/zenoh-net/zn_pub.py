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
import itertools
import zenoh
from zenoh.net import config

# --- Command line argument parsing --- --- --- --- --- ---
parser = argparse.ArgumentParser(
    prog='zn_pub',
    description='zenoh-net pub example')
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
parser.add_argument('--path', '-p', dest='path',
                    default='/demo/example/zenoh-python-pub',
                    type=str,
                    help='The name of the resource to publish.')
parser.add_argument('--value', '-v', dest='value',
                    default='Pub from Python!',
                    type=str,
                    help='The value of the resource to publish.')

args = parser.parse_args()
conf = { "mode": args.mode }
if args.peer is not None:
    conf["peer"] = ",".join(args.peer)
if args.listener is not None:
    conf["listener"] = ",".join(args.listener)
path = args.path
value = args.value

# zenoh-net code  --- --- --- --- --- --- --- --- --- --- ---

# initiate logging
zenoh.init_logger()

print("Openning session...")
session = zenoh.net.open(conf)

print("Declaring Resource " + path)
rid = session.declare_resource(path)
print(" => RId {}".format(rid))

print("Declaring Publisher on {}".format(rid))
publisher = session.declare_publisher(rid)

for idx in itertools.count():
    time.sleep(1)
    buf = "[{:4d}] {}".format(idx, value)
    print("Writing Data ('{}': '{}')...".format(rid, buf))
    session.write(rid, bytes(buf, encoding='utf8'))

publisher.undeclare()
session.close()
