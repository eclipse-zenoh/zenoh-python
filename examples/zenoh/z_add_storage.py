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
from zenoh import Zenoh, Selector, Path, Workspace, Encoding, Value

selector = '/demo/example/**'
if len(sys.argv) > 1:
    selector = sys.argv[1]

storage_id = 'Demo'
if len(sys.argv) > 2:
    storage_id = sys.argv[2]

locator = None
if len(sys.argv) > 3:
    locator = sys.argv[3]

print('Login to Zenoh (locator={})...'.format(locator))
z = Zenoh.login(locator)

a = z.admin()

print('Add storage {} with selector {}'.format(storage_id, selector))
properties = {'selector': selector}
a.add_storage(storage_id, properties)

z.logout()
