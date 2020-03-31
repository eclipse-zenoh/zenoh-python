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
import itertools
import argparse
from zenoh.net import Session

### --- Command line argument parsing --- --- --- --- --- --- 
parser = argparse.ArgumentParser(prog='z_stream', description='Illustrates the use of a zenoh-net publisher')
parser.add_argument('--path', '-p', dest='path',
                    default='/zenoh/examples/python-stream',
                    type=str,
                    help='the path representing the  URI')

parser.add_argument('--locator', '-l', dest='locator',
                    default=None,
                    type=str,
                    help='The locator to be used to boostrap the zenoh session. By default dynamic discovery is used')

parser.add_argument('--msg', '-m', dest='msg',
                    default='Zenitude from zenoh!',
                    type=str,
                    help='The quote associated with the welcoming resource')

args = parser.parse_args()
msg = args.msg
path = args.path
locator = args.locator

### zenoh-net code  --- --- --- --- --- --- --- --- --- --- --- 

print("Openning session...")
s = Session.open(locator)

print("Declaring Publisher on '{}'...".format(path))
pub = s.declare_publisher(path)

for idx in itertools.count():
    time.sleep(1)
    buf = "[{:4d}] {}".format(idx, msg)
    print("Streaming Data ('{}': '{}')...".format(path, buf))
    s.stream_data(pub, bytes(buf, encoding='utf8'))

s.undeclare_publisher(pub)
s.close()
