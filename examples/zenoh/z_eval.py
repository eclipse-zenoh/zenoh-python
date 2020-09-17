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
from zenoh.net import Config

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
config = Config(
    mode=Config.parse_mode(args.mode),
    peers=args.peer,
    listeners=args.listener)
path = args.path

# zenoh-net code  --- --- --- --- --- --- --- --- --- --- ---

# initiate logging
zenoh.init_logger()

print("Openning session...")
zenoh = Zenoh(config)

print("New workspace...")
workspace = zenoh.workspace()


def eval_callback(get_request):
    print(">> [Eval listener] received get with selector: {}".format(
        get_request.selector))
    name = 'Python!'
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
