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


def main(conf: zenoh.Config, key: str, payload: str):
    # initiate logging
    zenoh.init_log_from_env_or("error")

    print("Opening session...")
    with zenoh.open(conf) as session:

        print(f"Putting Data ('{key}': '{payload}')...")
        # Refer to z_bytes.py to see how to serialize different types of message
        session.put(key, payload)

        # --- Examples of put with other types:

        # - Integer
        # session.put('demo/example/Integer', 3)

        # - Float
        # session.put('demo/example/Float', 3.14)

        # - Properties (as a Dictionary with str only)
        # session.put('demo/example/Properties', {'p1': 'v1', 'p2': 'v2'}

        # - Json (str format)
        # session.put('demo/example/Json',
        #             json.dumps(['foo', {'bar': ('baz', None, 1.0, 2)}]).encode(),
        #             encoding=zenoh.Encoding.TEXT_JSON))

        # - Raw ('zenoh/bytes' encoding by default)
        # session.put('demo/example/Raw', b'\x48\x69\x21')

        # - Custom encoding
        # session.put('demo/example/Custom',
        #             b'\x48\x69\x21',
        #             encoding='my_encoding')

        # - UTF-16 String specifying the charset as Encoding schema
        # session.put('demo/example/UTF-16',
        #             'hello'.encode('utf-16'),
        #             encoding=zenoh.Encoding.TEXT_PLAIN.with_schema(';charset=utf-16'))


# --- Command line argument parsing --- --- --- --- --- ---
if __name__ == "__main__":
    import argparse
    import json

    parser = argparse.ArgumentParser(prog="z_put", description="zenoh put example")
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
        default="demo/example/zenoh-python-put",
        type=str,
        help="The key expression to write.",
    )
    parser.add_argument(
        "--payload",
        "-p",
        dest="payload",
        default="Put from Python!",
        type=str,
        help="The payload to write.",
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
        zenoh.Config.from_file(args.config)
        if args.config is not None
        else zenoh.Config()
    )
    if args.mode is not None:
        conf.insert_json5("mode", json.dumps(args.mode))
    if args.connect is not None:
        conf.insert_json5("connect/endpoints", json.dumps(args.connect))
    if args.listen is not None:
        conf.insert_json5("listen/endpoints", json.dumps(args.listen))

    main(conf, args.key, args.payload)
