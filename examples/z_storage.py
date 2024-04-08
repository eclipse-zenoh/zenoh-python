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
import argparse
import json
import zenoh
from zenoh.prelude import *
from zenoh.queryable import Query

# --- Command line argument parsing --- --- --- --- --- ---
parser = argparse.ArgumentParser(prog="z_storage", description="zenoh storage example")
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
    help="The key expression matching resources to store.",
)
parser.add_argument(
    "--complete",
    dest="complete",
    default=False,
    action="store_true",
    help="Declare the storage as complete w.r.t. the key expression.",
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
    zenoh.config.Config.from_file(args.config)
    if args.config is not None
    else zenoh.config.default()
)
if args.mode is not None:
    conf.insert_json5("mode", json.dumps(args.mode))
if args.connect is not None:
    conf.insert_json5("connect/endpoints", json.dumps(args.connect))
if args.listen is not None:
    conf.insert_json5("listen/endpoints", json.dumps(args.listen))
key = args.key
complete = args.complete

# Zenoh code  --- --- --- --- --- --- --- --- --- --- ---

store = {}


def listener(sample: Sample):
    print(
        ">> [Subscriber] Received {} ('{}': '{}')".format(
            sample.kind, sample.key_expr, sample.payload_as(str)
        )
    )
    if sample.kind == SampleKind.DELETE:
        store.pop(sample.key_expr, None)
    else:
        store[sample.key_expr] = sample


def query_handler(query: Query):
    print(">> [Queryable ] Received Query '{}'".format(query.selector))
    replies = []
    for stored_name, sample in store.items():
        if query.key_expr.intersects(stored_name):
            query.reply(
                sample.key_expr,
                sample.payload,
                encoding=sample.encoding,
                congestion_control=sample.qos.congestion_control,
                priority=sample.qos.priority,
                express=sample.qos.express,
            )


def main():
    # initiate logging
    zenoh.init_logger()

    print("Opening session...")
    session = zenoh.open(conf)

    print("Declaring Subscriber on '{}'...".format(key))
    sub = session.declare_subscriber(
        key, handler=listener, reliability=Reliability.RELIABLE
    )

    print("Declaring Queryable on '{}'...".format(key))
    queryable = session.declare_queryable(key, handler=query_handler, complete=complete)

    print("Press CTRL-C to quit...")
    while True:
        time.sleep(1)

    # Cleanup: note that even if you forget it, cleanup will happen automatically when
    # the reference counter reaches 0
    # sub.undeclare()
    # queryable.undeclare()
    # session.close()


main()
