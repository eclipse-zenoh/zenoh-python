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
from zenoh import Reliability, SampleKind, SubMode, Sample, KeyExpr
from zenoh.queryable import STORAGE

# --- Command line argument parsing --- --- --- --- --- ---
parser = argparse.ArgumentParser(
    prog='z_storage',
    description='zenoh storage example')
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
                    default='/demo/example/**',
                    type=str,
                    help='The key expression matching resources to store.')
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

# zenoh-net code  --- --- --- --- --- --- --- --- --- --- ---

store = {}


def listener(sample):
    print(">> [Subscriber] Received {} ('{}': '{}')"
          .format(sample.kind, sample.key_expr, sample.payload.decode("utf-8")))
    if sample.kind == SampleKind.DELETE:
        store.pop(str(sample.key_expr), None)
    else:
        store[str(sample.key_expr)] = (sample.value, sample.source_info)


def query_handler(query):
    print(">> [Queryable ] Received Query '{}'".format(query.selector))
    replies = []
    for stored_name, (data, source_info) in store.items():
        if KeyExpr.intersect(query.key_selector, stored_name):
            sample = Sample(stored_name, data)
            sample.with_source_info(source_info)
            query.reply(sample)


# initiate logging
zenoh.init_logger()

print("Openning session...")
session = zenoh.open(conf)

print("Creating Subscriber on '{}'...".format(key))
sub = session.subscribe(key, listener, reliability=Reliability.Reliable, mode=SubMode.Push)

print("Creating Queryable on '{}'...".format(key))
queryable = session.queryable(key, STORAGE, query_handler)

print("Enter 'q' to quit......")
c = '\0'
while c != 'q':
    c = sys.stdin.read(1)
    if c == '':
        time.sleep(1)

sub.close()
queryable.close()
session.close()
