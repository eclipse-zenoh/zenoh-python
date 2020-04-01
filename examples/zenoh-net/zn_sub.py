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


### --- Command line argument parsing --- --- --- --- --- --- 
parser = argparse.ArgumentParser(prog='zn_sub', description='An example illustrating zenoh subscribers')
parser.add_argument('--selector', '-s', dest='selector',
                    default='/zenoh/examples/**',
                    type=str,
                    help='The selector specifying the subscription')

parser.add_argument('--locator', '-l', dest='locator',
                    default=None,
                    type=str,
                    help='The locator to be used to boostrap the zenoh session. By default dynamic discovery is used')


args = parser.parse_args()

locator = args.locator
selector= args.selector

### zenoh code  --- --- --- --- --- --- --- --- --- --- --- 


def listener(rname, data, info):
    print(">> [Subscription listener] Received ('{}': '{}') at {}"
          .format(rname, data.decode("utf-8"), info.tstamp))


print("Openning session...")
s = Session.open(locator)

print("Declaring Subscriber on '{}'...".format(selector))
sub = s.declare_subscriber(selector, SubscriberMode.push(), listener)

print('Press "q" at any time to terminate...')
c = '\0'
while c != 'q':
    c = sys.stdin.read(1)

s.undeclare_subscriber(sub)
s.close()
