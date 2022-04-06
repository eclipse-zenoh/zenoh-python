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
from zenoh import config, Sample
from zenoh.queryable import EVAL


async def main():
    # --- Command line argument parsing --- --- --- --- --- ---
    parser = argparse.ArgumentParser(
        prog='z_eval',
        description='zenoh eval example')
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
                        default='/demo/example/zenoh-python-eval',
                        type=str,
                        help='The key expression matching queries to evaluate.')
    parser.add_argument('--value', '-v', dest='value',
                        default='Eval from Python!',
                        type=str,
                        help='The value to reply to queries.')
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

    # Note: As an example the concrete implementation of the eval callback is implemented here as a coroutine.
    #       It checks if the query's value_selector (the substring after '?') is a float, and if yes, sleeps for this number of seconds.
    #       Run example/asyncio/z_get_parallel.py example to see how 3 concurrent get() are executed in parallel in this z_eval.py
    async def eval_corouting(query):
        selector = query.selector
        try:
            sleep_time = selector.parse_value_selector().properties.get('sleep')
            if sleep_time is not None:
                print("  Sleeping {} secs before replying".format(
                    float(sleep_time)))
                await asyncio.sleep(float(sleep_time))
        except Exception as e:
            print("  WARN: error in value selector: {}. Ignore it.".format(e))
        print("  Replying to query on {}".format(selector))
        reply = "{} (this is the reply to query on {})".format(value, selector)
        query.reply(Sample(key_expr=key, payload=reply.encode()))

    async def eval_callback(query):
        print(">> [Queryable ] Received Query '{}'".format(query.selector))
        # schedule a task that will call eval_corouting(query)
        asyncio.create_task(eval_corouting(query))

    # initiate logging
    zenoh.init_logger()

    print("Openning session...")
    session = await zenoh.async_open(conf)

    print("Creating Queryable on '{}'...".format(key))
    queryable = await session.queryable(key, eval_callback, kind=EVAL)

    print("Enter 'q' to quit......")
    c = '\0'
    while c != 'q':
        c = sys.stdin.read(1)
        if c == '':
            time.sleep(1)

    await queryable.close()
    await session.close()

asyncio.run(main())
