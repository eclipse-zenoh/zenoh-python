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
from zenoh.net import Session, zn_rname_intersect

### --- Command line argument parsing --- --- --- --- --- --- 
parser = argparse.ArgumentParser(prog='zn_storage', description='Implements and register a zenoh storage')
parser.add_argument('--selector', '-s', dest='selector',
                    default='/zenoh/examples/**',
                    type=str,
                    help='the selector associated with this storage')

parser.add_argument('--locator', '-l', dest='locator',
                    default=None,
                    type=str,
                    help='The locator to be used to boostrap the zenoh session. By default dynamic discovery is used')

args = parser.parse_args()

selector = args.selector
locator = args.locator

### zenoh-net code  --- --- --- --- --- --- --- --- --- --- --- 


store = {}

def listener(rname, data, info):
    print(">> [Storage listener] Received ('{}': '{}')"
          .format(rname, data.decode("utf-8")))
    store[rname] = (data, info)


def query_handler(path_selector, content_selector, send_replies):
    print(">> [Query handler   ] Handling '{}?{}'"
          .format(path_selector, content_selector))
    replies = []
    for k, v in store.items():
        if zn_rname_intersect(path_selector, k):
            replies.append((k, v))
    send_replies(replies)


print("Openning session...")
s = Session.open(locator)

print("Declaring Storage on '{}'...".format(selector))
sto = s.declare_storage(selector, listener, query_handler)

print('Press "q" at any time to terminate...')
c = '\0'
while c != 'q':
    c = sys.stdin.read(1)

s.undeclare_storage(sto)
s.close()
