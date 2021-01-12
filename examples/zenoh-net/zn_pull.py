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
from datetime import datetime
import argparse
import zenoh
from zenoh.net import config, SubInfo, Reliability, SubMode

# --- Command line argument parsing --- --- --- --- --- ---
parser = argparse.ArgumentParser(
    prog='zn_pull',
    description='zenoh-net pull example')
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
parser.add_argument('--selector', '-s', dest='selector',
                    default='/demo/example/**',
                    type=str,
                    help='The selection of resources to pull.')

args = parser.parse_args()
conf = {"mode": args.mode}
if args.peer is not None:
    conf["peer"] = ",".join(args.peer)
if args.listener is not None:
    conf["listener"] = ",".join(args.listener)
selector = args.selector

# zenoh-net code  --- --- --- --- --- --- --- --- --- --- ---


def listener(sample):
    time = '(not specified)' if sample.data_info is None else datetime.fromtimestamp(
        sample.data_info.timestamp.time)
    print(">> [Subscription listener] Received ('{}': '{}') published at {}"
          .format(sample.res_name, sample.payload.decode("utf-8"), time))


# initiate logging
zenoh.init_logger()

print("Openning session...")
session = zenoh.net.open(conf)

print("Declaring Subscriber on '{}'...".format(selector))
sub_info = SubInfo(Reliability.Reliable, SubMode.Pull)

sub = session.declare_subscriber(selector, sub_info, listener)

print("Press <enter> to pull data...")
c = sys.stdin.read(1)
while c != 'q':
    sub.pull()
    c = sys.stdin.read(1)

sub.undeclare()
session.close()
