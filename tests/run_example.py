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
"""Run an example with a private Zenoh configuration in this subprocess."""

import runpy
import sys
from pathlib import Path
from unittest.mock import patch

import zenoh


def main():
    config_json, script, *args = sys.argv[1:]
    open_session = zenoh.open
    scout = zenoh.scout

    def open_example(_config, **kwargs):
        return open_session(zenoh.Config.from_json5(config_json), **kwargs)

    def scout_example(handler=None, what=None, config=None):
        return scout(handler, what, config=zenoh.Config.from_json5(config_json))

    sys.argv = [script, *args]
    sys.path.insert(0, str(Path(script).parent))
    with patch.multiple(zenoh, open=open_example, scout=scout_example):
        runpy.run_path(script, run_name="__main__")


if __name__ == "__main__":
    main()
