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

store = {}


def listener(sample: zenoh.Sample):
    print(
        f">> [Subscriber] Received {sample.kind} ('{sample.key_expr}': '{sample.payload.to_string()}')"
    )
    if sample.kind == zenoh.SampleKind.DELETE:
        store.pop(sample.key_expr, None)
    else:
        store[sample.key_expr] = sample


def query_handler(query: zenoh.Query):
    print(f">> [Queryable ] Received Query '{query.selector}'")
    for stored_name, sample in store.items():
        if query.key_expr.intersects(stored_name):
            query.reply(
                sample.key_expr,
                sample.payload,
                encoding=sample.encoding,
                congestion_control=sample.congestion_control,
                priority=sample.priority,
                express=sample.express,
            )


def main(conf: zenoh.Config, key: str, complete: bool):
    # initiate logging
    zenoh.init_log_from_env_or("error")

    print("Opening session...")
    with zenoh.open(conf) as session:
        print(f"Declaring Subscriber on '{key}'...")
        session.declare_subscriber(key, listener)

        print(f"Declaring Queryable on '{key}'...")
        session.declare_queryable(key, query_handler, complete=complete)

        print("Press CTRL-C to quit...")
        while True:
            time.sleep(1)


# --- Command line argument parsing --- --- --- --- --- ---
if __name__ == "__main__":
    import argparse
    import json

    import common

    parser = argparse.ArgumentParser(
        prog="z_storage", description="zenoh storage example"
    )
    common.add_config_arguments(parser)
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

    args = parser.parse_args()
    conf = common.get_config_from_args(args)

    main(conf, args.key, args.complete)
