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

import argparse
import json
import time

import zenoh

# --- Command line argument parsing --- --- --- --- --- ---
parser = argparse.ArgumentParser(
    prog="z_sub_thr", description="zenoh throughput sub example"
)
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
    "--number",
    "-n",
    dest="number",
    default=50000,
    metavar="NUMBER",
    action="append",
    type=int,
    help="Number of messages in each throughput measurements.",
)
parser.add_argument(
    "--config",
    "-c",
    dest="config",
    metavar="FILE",
    type=str,
    help="A configuration file.",
)

args = parser.parse_args()
conf = (
    zenoh.Config.from_file(args.config) if args.config is not None else zenoh.Config()
)
if args.mode is not None:
    conf.insert_json5("mode", json.dumps(args.mode))
if args.connect is not None:
    conf.insert_json5("connect/endpoints", json.dumps(args.connect))
if args.listen is not None:
    conf.insert_json5("listen/endpoints", json.dumps(args.listen))
n = args.number


batch_count = 0
count = 0
start = None
global_start = None


def listener(sample):
    global n, count, batch_count, start, global_start
    if count == 0:
        start = time.time()
        if global_start is None:
            global_start = start
        count += 1
    elif count < n:
        count += 1
    else:
        stop = time.time()
        print(f"{n / (stop - start):.6f} msgs/sec")
        batch_count += 1
        count = 0


def report():
    global n, m, count, batch_count, global_start
    end = time.time()
    total = batch_count * n + count
    print(
        f"Received {total} messages in {end - global_start}: averaged {total / (end - global_start):.6f} msgs/sec"
    )


def main():
    # initiate logging
    zenoh.try_init_log_from_env()

    with zenoh.open(conf) as session:
        session.declare_subscriber(
            "test/thr",
            zenoh.handlers.Callback(listener, report),
            reliability=zenoh.Reliability.RELIABLE,
        )

        print("Press CTRL-C to quit...")
        while True:
            time.sleep(1)


if __name__ == "__main__":
    main()
