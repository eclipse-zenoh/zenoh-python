#
# Copyright (c) 2024 ZettaScale Technology
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
from zenoh.config import ZenohId

class SessionInfo:
    def zid(self) -> ZenohId:
        """Return the ZenohId of the current zenoh Session."""

    def routers_zid(self) -> list[ZenohId]:
        """Return the ZenohId of the zenoh routers this process is currently connected to or the ZenohId of the current router if this code is run from a router (plugin)."""

    def peers_zid(self) -> list[ZenohId]:
        """Return the ZenohId of the zenoh peers this process is currently connected to."""
