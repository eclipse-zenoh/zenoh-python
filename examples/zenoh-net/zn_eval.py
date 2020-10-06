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
from zenoh.net import config, Sample
from zenoh.net.queryable import EVAL

# --- Command line argument parsing --- --- --- --- --- ---
parser = argparse.ArgumentParser(
    prog='zn_eval',
    description='zenoh-net eval example')
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
                    default='/demo/example/zenoh-python-eval',
                    type=str,
                    help='The name of the resource to evaluate.')
parser.add_argument('--value', '-v', dest='value',
                    default='Eval from Python!',
                    type=str,
                    help='The value to reply to queries.')

args = parser.parse_args()
conf = []
conf.append((config.ZN_MODE_KEY, args.mode.encode('utf-8')))
if args.peer is not None:
    for peer in args.peer:
        conf.append((config.ZN_PEER_KEY, peer.encode('utf-8')))
if args.listener is not None:
    for listener in args.listener:
        conf.append((config.ZN_LISTENER_KEY, listener.encode('utf-8')))
path = args.path
value = args.value

# zenoh-net code  --- --- --- --- --- --- --- --- --- --- ---


def eval_callback(query):
    print(">> [Query handler] Handling '{}{}'".format(
        query.res_name, query.predicate))
    query.reply(Sample(res_name=path, payload=value.encode()))


# initiate logging
zenoh.init_logger()

print("Openning session...")
session = zenoh.net.open(conf)

print("Declaring Queryable on '{}'...".format(path))
queryable = session.declare_queryable(
    path, EVAL, eval_callback)

print("Press q to stop...")
c = '\0'
while c != 'q':
    c = sys.stdin.read(1)

queryable.undeclare()
session.close()
