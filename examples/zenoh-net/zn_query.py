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
from zenoh.net import (
    Session, QueryDest,
    ZN_STORAGE_DATA, ZN_STORAGE_FINAL,
    ZN_EVAL_DATA, ZN_EVAL_FINAL
)

### --- Command line argument parsing --- --- --- --- --- --- 
parser = argparse.ArgumentParser(prog='zn_query', description='Issues a query for a selector specified by the command-line')

parser.add_argument('--selector', '-s', dest='selector',
                    default='/zenoh/examples/**',
                    type=str,
                    help='The selector to be used for issuing the query')

parser.add_argument('--locator', '-l', dest='locator',
                    default=None,
                    type=str,
                    help='The locator to be used to boostrap the zenoh session. By default dynamic discovery is used')


args = parser.parse_args()
selector = args.selector
locator = args.locator

### zenoh-net code  --- --- --- --- --- --- --- --- --- --- --- 

def reply_handler(reply):
    if reply.kind == ZN_STORAGE_DATA:
        print(">> [Reply handler] received -Storage Data- ('{}': '{}')"
              .format(reply.rname, reply.data.decode("utf-8")))
    elif reply.kind == ZN_EVAL_DATA:
        print(">> [Reply handler] received -Eval Data-    ('{}': '{}')"
              .format(reply.rname, reply.data.decode("utf-8")))
    elif reply.kind == ZN_STORAGE_FINAL:
        print(">> [Reply handler] received -Storage Final-")
    elif reply.kind == ZN_EVAL_FINAL:
        print(">> [Reply handler] received -Eval Final-")
    else:
        print(">> [Reply handler] received -Reply Final-")


print("Openning session...")
s = Session.open(locator)

print("Sending query '{}'...".format(selector))
s.query(selector, "", reply_handler,
        dest_storages=QueryDest(QueryDest.ZN_ALL),
        dest_evals=QueryDest(QueryDest.ZN_ALL))

time.sleep(1)

s.close()
