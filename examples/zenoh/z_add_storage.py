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
parser = argparse.ArgumentParser(
    prog='z_add_storage', description='Adds a storage')
parser.add_argument('--selector', '-s', dest='selector',
                    default='/zenoh/examples/**',
                    type=str,
                    help='the selector associated with this storage')

parser.add_argument('--id', '-i', dest='id',
                    default='zenoh-examples-storage',
                    type=str,
                    help='the storage identifier')

parser.add_argument(
    '--locator', '-l', dest='locator',
    default=None,
    type=str,
    help='The locator to be used to boostrap the zenoh session.'
         ' By default dynamic discovery is used')

args = parser.parse_args()


# zenoh code  --- --- --- --- --- --- --- --- --- --- ---
print('Login to Zenoh...')
z = Zenoh.login(args.locator)

a = z.admin()

print('Add storage {} with selector {}'.format(args.id, args.selector))
properties = {'selector': args.selector}
a.add_storage(args.id, properties)

z.logout()
