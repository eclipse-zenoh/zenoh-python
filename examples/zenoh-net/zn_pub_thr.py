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
from zenoh.net import Session

if __name__ == '__main__':
    locator = None
    if len(sys.argv) < 2:
        print('USAGE:\n\tzn_pub_thr <payload-size> [<zenoh-locator>]\n\n')
        sys.exit(-1)
    size = int(sys.argv[1])
    print("Running throughput test for payload of {} bytes".format(size))
    if len(sys.argv) > 2:
        locator = sys.argv[2]

    s = Session.open(locator)
    pub = s.declare_publisher('/test/thr')

    bs = bytearray()
    for i in range(0, size):
        bs.append(i % 10)

    while True:
        s.stream_data(pub, bytes(bs))

    s.close()
