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


def main(conf: zenoh.Config, key: str, history: bool):
    # initiate logging
    zenoh.init_log_from_env_or("error")

    print("Opening session...")
    with zenoh.open(conf) as session:
        print(f"Declaring Liveliness Subscriber on '{key}'...")
        with session.liveliness().declare_subscriber(key, history=history) as sub:
            for sample in sub:
                if sample.kind == zenoh.SampleKind.PUT:
                    print(
                        f">> [LivelinessSubscriber] New alive token ('{sample.key_expr}')"
                    )
                elif sample.kind == zenoh.SampleKind.DELETE:
                    print(
                        f">> [LivelinessSubscriber] Dropped token ('{sample.key_expr}')"
                    )


# --- Command line argument parsing --- --- --- --- --- ---
if __name__ == "__main__":
    import argparse

    import common

    parser = argparse.ArgumentParser(
        prog="z_sub_liveliness", description="zenoh sub example"
    )
    common.add_config_arguments(parser)
    parser.add_argument(
        "--key",
        "-k",
        dest="key",
        default="group1/**",
        type=str,
        help="The key expression to subscribe to.",
    )
    parser.add_argument(
        "--history",
        dest="history",
        default=False,
        type=bool,
        help="Get historical liveliness tokens.",
    )

    args = parser.parse_args()
    conf = common.get_config_from_args(args)

    main(conf, args.key, args.history)
