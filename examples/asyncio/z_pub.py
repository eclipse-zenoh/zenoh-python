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

import asyncio
import sys
import time
import argparse
import itertools
import json
import zenoh
from zenoh import config


async def main():
    # --- Command line argument parsing --- --- --- --- --- ---
    parser = argparse.ArgumentParser(
        prog='z_pub',
        description='zenoh pub example')
    parser.add_argument('--mode', '-m', dest='mode',
                        choices=['peer', 'client'],
                        type=str,
                        help='The zenoh session mode.')
    parser.add_argument('--connect', '-e', dest='connect',
                        metavar='ENDPOINT',
                        action='append',
                        type=str,
                        help='Endpoints to connect to.')
    parser.add_argument('--listen', '-l', dest='listen',
                        metavar='ENDPOINT',
                        action='append',
                        type=str,
                        help='Endpoints to listen on.')
    parser.add_argument('--key', '-k', dest='key',
                        default='/demo/example/zenoh-python-pub',
                        type=str,
                        help='The key expression to publish onto.')
    parser.add_argument('--value', '-v', dest='value',
                        default='Pub from Python!',
                        type=str,
                        help='The value to publish.')
    parser.add_argument("--iter", dest="iter", type=int,
                        help="How many puts to perform")
    parser.add_argument('--config', '-c', dest='config',
                        metavar='FILE',
                        type=str,
                        help='A configuration file.')

    args = parser.parse_args()
    conf = zenoh.config_from_file(
        args.config) if args.config is not None else zenoh.Config()
    if args.mode is not None:
        conf.insert_json5(zenoh.config.MODE_KEY, json.dumps(args.mode))
    if args.connect is not None:
        conf.insert_json5(zenoh.config.CONNECT_KEY, json.dumps(args.connect))
    if args.listen is not None:
        conf.insert_json5(zenoh.config.LISTEN_KEY, json.dumps(args.listen))
    key = args.key
    value = args.value

    # zenoh-net code  --- --- --- --- --- --- --- --- --- --- ---

    # initiate logging
    zenoh.init_logger()

    print("Openning session...")
    session = await zenoh.async_open(conf)

    print("Declaring key expression '{}'...".format(key), end='')
    rid = await session.declare_expr(key)
    print(" => RId {}".format(rid))

    print("Declaring publication on '{}'...".format(rid))
    await session.declare_publication(rid)

    for idx in itertools.count() if args.iter is None else range(args.iter):
        time.sleep(1)
        buf = "[{:4d}] {}".format(idx, value)
        print("Putting Data ('{}': '{}')...".format(rid, buf))
        await session.put(rid, bytes(buf, encoding='utf8'))

    await session.undeclare_publication(rid)
    await session.undeclare_expr(rid)
    await session.close()

asyncio.run(main())
