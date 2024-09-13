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

        print("Declaring Subscriber on '{}'...".format(key))
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
                    f">> [Subscriber] Received {sample.kind} ('{sample.key_expr}': '{sample.payload.deserialize(str)}')"
                )


# --- Command line argument parsing --- --- --- --- --- ---
if __name__ == "__main__":
    import argparse
    import json

    parser = argparse.ArgumentParser(prog="z_pull", description="zenoh pull example")
    parser.add_argument(
        "--mode",
        "-m",
        dest="mode",
        choices=["peer", "client"],
        type=str,
        help="The zenoh session mode.",
    )
    parser.add_argument(
        "--connect",
        "-e",
        dest="connect",
        metavar="ENDPOINT",
        action="append",
        type=str,
        help="Endpoints to connect to.",
    )
    parser.add_argument(
        "--listen",
        "-l",
        dest="listen",
        metavar="ENDPOINT",
        action="append",
        type=str,
        help="Endpoints to listen on.",
    )
    parser.add_argument(
        "--key",
        "-k",
        dest="key",
        default="demo/example/**",
        type=str,
        help="The key expression matching resources to pull.",
    )
    parser.add_argument(
        "--config",
        "-c",
        dest="config",
        metavar="FILE",
        type=str,
        help="A configuration file.",
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
    conf = (
        zenoh.Config.from_file(args.config)
        if args.config is not None
        else zenoh.Config()
    )
    if args.mode is not None:
        conf.insert_json5("mode", json.dumps(args.mode))
    if args.connect is not None:
        conf.insert_json5("connect/endpoints", json.dumps(args.connect))
    if args.listen is not None:
        conf.insert_json5("listen/endpoints", json.dumps(args.listen))

    main(conf, args.key, args.size, args.interval)
