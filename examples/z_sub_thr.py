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

batch_count = 0
count = 0
start = None
global_start = None


def main(conf: zenoh.Config, number: int):
    def listener(_sample: zenoh.Sample):
        global count, batch_count, start, global_start
        if count == 0:
            start = time.time()
            if global_start is None:
                global_start = start
            count += 1
        elif count < number:
            count += 1
        else:
            stop = time.time()
            print(f"{number / (stop - start):.6f} msgs/sec")
            batch_count += 1
            count = 0

    def report():
        assert global_start is not None
        end = time.time()
        total = batch_count * number + count
        print(
            f"Received {total} messages in {end - global_start}: averaged {total / (end - global_start):.6f} msgs/sec"
        )

    # initiate logging
    zenoh.init_log_from_env_or("error")

    with zenoh.open(conf) as session:
        session.declare_subscriber(
            "test/thr", zenoh.handlers.Callback(listener, report)
        )

        print("Press CTRL-C to quit...")
        while True:
            time.sleep(1)


if __name__ == "__main__":
    # --- Command line argument parsing --- --- --- --- --- ---
    import argparse
    import json

    import common

    parser = argparse.ArgumentParser(
        prog="z_sub_thr", description="zenoh throughput sub example"
    )
    common.add_config_arguments(parser)
    parser.add_argument(
        "--number",
        "-n",
        dest="number",
        default=50000,
        metavar="NUMBER",
        type=int,
        help="Number of messages in each throughput measurements.",
    )

    args = parser.parse_args()
    conf = common.get_config_from_args(args)

    main(conf, args.number)
