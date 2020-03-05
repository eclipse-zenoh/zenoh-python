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

# If not specified as 1st argument, use a relative path
# (to the workspace below): 'zenoh-python-put'
path = 'zenoh-python-put'
if len(sys.argv) > 1:
    path = sys.argv[1]

locator = None
if len(sys.argv) > 2:
    locator = sys.argv[2]

print('Login to Zenoh (locator={})...'.format(locator))
z = Zenoh.login(locator)

print('Use Workspace on "/demo/example"')
w = z.workspace('/demo/example')

print('Remove {}'.format(path))
w.remove(path)

z.logout()
