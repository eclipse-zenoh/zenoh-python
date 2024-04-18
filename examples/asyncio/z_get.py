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
import json
import zenoh
from zenoh import config, QueryTarget


async def main():
    # --- Command line argument parsing --- --- --- --- --- ---
    parser = argparse.ArgumentParser(
        prog='z_get',
        description='zenoh get example')
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
    parser.add_argument('--selector', '-s', dest='selector',
                        default='/demo/example/**',
                        type=str,
                        help='The selection of resources to query.')
    parser.add_argument('--target', '-t', dest='target',
                        choices=['ALL', 'BEST_MATCHING',
                                 'ALL_COMPLETE', 'NONE'],
                        default='ALL',
                        type=str,
                        help='The target queryables of the query.')
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
    selector = args.selector
    target = {
        'ALL': QueryTarget.All(),
        'BEST_MATCHING': QueryTarget.BestMatching(),
        'ALL_COMPLETE': QueryTarget.AllComplete(),
        'NONE': QueryTarget.No()}.get(args.target)

    # zenoh-net code  --- --- --- --- --- --- --- --- --- --- ---

    # initiate logging
    zenoh.init_logger()

    print("Openning session...")
    session = await zenoh.async_open(conf)

    print("Sending Query '{}'...".format(selector))
    replies = await session.get(selector, target=target)
    for reply in replies:
        if isinstance(reply.sample, zenoh.Sample):
            print(">> Received ('{}': '{}')"
                .format(reply.sample.key_expr, reply.sample.payload.decode("utf-8")))
        else: 
            print(">> Received (ERROR: '{}')"
                .format(reply.sample.payload.decode("utf-8")))

    await session.close()

asyncio.run(main())
