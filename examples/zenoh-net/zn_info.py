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
import binascii
import argparse
from zenoh.net import (
    Session,
    ZN_USER_KEY, ZN_PASSWD_KEY,
    ZN_INFO_PEER_KEY, ZN_INFO_PID_KEY, ZN_INFO_PEER_PID_KEY
)

### --- Command line argument parsing --- --- --- --- --- --- 
parser = argparse.ArgumentParser(prog='zn_info', description='Shows how to retrieve peers information')

parser.add_argument('--locator', '-l', dest='locator',
                    default=None,
                    type=str,
                    help='The locator to be used to boostrap the zenoh session. By default dynamic discovery is used')

args = parser.parse_args()

locator = args.locator

### zenoh-net code  --- --- --- --- --- --- --- --- --- --- --- 

print("Openning session...")
s = Session.open(locator, {ZN_USER_KEY: "user".encode(),
                 ZN_PASSWD_KEY: "password".encode()})

info = s.info()
peer = info[ZN_INFO_PEER_KEY]
pid = info[ZN_INFO_PID_KEY]
peer_pid = info[ZN_INFO_PEER_PID_KEY]
print("LOCATOR :  {}".format(peer.decode("utf-8")))
print("PID :      {}".format(binascii.hexlify(pid).decode("ascii")))
print("PEER PID : {}".format(binascii.hexlify(peer_pid).decode("ascii")))

s.close()
