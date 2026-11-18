#
# Copyright (c) 2026 ZettaScale Technology
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
import sys
from pathlib import Path

import pytest

# Share configuration with executable examples without importing the source
# tree's zenoh package in place of the installed extension.
sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "examples"))

from common.testing import open_session_pair


@pytest.fixture
def session_pair(request):
    def open_pair(**listener_options):
        listener, connector = open_session_pair(**listener_options)
        request.addfinalizer(listener.close)
        request.addfinalizer(connector.close)
        return listener, connector

    return open_pair
