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
from zenoh import Selector
from zenoh import Path


class SelectorTests(unittest.TestCase):

    def test_selector_simple(self):
        s = Selector('/this/is/a/selector')
        self.assertEqual('/this/is/a/selector', s.to_string())

    def test_selector_with_predicate(self):
        s = Selector('/this/is/a/selector?x>10')
        self.assertEqual(Path('/this/is/a/selector'), s.get_prefix())
        self.assertEqual('x>10', s.get_predicate())

    def test_selector_with_fragment(self):
        s = Selector('/this/is/a/selector#field')
        self.assertEqual(Path('/this/is/a/selector'), s.get_prefix())
        self.assertEqual('field', s.get_fragment())

    def test_selector_complete(self):
        s = Selector('/this/is/a/**?x>10(x.y.z=100)#field')
        self.assertEqual(Path('/this/is/a/'), s.get_prefix())
        self.assertEqual('field', s.get_fragment())
        self.assertEqual('x.y.z=100', s.get_properties())
        self.assertEqual('x>10', s.get_predicate())

    def test_selector_is_path_unique(self):
        s = Selector('/this/is/a/**?x>10(x.y.z=100)#field')
        self.assertFalse(s.is_path_unique())
        s = Selector('/this/is/a/selector?x>10(x.y.z=100)#field')
        self.assertTrue(s.is_path_unique())

    def test_selector_check_absolute_ok(self):
        s = Selector('/this/is/a/absolute/selector/*')
        self.assertTrue(s.is_absolute())

    def test_selector_check_absolute_ko(self):
        s = Selector('this/is/a/relative/selector/*')
        self.assertFalse(s.is_absolute())

    def test_selector_prefix(self):
        s = Selector('/this/is/a/selector/with/a/prefix')
        self.assertTrue(s.is_prefixed_by_path('/this/is/a/selector'))
        self.assertFalse(s.is_prefixed_by_path('/that/is/a/selector'))

    def test_selector_equal(self):
        s1 = Selector('/this/is/a/**?x>10(x.y.z=100)#field')
        s2 = Selector('/this/is/a/**?x>10(x.y.z=100)#field')
        self.assertEqual(s1, s2)

    def test_selector_properties_dict(self):
        s1 = Selector('/this/is/a/**?x>10(x=100;y.z=1)#field')
        d = {'x': '100', 'y': {'z': '1'}}
        self.assertEqual(s1.dict_from_properties(), d)

    def test_selector_properties_dic_emptyt(self):
        s1 = Selector('/this/is/a/**?x>10')
        d = {}
        self.assertEqual(s1.dict_from_properties(), d)

    def test_to_selector(self):
        s = '/test/**'
        s2 = Selector('/test/**')
        self.assertEqual(Selector.to_selector(s), Selector('/test/**'))
        self.assertEqual(Selector.to_selector(s2), Selector('/test/**'))

    def test_selector_not_equal(self):
        s1 = Selector('/this/is/a/**?x>10(x.y.z=100)#field')
        s2 = Path('/this/is/a/path')
        self.assertNotEqual(s1, s2)

    def test_selector_str(self):
        s1 = Selector('/this/is/a/selector')
        self.assertEqual(str(s1), '/this/is/a/selector')

    def test_selector_repr(self):
        s1 = Selector('/this/is/a/selector')
        self.assertEqual(repr(s1), '/this/is/a/selector')

    def test_selector_len(self):
        s = '/this/is/a/selector'
        s1 = Selector('/this/is/a/selector')
        self.assertEqual(len(s), len(s1))

    def test_selector_hash(self):
        s = '/this/is/a/selector'
        s1 = Selector('/this/is/a/selector')
        self.assertEqual(hash(s), hash(s1))

    def test_selector_check_ko_1(self):
        self.assertRaises(ValueError, Selector,
                          '//this/is/a/not/selector')
