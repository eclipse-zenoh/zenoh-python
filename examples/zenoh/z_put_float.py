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

# --- Command line argument parsing --- --- --- --- --- ---
parser = argparse.ArgumentParser(prog='z_put_float',
                                 description='Produces float values')
parser.add_argument('--path', '-p', dest='path',
                    default='/zenoh/examples/native/float',
                    type=str,
                    help='the path representing the float resource')

parser.add_argument(
    '--locator', '-l', dest='locator',
    default=None,
    type=str,
    help='The locator to be used to boostrap the zenoh session.'
         ' By default dynamic discovery is used')

args = parser.parse_args()

# zenoh code  --- --- --- --- --- --- --- --- --- --- ---
z = Zenoh.login(args.locator)
w = z.workspace()

while (True):
    v = input("Insert value (\'.\' to exit): ")
    if v != '.':
        w.put(args.path, float(v))
    else:
        z.logout()
        break
