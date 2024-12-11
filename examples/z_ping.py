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


def main(conf: zenoh.Config, payload_size: int, warmup: int, samples: int):
    # initiate logging
    zenoh.init_log_from_env_or("error")

    print("Opening session...")
    with zenoh.open(conf) as session:

        sub = session.declare_subscriber("test/pong")
        pub = session.declare_publisher(
            "test/ping", congestion_control=zenoh.CongestionControl.BLOCK
        )
        data = bytes(i % 10 for i in range(0, payload_size))

        print(f"Warming up for {warmup}...")
        warmup_end = time.time() + warmup
        while time.time() < warmup_end:
            pub.put(data)
            sub.recv()

        sample_list = []
        for i in range(samples):
            write_time = time.time()
            pub.put(data)
            sub.recv()
            sample_list.append(round((time.time() - write_time) * 1_000_000))

        for i, rtt in enumerate(sample_list):
            print(f"{payload_size} bytes: seq={i} rtt={rtt}µs lat={rtt / 2}µs")


# --- Command line argument parsing --- --- --- --- --- ---
if __name__ == "__main__":
    import argparse
    import json

    import common

    parser = argparse.ArgumentParser(prog="z_ping", description="zenoh get example")
    common.add_config_arguments(parser)
    parser.add_argument(
        "--warmup",
        "-w",
        dest="warmup",
        metavar="WARMUP",
        type=float,
        default=1.0,
        help="The number of seconds to warmup (float)",
    )
    parser.add_argument(
        "--samples",
        "-n",
        dest="samples",
        metavar="SAMPLES",
        type=int,
        default=100,
        help="The number of round-trip to measure",
    )
    parser.add_argument(
        "payload_size",
        metavar="PAYLOAD_SIZE",
        type=int,
        help="Sets the size of the payload to publish.",
    )
    parser.add_argument(
        "--no-multicast-scouting",
        dest="no_multicast_scouting",
        default=False,
        action="store_true",
        help="Disable multicast scouting.",
    )

    args = parser.parse_args()
    conf = common.get_config_from_args(args)

    main(conf, args.payload_size, args.warmup, args.samples)
