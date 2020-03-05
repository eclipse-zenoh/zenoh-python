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

import re
import json
import zenoh.net
from zenoh.core import ZException
from zenoh.encoding import Encoding


class Value(object):
    '''
    Associated to a path, a Value can be published into zenoh
    via :func:`Workspace.put`, or retrieved via :func:`Workspace.get` or
    via a subscription (see :func:`Workspace.subscribe`).
    '''

    def __init__(self, value, encoding=Encoding.RAW, raw_format=""):
        if encoding is None:
            encoding = Encoding.RAW
        self.encoding = encoding
        if self.encoding == Encoding.JSON:
            if not (isinstance(value, dict) or isinstance(value, str)):
                raise ZException("Value is not a valid JSON")
            self.value = json.dumps(value)
        elif self.encoding == Encoding.RAW and isinstance(value, str):
            self.value = value.encode()
        else:
            self.value = value
        self.raw_format = raw_format

    def as_zn_payload(self):
        if self.encoding == Encoding.RAW:
            return bytes(self.value)
        if self.encoding == Encoding.PROPERTIES:
            s = ';'.join(map('='.join, map(list, self.value.items())))
            return s.encode()
        return self.value.encode()

    def get_encoding(self):
        '''
        Return the :class:`Encoding` of this value.
        '''
        return self.encoding

    def get_value(self):
        '''
        Return the content of the value.
        '''
        if self.encoding is Encoding.JSON:
            return json.loads(self.value)
        return self.value

    def __eq__(self, second_value):
        if isinstance(second_value, self.__class__):
            return self.value == second_value.value
        return False

    def __str__(self):
        return str(self.value)

    def __repr__(self):
        return self.__str__()

    @staticmethod
    def from_zn_resource(buf, info):
        encoding = Encoding(info.encoding)
        data = None
        if(encoding == Encoding.RAW):
            data = bytearray(buf)
        else:
            data = buf.decode()
        return Value(data, encoding)
