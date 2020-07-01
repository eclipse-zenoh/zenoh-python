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
from zenoh import Zenoh, Selector, Path, Workspace, Encoding, Value

# --- Command line argument parsing --- --- --- --- --- ---
parser = argparse.ArgumentParser(
    prog='z_get',
    description='Issues a query for a selector specified by the command-line')

parser.add_argument('--selector', '-s', dest='selector',
                    default='/zenoh/examples/**',
                    type=str,
                    help='The selector to be used for issuing the query')

parser.add_argument(
    '--locator', '-l', dest='locator',
    default=None,
    type=str,
    help='The locator to be used to boostrap the zenoh session.'
         ' By default dynamic discovery is used')


args = parser.parse_args()
selector = args.selector
locator = args.locator

# zenoh code  --- --- --- --- --- --- --- --- --- --- ---
print('Login to Zenoh...')
z = Zenoh.login(locator)
w = z.workspace()

print('Get from {}'.format(selector))
for data in w.get(selector):
    print('  {} : {} - {}'.format(data.path, data.value, data.value.encoding))


z.logout()
