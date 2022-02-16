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
from zenoh import config, queryable, QueryTarget, Target

# --- Command line argument parsing --- --- --- --- --- ---
parser = argparse.ArgumentParser(
    prog='z_get',
    description='zenoh get example')
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
parser.add_argument('--selector', '-s', dest='selector',
                    default='/demo/example/**',
                    type=str,
                    help='The selection of resources to query.')
parser.add_argument('--kind', '-k', dest='kind',
                    choices=['ALL_KINDS', 'STORAGE', 'EVAL'],
                    default='ALL_KINDS',
                    type=str,
                    help='The KIND of queryables to query.')
parser.add_argument('--target', '-t', dest='target',
                    choices=['ALL', 'BEST_MATCHING', 'ALL_COMPLETE', 'NONE'],
                    default='ALL',
                    type=str,
                    help='The target queryables of the query.')
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
selector = args.selector
kind = {
    'ALL_KINDS': queryable.ALL_KINDS,
    'STORAGE': queryable.STORAGE,
    'EVAL': queryable.EVAL}.get(args.kind)
target = {
    'ALL': Target.All(),
    'BEST_MATCHING': Target.BestMatching(),
    'ALL_COMPLETE': Target.AllComplete(),
    'NONE': Target.No()}.get(args.target)

# zenoh-net code  --- --- --- --- --- --- --- --- --- --- ---

# initiate logging
zenoh.init_logger()

print("Openning session...")
session = zenoh.open(conf)

print("Sending Query '{}'...".format(selector))
replies = session.get(selector, target=QueryTarget(kind, target))
for reply in replies:
    print(">> Received ('{}': '{}')"
          .format(reply.data.key_expr, reply.data.payload.decode("utf-8")))

session.close()
