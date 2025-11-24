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
from typing import Optional

import zenoh
from zenoh.ext import CacheConfig, MissDetectionConfig, declare_advanced_publisher


def main(conf: zenoh.Config, key: str, payload: str, history: int):
    # initiate logging
    zenoh.init_log_from_env_or("error")

    print("Opening session...")
    with zenoh.open(conf) as session:
        print(f"Declaring AdvancedPublisher on '{key}'...")
        pub = declare_advanced_publisher(
            session,
            key,
            cache=CacheConfig(max_samples=history),
            sample_miss_detection=MissDetectionConfig(heartbeat=5),
            publisher_detection=True,
        )

        print("Press CTRL-C to quit...")
        for idx in itertools.count():
            time.sleep(1)
            buf = f"[{idx:4d}] {payload}"
            print(f"Putting Data ('{key}': '{buf}')...")
            pub.put(buf)


# --- Command line argument parsing --- --- --- --- --- ---
if __name__ == "__main__":
    import argparse
    import itertools

    import common

    parser = argparse.ArgumentParser(
        prog="z_advanced_pub", description="zenoh advanced pub example"
    )
    common.add_config_arguments(parser)
    parser.add_argument(
        "--key",
        "-k",
        dest="key",
        default="demo/example/zenoh-python-pub",
        type=str,
        help="The key expression to publish onto.",
    )
    parser.add_argument(
        "--payload",
        "-p",
        dest="payload",
        default="Pub from Python!",
        type=str,
        help="The payload to publish.",
    )
    parser.add_argument(
        "--history",
        dest="history",
        type=int,
        default=1,
        help="The number of publications to keep in cache",
    )

    args = parser.parse_args()
    conf = common.get_config_from_args(args)

    main(conf, args.key, args.payload, args.history)
