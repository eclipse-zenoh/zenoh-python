#
# Copyright (c) 2022 ZettaScale Technology
#
# This program and the accompanying materials are made available under the
# terms of the Eclipse Public License 2.0 which is available at
# http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
# which is available at https://www.apache.org/licenses/LICENSE-2.0.
#
# SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
#
# Contributors:
#   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
#

import sys
import time
import datetime
import argparse
import json
import zenoh
from zenoh import Reliability, SubMode

# --- Command line argument parsing --- --- --- --- --- ---
parser = argparse.ArgumentParser(
    prog='z_sub_thr',
    description='zenoh throughput sub example')
parser.add_argument('--mode', '-m', dest='mode',
                    choices=['peer', 'client'],
                    type=str,
                    help='The zenoh session mode.')
parser.add_argument('--connect', '-e', dest='connect',
                    metavar='ENDPOINT',
                    action='append',
                    type=str,
                    help='Endpoints to connect to.')
parser.add_argument('--listen', '-l', dest='listen',
                    metavar='ENDPOINT',
                    action='append',
                    type=str,
                    help='Endpoints to listen on.')
parser.add_argument('--samples', '-s', dest='samples',
                    default=10,
                    metavar='NUMBER',
                    action='append',
                    type=int,
                    help='Number of throughput measurements.')
parser.add_argument('--number', '-n', dest='number',
                    default=50000,
                    metavar='NUMBER',
                    action='append',
                    type=int,
                    help='Number of messages in each throughput measurements.')
parser.add_argument('--config', '-c', dest='config',
                    metavar='FILE',
                    type=str,
                    help='A configuration file.')

args = parser.parse_args()
conf = zenoh.config_from_file(
    args.config) if args.config is not None else zenoh.Config()
if args.mode is not None:
    conf.insert_json5(zenoh.config.MODE_KEY, json.dumps(args.mode))
if args.connect is not None:
    conf.insert_json5(zenoh.config.CONNECT_KEY, json.dumps(args.connect))
if args.listen is not None:
    conf.insert_json5(zenoh.config.LISTEN_KEY, json.dumps(args.listen))
m = args.samples
n = args.number

# zenoh-net code  --- --- --- --- --- --- --- --- --- --- ---


def print_stats(start):
    stop = datetime.datetime.now()
    print("{:.6f} msgs/sec".format(n / (stop - start).total_seconds()))


count = 0
start = None
nm = 0


def listener(sample):
    global n, m, count, start, nm
    if count == 0:
        start = datetime.datetime.now()
        count += 1
    elif count < n:
        count += 1
    else:
        print_stats(start)
        nm += 1
        count = 0
        if nm >= m:
            sys.exit(0)


# initiate logging
zenoh.init_logger()

session = zenoh.open(conf)

rid = session.declare_expr('/test/thr')

sub = session.subscribe(
    rid, listener, reliablity=Reliability.Reliable, mode=SubMode.Push)

time.sleep(600)

session.undeclare_expr(rid)
session.close()
