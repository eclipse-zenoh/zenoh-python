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


def main(
    conf: zenoh.Config,
    selector: str,
    target: zenoh.QueryTarget,
    payload: str,
    timeout: float,
):
    # initiate logging
    zenoh.init_log_from_env_or("error")

    print("Opening session...")
    with zenoh.open(conf) as session:
        print(f"Sending Query '{selector}'...")
        replies = session.get(selector, target=target, payload=payload, timeout=timeout)
        for reply in replies:
            try:
                print(
                    f">> Received ('{reply.ok.key_expr}': '{reply.ok.payload.to_string()}')"
                )
            except:
                print(f">> Received (ERROR: '{reply.err.payload.to_string()}')")


if __name__ == "__main__":
    # --- Command line argument parsing --- --- --- --- --- ---
    import argparse
    import json

    import common

    parser = argparse.ArgumentParser(prog="z_get", description="zenoh get example")
    common.add_config_arguments(parser)
    parser.add_argument(
        "--selector",
        "-s",
        dest="selector",
        default="demo/example/**",
        type=str,
        help="The selection of resources to query.",
    )
    parser.add_argument(
        "--target",
        "-t",
        dest="target",
        choices=["ALL", "BEST_MATCHING", "ALL_COMPLETE", "NONE"],
        default="BEST_MATCHING",
        type=str,
        help="The target queryables of the query.",
    )
    parser.add_argument(
        "--payload",
        "-p",
        dest="payload",
        type=str,
        help="An optional payload to send in the query.",
    )
    parser.add_argument(
        "--timeout",
        "-o",
        dest="timeout",
        default=10.0,
        type=float,
        help="The query timeout",
    )

    args = parser.parse_args()
    conf = common.get_config_from_args(args)

    target = {
        "ALL": zenoh.QueryTarget.ALL,
        "BEST_MATCHING": zenoh.QueryTarget.BEST_MATCHING,
        "ALL_COMPLETE": zenoh.QueryTarget.ALL_COMPLETE,
    }.get(args.target)

    main(conf, args.selector, target, args.payload, args.timeout)
