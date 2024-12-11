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
from zenoh.ext import z_serialize


def main(conf: zenoh.Config, key: str, payload: float):
    # initiate logging
    zenoh.init_log_from_env_or("error")

    print("Opening session...")
    with zenoh.open(conf) as session:

        print(f"Putting Data ('{key}': '{payload}')...")
        # Refer to z_bytes.py to see how to serialize different types of message
        session.put(key, z_serialize(payload))


# --- Command line argument parsing --- --- --- --- --- ---
if __name__ == "__main__":
    import argparse
    import json

    import common

    parser = argparse.ArgumentParser(
        prog="z_put_float", description="zenoh put example"
    )
    common.add_config_arguments(parser)
    parser.add_argument(
        "--key",
        "-k",
        dest="key",
        default="demo/example/zenoh-python-put",
        type=str,
        help="The key expression to write.",
    )
    parser.add_argument(
        "--payload",
        "-p",
        dest="payload",
        default=42.0,
        type=float,
        help="The payload to write.",
    )

    args = parser.parse_args()
    conf = common.get_config_from_args(args)

    main(conf, args.key, args.payload)
