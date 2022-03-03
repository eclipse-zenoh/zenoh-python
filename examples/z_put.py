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
import json
import zenoh
from zenoh import config, Encoding, Value

# --- Command line argument parsing --- --- --- --- --- ---
parser = argparse.ArgumentParser(
    prog='z_put',
    description='zenoh put example')
parser.add_argument('--mode', '-m', dest='mode',
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
parser.add_argument('--key', '-k', dest='key',
                    default='/demo/example/zenoh-python-put',
                    type=str,
                    help='The key expression to write.')
parser.add_argument('--value', '-v', dest='value',
                    default='Put from Python!',
                    type=str,
                    help='The value to write.')
parser.add_argument('--config', '-c', dest='config',
                    metavar='FILE',
                    type=str,
                    help='A configuration file.')

args = parser.parse_args()
conf = zenoh.config_from_file(args.config) if args.config is not None else zenoh.Config()
if args.mode is not None:
    conf.insert_json5("mode", json.dumps(args.mode))
if args.peer is not None:
    conf.insert_json5("peers", json.dumps(args.peer))
if args.listener is not None:
    conf.insert_json5("listeners", json.dumps(args.listener))
key = args.key
value = args.value

# zenoh-net code  --- --- --- --- --- --- --- --- --- --- ---

# initiate logging
zenoh.init_logger()

print("Openning session...")
session = zenoh.open(conf)

print("Putting Data ('{}': '{}')...".format(key, value))
session.put(key, value)

# --- Examples of put with other types:

# - Integer
# session.put('/demo/example/Integer', 3)

# - Float
# session.put('/demo/example/Float', 3.14)

# - Properties (as a Dictionary with str only)
# session.put('/demo/example/Properties', {'p1': 'v1', 'p2': 'v2'})

# - Json (str format)
# session.put('/demo/example/Json',
#             (json.dumps(['foo', {'bar': ('baz', None, 1.0, 2)}]), Encoding.TEXT_JSON))

# - Raw ('application/octet-stream' encoding by default)
# session.put('/demo/example/Raw', b'\x48\x69\x21')

# - Custom encoding
# session.put('/demo/example/Custom',
#             (b'\x48\x69\x21', 'my_encoding'))

# - UTF-16 String specifying the charset as Encoding suffix
# session.put('/demo/example/UTF-16', ('hello'.encode('utf-16'), Encoding.TEXT_PLAIN.with_suffix(';charset=utf-16')))

session.close()
