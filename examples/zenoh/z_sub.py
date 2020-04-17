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
from zenoh import Zenoh, Selector, Path, Workspace
from zenoh import Change, ChangeKind, Encoding, Value

# --- Command line argument parsing --- --- --- --- --- ---
parser = argparse.ArgumentParser(
    prog='z_sub',
    description='An example illustrating zenoh subscribers')
parser.add_argument('--selector', '-s', dest='selector',
                    default='/zenoh/examples/**',
                    type=str,
                    help='The selector specifying the subscription')

parser.add_argument(
    '--locator', '-l', dest='locator',
    default=None,
    type=str,
    help='The locator to be used to boostrap the zenoh session.'
         ' By default dynamic discovery is used')


args = parser.parse_args()

locator = args.locator
selector = args.selector

# zenoh code  --- --- --- --- --- --- --- --- --- --- ---


print('Login to zenoh...')
z = Zenoh.login(locator)

w = z.workspace()


def listener(changes):
    for change in changes:
        v = change.get_value()
        if change.get_kind() == ChangeKind.PUT:
            print('>> [Subscription listener] Received PUT on "{}": {} [{}] {}'
                  .format(change.get_path(), v.get_value(), v.get_encoding(), type(v.get_value())))
        elif change.get_kind() == ChangeKind.UPDATE:
            print('>> [Subscription listener] Received UPDATE on "{}": {} [{}]'
                  .format(change.get_path(), v.get_value()))
        elif change.get_kind() == ChangeKind.REMOVE:
            print('>> [Subscription listener] Received REMOVE on "{}"'
                  .format(change.get_path()))
        else:
            print('>> [Subscription listener] Received kind:"{}" on "{}"'
                  .format(change.get_kind(), change.get_path()))


print('Subscribe on {}'.format(selector))
subid = w.subscribe(selector, listener)

print('Enter \'q\' to quit...')
while sys.stdin.read(1) != 'q':
    pass

w.unsubscribe(subid)
z.logout()
