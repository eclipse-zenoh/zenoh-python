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
import argparse
import concurrent.futures
from zenoh import Zenoh, Selector, Path, Workspace
from zenoh import Change, ChangeKind, Encoding, Value


### --- Command line argument parsing --- --- --- --- --- --- 
parser = argparse.ArgumentParser(prog='z_eval', description='Shows how to use zenoh evaluated/computed resources')
parser.add_argument('--path', '-p', dest='path',
                    default='/zenoh/examples/python/eval',
                    type=str,
                    help='the path representing the  URI')

parser.add_argument('--locator', '-l', dest='locator',
                    default=None,
                    type=str,
                    help='The locator to be used to boostrap the zenoh session. By default dynamic discovery is used')

args = parser.parse_args()

path = args.path
locator = args.locator

### zenoh code  --- --- --- --- --- --- --- --- --- --- --- 

print('Login to Zenoh...')
z = Zenoh.login(locator)

# Note that we give a ThreadPool to the workspace here, for our eval_callback
# below to be called in a separate thread rather that in Zenoh I/O thread.
# Thus, the callback can perform some Zenoh operations (e.g.: get)
w = z.workspace(executor = concurrent.futures.ThreadPoolExecutor())


def eval_callback(path, properties):
    # In this Eval function, we choosed to get the name to be returned in the
    # StringValue in 3 possible ways, depending the properties specified in the
    # selector. For example, with the following selectors:
    #   - '/demo/example/zenoh-python-eval' :
    #         no properties are set, a default value is used for the name
    #   - '/demo/example/zenoh-python-eval?(name=Bob)' :
    #         'Bob' is used for the name
    #   - '/demo/example/zenoh-python-eval?(name=/demo/example/name)' :
    #         the Eval function does a GET on '/demo/example/name' and uses the
    #         1st result for the name
    print('>> Processing eval for path {} with properties: {}'
          .format(path, properties))
    # name = properties['name']
    name = properties.get('name', 'Zenoh Python!')

    if name.startswith('/'):
        print('   >> Get name to use from Zenoh at path: {}'.format(name))
        dataset = w.get(name)
        if len(dataset) > 0:
            name = dataset[0].value

    print('   >> Returning string: "Eval from {}"'.format(name))
    return Value('Eval from {}'.format(name), encoding=Encoding.STRING)


print('Register eval {}'.format(path))
w.register_eval(path, eval_callback)

print('Enter \'q\' to quit...')
while sys.stdin.read(1) != 'q':
    pass

w.unregister_eval(path)
z.logout()
