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

import argparse
import json
import time

import zenoh

# --- Command line argument parsing --- --- --- --- --- ---
parser = argparse.ArgumentParser(prog="z_get", description="zenoh get example")
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
    "--config",
    "-c",
    dest="config",
    metavar="FILE",
    type=str,
    help="A configuration file.",
)
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


# Zenoh code  --- --- --- --- --- --- --- --- --- --- ---
def main():
    # initiate logging
    zenoh.init_logger()

    print("Opening session...")
    session = zenoh.open(conf)

    sub = session.declare_subscriber("test/pong", zenoh.Queue())
    pub = session.declare_publisher(
        "test/ping",
        congestion_control=zenoh.CongestionControl.BLOCK(),
    )
    data = bytes(i % 10 for i in range(0, args.payload_size))

    print(f"Warming up for {args.warmup}...")
    warmup_end = time.time() + args.warmup
    while time.time() < warmup_end:
        pub.put(data)
        sub.receiver.get()

    samples = []
    for i in range(args.samples):
        write_time = time.time()
        pub.put(data)
        sub.receiver.get()
        samples.append(round((time.time() - write_time) * 1_000_000))

    for i, rtt in enumerate(samples):
        print(f"{args.payload_size} bytes: seq={i} rtt={rtt}µs lat={rtt / 2}µs")

    pub.undeclare()
    sub.undeclare()
    session.close()


main()
