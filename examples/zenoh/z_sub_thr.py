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
import datetime
import argparse
from zenoh import Zenoh, Selector, Path, Workspace, Encoding, Value

### --- Command line argument parsing --- --- --- --- --- --- 
parser = argparse.ArgumentParser(prog='z_sub_thr', description='The zenoh throughput subscriber')

parser.add_argument('--path', '-p', dest='path',
                    default='/zenoh/examples/throughput/data',
                    type=str,
                    help='The subscriber path')

parser.add_argument('--locator', '-l', dest='locator',
                    default=None,
                    type=str,
                    help='The locator to be used to boostrap the zenoh session. By default dynamic discovery is used')

args = parser.parse_args()

locator = args.locator
path = args.path

### zenoh code  --- --- --- --- --- --- --- --- --- --- --- 

N = 100000

count = 0
start = None
stop = None


def print_stats(start, stop):
    print("{:.6f} msgs/sec".format(N / (stop - start).total_seconds()))

# Listener function triggered each time there is a change on a resource whose
# path matches the selector
def listener(changes):
    global count, start, stop
    if count == 0:
        start = datetime.datetime.now()
        count += 1
    elif count < N:
        count += 1
    else:
        stop = datetime.datetime.now()
        print_stats(start, stop)
        count = 0


print('Login to Zenoh...')
z = Zenoh.login(locator)

w = z.workspace()

print("Subscribe on {}".format(path))
subid = w.subscribe(path, listener)

time.sleep(60)

w.unsubscribe(subid)
z.logout()
