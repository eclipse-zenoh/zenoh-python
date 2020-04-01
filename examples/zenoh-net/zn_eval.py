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
from zenoh.net import Session, DataInfo, ZN_PUT

### --- Command line argument parsing --- --- --- --- --- --- 
parser = argparse.ArgumentParser(prog='zn_eval', description='Shows how to use zenoh-net evaluated/computed resources')
parser.add_argument('--path', '-p', dest='path',
                    default='/zenoh/examples/python/eval',
                    type=str,
                    help='the path representing the  URI')

parser.add_argument('--locator', '-l', dest='locator',
                    default=None,
                    type=str,
                    help='The locator to be used to boostrap the zenoh session. By default dynamic discovery is used')

args = parser.parse_args()

path = args.path
locator = args.locator

### zenoh-net code  --- --- --- --- --- --- --- --- --- --- --- 


def query_handler(path_selector, content_selector, send_replies):
    print(">> [Query handler] Handling '{}?{}'"
          .format(path_selector, content_selector))
    k, v = path, "Eval from Python!".encode()
    send_replies([(k, (v, DataInfo(kind=ZN_PUT)))])


print("Openning zenoh session...")
s = Session.open(locator)

print("Declaring Eval on '{}'...".format(path))
e = s.declare_eval(path, query_handler)

print('Press "q" at any time to terminate...')
c = '\0'
while c != 'q':
    c = sys.stdin.read(1)
    
s.undeclare_eval(e)
s.close()
