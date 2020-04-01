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
import argparse
from zenoh import Zenoh, Workspace

### --- Command line argument parsing --- --- --- --- --- --- 
parser = argparse.ArgumentParser(prog='z_put', description='Produces welcome messages')
parser.add_argument('--path', '-p', dest='path',
                    default='/zenoh/examples/python/put/hello',
                    type=str,
                    help='the path representing the  URI')

parser.add_argument('--locator', '-l', dest='locator',
                    default=None,
                    type=str,
                    help='The locator to be used to boostrap the zenoh session. By default dynamic discovery is used')

parser.add_argument('--msg', '-m', dest='msg',
                    default='Zenitude put from zenoh-python!',
                    type=str,
                    help='The quote associated with the welcoming resource')

args = parser.parse_args()


### zenoh code  --- --- --- --- --- --- --- --- --- --- --- 
print('Login to Zenoh...')
z = Zenoh.login(args.locator)
w = z.workspace(args.path)

z = Zenoh.login(args.locator)

w = z.workspace()
w.put(args.path, args.msg)
z.logout()
