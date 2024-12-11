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
import time

import zenoh


def main(conf: zenoh.Config, express: bool):
    # initiate logging
    zenoh.init_log_from_env_or("error")

    print("Opening session...")
    with zenoh.open(conf) as session:
        pub = session.declare_publisher(
            "test/pong",
            congestion_control=zenoh.CongestionControl.BLOCK,
            express=express,
        )
        session.declare_subscriber("test/ping", lambda s: pub.put(s.payload))

        print("Press CTRL-C to quit...")
        while True:
            time.sleep(1)


# --- Command line argument parsing --- --- --- --- --- ---
if __name__ == "__main__":
    import argparse
    import json

    import common

    parser = argparse.ArgumentParser(prog="z_pong", description="zenoh get example")
    common.add_config_arguments(parser)
    parser.add_argument(
        "--express",
        dest="express",
        metavar="EXPRESS",
        type=bool,
        default=False,
        help="Express publishing",
    )

    args = parser.parse_args()
    conf = common.get_config_from_args(args)

    main(conf, args.express)
