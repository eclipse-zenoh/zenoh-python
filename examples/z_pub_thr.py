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
import zenoh


def main(conf: zenoh.Config, payload_size: int):
    # initiate logging
    zenoh.init_log_from_env_or("error")

    data = bytearray()
    for i in range(0, payload_size):
        data.append(i % 10)
    data = zenoh.ZBytes(data)
    congestion_control = zenoh.CongestionControl.BLOCK

    with zenoh.open(conf) as session:
        pub = session.declare_publisher(
            "test/thr", congestion_control=congestion_control
        )

        print("Press CTRL-C to quit...")
        while True:
            pub.put(data)


# --- Command line argument parsing --- --- --- --- --- ---
if __name__ == "__main__":
    import argparse
    import json

    import common

    parser = argparse.ArgumentParser(
        prog="z_pub_thr", description="zenoh throughput pub example"
    )
    common.add_config_arguments(parser)
    parser.add_argument(
        "payload_size", type=int, help="Sets the size of the payload to publish."
    )

    args = parser.parse_args()
    conf = common.get_config_from_args(args)

    main(conf, args.payload_size)
