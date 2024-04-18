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

import asyncio
import sys
import time
import argparse
import zenoh
from zenoh import WhatAmI


async def main():
    # initiate logging
    zenoh.init_logger()

    print("Scouting...")
    hellos = await zenoh.async_scout(WhatAmI.Peer | WhatAmI.Router, 1.0)

    for hello in hellos:
        print(hello)

asyncio.run(main())
