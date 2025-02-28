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
import time

import zenoh


def main(conf: zenoh.Config, key: str, payload: str, complete: bool):
    # initiate logging
    zenoh.init_log_from_env_or("error")

    print("Opening session...")
    with zenoh.open(conf) as session:
        print(f"Declaring Queryable on '{key}'...")
        queryable = session.declare_queryable(key, complete=complete)

        print("Press CTRL-C to quit...")
        while True:
            with queryable.recv() as query:
                if query.payload is not None:
                    print(
                        f">> [Queryable ] Received Query '{query.selector}'"
                        f" with payload: '{query.payload.to_string()}'"
                    )
                else:
                    print(f">> [Queryable ] Received Query '{query.selector}'")
                query.reply(key, payload)
                # it's possible to call `query.drop()` after handling it
                # instead of using a context manager


# --- Command line argument parsing --- --- --- --- --- ---
if __name__ == "__main__":
    import argparse
    import json

    import common

    parser = argparse.ArgumentParser(
        prog="z_queryable", description="zenoh queryable example"
    )
    common.add_config_arguments(parser)
    parser.add_argument(
        "--key",
        "-k",
        dest="key",
        default="demo/example/zenoh-python-queryable",
        type=str,
        help="The key expression matching queries to reply to.",
    )
    parser.add_argument(
        "--payload",
        "-p",
        dest="payload",
        default="Queryable from Python!",
        type=str,
        help="The payload to reply to queries.",
    )
    parser.add_argument(
        "--complete",
        dest="complete",
        default=False,
        action="store_true",
        help="Declare the queryable as complete w.r.t. the key expression.",
    )

    args = parser.parse_args()
    conf = common.get_config_from_args(args)

    main(conf, args.key, args.payload, args.complete)
