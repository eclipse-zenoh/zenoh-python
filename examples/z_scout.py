#
# Copyright (c) 2022 ZettaScale Technology
#
# This program and the accompanying materials are made available under the
# terms of the Eclipse Public License 2.0 which is available at
# http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
# which is available at https://www.apache.org/licenses/LICENSE-2.0.
#
# SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
#
# Contributors:
#   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
#

import zenoh

# initiate logging
zenoh.init_logger()

print("Scouting...")
scout = zenoh.scout(what = "peer|router", timeout=1.0)

def dbg(x):
    print(x)
    return x
for hello in dbg(scout.receiver()):
    print(hello)
