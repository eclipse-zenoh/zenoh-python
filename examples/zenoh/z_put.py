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

import json
import sys
import time
import argparse
import zenoh
from zenoh import Zenoh, Value

# --- Command line argument parsing --- --- --- --- --- ---
parser = argparse.ArgumentParser(
    prog='z_put',
    description='zenoh put example')
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
                    default='/demo/example/zenoh-python-put',
                    type=str,
                    help='The name of the resource to put.')
parser.add_argument('--value', '-v', dest='value',
                    default='Put from Python!',
                    type=str,
                    help='The value of the resource to put.')

args = parser.parse_args()
conf = { "mode": args.mode }
if args.peer is not None:
    conf["peer"] = ",".join(args.peer)
if args.listener is not None:
    conf["listener"] = ",".join(args.listener)
path = args.path
value = args.value

# --- zenoh-net code --- --- --- --- --- --- --- --- --- --- ---

# initiate logging
zenoh.init_logger()

print("Openning session...")
z = Zenoh(conf)

print("New workspace...")
workspace = z.workspace()

print("Put Data ('{}': '{}')...".format(path, value))
workspace.put(path, value)


# --- Examples of put with other types:

# - Integer
# workspace.put('/demo/example/Integer', 3)

# - Float
# workspace.put('/demo/example/Float', 3.14)

# - Properties (as a Dictionary with str only)
# workspace.put('/demo/example/Properties', {'p1': 'v1', 'p2': 'v2'})

# - Json (str format)
# workspace.put('/demo/example/Json',
#               Value.Json(json.dumps(['foo', {'bar': ('baz', None, 1.0, 2)}])))

# - Raw ('application/octet-stream' encoding by default)
# workspace.put('/demo/example/Raw', b'\x48\x69\x33'))

# - Custom
# workspace.put('/demo/example/Custom',
#               Value.Custom('my_encoding', b'\x48\x69\x33'))

z.close()
