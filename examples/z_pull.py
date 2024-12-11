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


def main(conf: zenoh.Config, key: str, size: int, interval: int):
    # initiate logging
    zenoh.init_log_from_env_or("error")

    print("Opening session...")
    with zenoh.open(conf) as session:

        print(f"Declaring Subscriber on '{key}'...")
        # Subscriber doesn't receive messages over the RingBuffer size.
        # The oldest message is overwritten by the latest one.
        sub = session.declare_subscriber(key, zenoh.handlers.RingChannel(size))

        print("Press CTRL-C to quit...")
        while True:
            time.sleep(interval)
            while True:
                sample = sub.try_recv()
                if sample is None:
                    break
                print(
                    f">> [Subscriber] Received {sample.kind} ('{sample.key_expr}': '{sample.payload.to_string()}')"
                )


# --- Command line argument parsing --- --- --- --- --- ---
if __name__ == "__main__":
    import argparse
    import json

    import common

    parser = argparse.ArgumentParser(prog="z_pull", description="zenoh pull example")
    common.add_config_arguments(parser)
    parser.add_argument(
        "--key",
        "-k",
        dest="key",
        default="demo/example/**",
        type=str,
        help="The key expression matching resources to pull.",
    )
    parser.add_argument(
        "--size", dest="size", default=3, type=int, help="The size of the ringbuffer"
    )
    parser.add_argument(
        "--interval",
        dest="interval",
        default=1.0,
        type=float,
        help="The interval for pulling the ringbuffer",
    )

    args = parser.parse_args()
    conf = common.get_config_from_args(args)

    main(conf, args.key, args.size, args.interval)
