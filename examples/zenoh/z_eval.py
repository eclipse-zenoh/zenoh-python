# Copyright (c) 2017, 2020 ADLINK Technology Inc.
#
# This program and the accompanying materials are made available under the
# terms of the Eclipse Public License 2.0 which is available at
# http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
# which is available at https://www.apache.org/licenses/LICENSE-2.0.
#
# SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
#
# Contributors:
#   ADLINK zenoh team, <zenoh@adlink-labs.tech>

import sys
import time
import argparse
import zenoh
from zenoh import Zenoh

# --- Command line argument parsing --- --- --- --- --- ---
parser = argparse.ArgumentParser(
    prog='z_eval',
    description='zenoh eval example')
parser.add_argument('--mode', '-m', dest='mode',
                    default='peer',
                    choices=['peer', 'client'],
                    type=str,
                    help='The zenoh session mode.')
parser.add_argument('--peer', '-e', dest='peer',
                    metavar='LOCATOR',
                    action='append',
                    type=str,
                    help='Peer locators used to initiate the zenoh session.')
parser.add_argument('--listener', '-l', dest='listener',
                    metavar='LOCATOR',
                    action='append',
                    type=str,
                    help='Locators to listen on.')
parser.add_argument('--path', '-p', dest='path',
                    default='/demo/example/eval',
                    type=str,
                    help='The path the eval will respond for.')

args = parser.parse_args()
conf = { "mode": args.mode }
if args.peer is not None:
    conf["peer"] = ",".join(args.peer)
if args.listener is not None:
    conf["listener"] = ",".join(args.listener)
path = args.path

# zenoh-net code  --- --- --- --- --- --- --- --- --- --- ---

# initiate logging
zenoh.init_logger()

print("Openning session...")
zenoh = Zenoh(conf)

print("New workspace...")
workspace = zenoh.workspace()


def eval_callback(get_request):
    print(">> [Eval listener] received get with selector: {}".format(
        get_request.selector))

    # The returned Value is a StringValue with a 'name' part which is set in 3 possible ways,
    # depending the properties specified in the selector. For example, with the
    # following selectors:
    # - "/zenoh/example/eval" : no properties are set, a default value is used for the name
    # - "/zenoh/example/eval?(name=Bob)" : "Bob" is used for the name
    # - "/zenoh/example/eval?(name=/zenoh/example/name)" : the Eval function does a GET
    #      on "/zenoh/example/name" an uses the 1st result for the name
    # properties.get('name', 'Zenoh Python!')
    name = get_request.selector.properties.get('name', 'Python!')
    if name.startswith('/'):
        print('   >> Get name to use from Zenoh at path: {}'.format(name))
        dataset = workspace.get(name)
        print('   >> get result: {}'.format(dataset))
        if len(dataset) > 0:
            name = dataset[0].value.get_content()

    print('   >> Replying string: "Eval from {}"'.format(name))
    get_request.reply(path, 'Eval from {}'.format(name))


print("Register eval for '{}'...".format(path))
eval = workspace.register_eval(path, eval_callback)

print("Press q to stop...")
c = '\0'
while c != 'q':
    c = sys.stdin.read(1)

eval.close()
zenoh.close()
