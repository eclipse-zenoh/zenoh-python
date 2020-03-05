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

import unittest
from zenoh import Value, Change, ChangeKind
from zenoh.encoding import *
from zenoh.core import ZException


class ValueTests(unittest.TestCase):

    def test_raw_value_str(self):
        v = Value('test raw value')
        self.assertEqual('test raw value'.encode(), v.get_value())
        self.assertEqual(Encoding.RAW, v.encoding)

    def test_raw_value_bytes(self):
        iv = 'test raw value bytes'.encode()
        v = Value(iv)
        self.assertEqual(iv, v.get_value())
        self.assertEqual(Encoding.RAW, v.encoding)

    def test_string_value(self):
        v = Value('test string value', encoding=Encoding.STRING)
        self.assertEqual('test string value', v.get_value())
        self.assertEqual(Encoding.STRING, v.encoding)
        self.assertEquals(Encoding.STRING, v.get_encoding())

    def test_json_value(self):
        d = {
            'this': 'is',
            'a': 'json value'
        }
        v = Value(d, encoding=Encoding.JSON)
        self.assertEqual(d, v.get_value())
        self.assertEqual(Encoding.JSON, v.encoding)

    def test_not_valid_json(self):
        nvj = ['hello!']
        self.assertRaises(ZException, Value, nvj, Encoding.JSON)

    def test_equal(self):
        v1 = Value('test string value', encoding=Encoding.STRING)
        v2 = Value('test string value', encoding=Encoding.STRING)
        self.assertEqual(v1, v2)

    def test_repr(self):
        v1 = Value('test string value', encoding=Encoding.STRING)
        v2 = 'test string value'
        self.assertEqual(repr(v1), v2)

    def test_not_equal(self):
        v1 = Value('test string value', encoding=Encoding.STRING)
        v2 = 'test string value'
        self.assertNotEqual(v1, v2)

    def test_change(self):
        v1 = Value('test string value', encoding=Encoding.STRING)
        c = Change('/test', ChangeKind.PUT, None, v1)
        self.assertEqual('/test', c.get_path())
        self.assertEqual(ChangeKind.PUT, c.get_kind())
        self.assertEqual(v1, c.get_value())
        self.assertEqual(None, c.get_timestamp())
