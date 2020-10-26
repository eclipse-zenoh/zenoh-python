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
from zenoh.net import config

# --- Command line argument parsing --- --- --- --- --- ---
parser = argparse.ArgumentParser(
    prog='zn_info',
    description='zenoh-net info example')
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

args = parser.parse_args()
conf = { "mode": args.mode }
if args.peer is not None:
    conf["peer"] = ",".join(args.peer)
if args.listener is not None:
    conf["listener"] = ",".join(args.listener)
# zenoh-net code  --- --- --- --- --- --- --- --- --- --- ---

# initiate logging
zenoh.init_logger()

conf['user'] = 'user'
conf['password'] = 'password'

print("Openning session...")
session = zenoh.net.open(conf)

info = session.info()
for key in info:
    print("{} : {}".format(key, info[key]))

session.close()
