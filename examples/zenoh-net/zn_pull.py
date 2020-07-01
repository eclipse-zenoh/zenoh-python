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
from zenoh.net import Session, SubscriberMode


# --- Command line argument parsing --- --- --- --- --- ---
parser = argparse.ArgumentParser(
    prog='zn_pull',
    description='Illustrates the use of a pull subscriber')

parser.add_argument(
    '--selector', '-s', dest='selector',
    default='/zenoh/examples/**',
    type=str,
    help='The selector to be used for issuing the pull subscription')

parser.add_argument(
    '--locator', '-l', dest='locator',
    default=None,
    type=str,
    help='The locator to be used to boostrap the zenoh session.'
         ' By default dynamic discovery is used')

args = parser.parse_args()
selector = args.selector
locator = args.locator


# zenoh-net code  --- --- --- --- --- --- --- --- --- --- ---
def listener(rname, data, info):
    print(">> [Subscription listener] Received ('{}': '{}')"
          .format(rname, data.decode("utf-8")))


print("Openning session...")
s = Session.open(locator)

print("Declaring Subscriber on '{}'...".format(selector))
sub = s.declare_subscriber(selector, SubscriberMode.pull(), listener)

print("Press <enter> to pull data...")
c = '\0'
while c != 'q':
    c = sys.stdin.read(1)
    s.pull(sub)

s.undeclare_subscriber(sub)
s.close()
