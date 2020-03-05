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
from zenoh import Path


class PathTests(unittest.TestCase):

    def test_path_check_ok(self):
        p = Path('/this/is/a/path')
        self.assertEqual('/this/is/a/path', p.to_string())

    def test_path_check_absolute_ok(self):
        p = Path('/this/is/a/absoliute/path')
        self.assertTrue(p.is_absolute())

    def test_path_check_absolute_ko(self):
        path = Path('this/is/a/relative/path')
        self.assertFalse(path.is_absolute())

    def test_path_prefix(self):
        p = Path('/this/is/a/path/with/a/prefix')
        self.assertTrue(p.is_prefix('/this/is/a/path'))
        self.assertFalse(p.is_prefix('/that/is/a/path'))

    def test_path_remove_prefix(self):
        p = Path('/this/is/a/path/with/a/prefix')
        self.assertTrue(p.is_prefix('/this/is/a/path'))
        p.remove_prefix('/this/is/a/path')
        self.assertEqual(p.to_string(), '/with/a/prefix')

    def test_path_remove_prefix_no_prefix(self):
        p = Path('/this/is/a/path/with/a/prefix')
        p.remove_prefix('/that/is/a/path')
        self.assertEqual(p.to_string(), '/this/is/a/path/with/a/prefix')

    def test_to_path(self):
        p = '/test'
        p2 = Path('/test')
        self.assertEqual(Path.to_path(p), Path('/test'))
        self.assertEqual(Path.to_path(p2), Path('/test'))

    def test_path_equal(self):
        p1 = Path('/this/is/a/path')
        p2 = Path('/this/is/a/path')
        self.assertEqual(p1, p2)

    def test_path_not_equal(self):
        p1 = Path('/this/is/a/path')
        p2 = '/this/is/not/a/path'
        self.assertNotEqual(p1, p2)

    def test_path_str(self):
        p1 = Path('/this/is/a/path')
        self.assertEqual(str(p1), '/this/is/a/path')

    def test_path_hash(self):
        p1 = Path('/this/is/a/path')
        self.assertEqual(hash(p1), hash('/this/is/a/path'))

    def test_path_repr(self):
        p1 = Path('/this/is/a/path')
        self.assertEqual(repr(p1), '/this/is/a/path')

    def test_path_len(self):
        s = '/this/is/a/path'
        p1 = Path('/this/is/a/path')
        self.assertEqual(len(s), len(p1))

    def test_path_check_ko_1(self):
        self.assertRaises(ValueError, Path, '//this/is/a/not/path')

    def test_path_check_ko_2(self):
        self.assertRaises(ValueError, Path, '//this/is/a/not/path/**')

    def test_path_check_ko_3(self):
        self.assertRaises(ValueError,
                          Path, '//this/is/a/not/path?with=query')

    def test_path_check_ko_4(self):
        self.assertRaises(ValueError,
                          Path, '//this/is/a/not/path#fragment')
