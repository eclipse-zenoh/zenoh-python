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
import json
import mvar
import time
import os
from zenoh import Zenoh
# from papero import Property
from zenoh import Selector
from zenoh import Path
from zenoh import Value, ChangeKind
from zenoh import Encoding

ZSERVER = '127.0.0.1'


class APITest(unittest.TestCase):

    def test_create_close_api(self):
        y = Zenoh.login(ZSERVER)
        self.assertTrue(y.rt.running)
        y.logout()
        self.assertFalse(y.rt.running)

    def test_create_delete_storage(self):
        y = Zenoh.login(ZSERVER)
        admin = y.admin()
        stid = '123'
        res1 = admin.add_storage(stid, {'selector': '/myzenoh/**'})
        time.sleep(1)  # TODO remove
        res2 = admin.remove_storage(stid)
        y.logout()
        self.assertTrue(res1)
        self.assertTrue(res2)

    def test_create_delete_workspace(self):
        y = Zenoh.login(ZSERVER)
        admin = y.admin()
        stid = '123'
        admin.add_storage(stid, {'selector': '/myzenoh/**'})
        time.sleep(1)  # TODO remove
        workspace = y.workspace('/myzenoh')
        self.assertEqual(workspace.path, Path('/myzenoh'))
        admin.remove_storage(stid)
        y.logout()

    def test_put_get_remove(self):
        y = Zenoh.login(ZSERVER)
        admin = y.admin()
        stid = '123'
        admin.add_storage(stid, {'selector': '/myzenoh/**'})
        time.sleep(1)  # TODO remove
        workspace = y.workspace('/myzenoh')
        d = Value('hello!', encoding=Encoding.STRING)
        self.assertTrue(workspace.put('/myzenoh/key1', d))
        data = workspace.get('/myzenoh/key1')[0]
        self.assertEqual(data.get_value(), d)
        self.assertEqual(data.get_path(), '/myzenoh/key1')
        self.assertTrue(workspace.remove('/myzenoh/key1'))
        self.assertEqual(workspace.get('/myzenoh/key1'), [])
        admin.remove_storage(stid)
        y.logout()

    def test__big_put_get_remove(self):
        y = Zenoh.login('127.0.0.1')
        admin = y.admin()
        stid = '123'
        admin.add_storage(stid, {'selector': '/myzenoh/**'})
        time.sleep(1)  # TODO remove
        workspace = y.workspace('/myzenoh')

        for i in range(0, 100):
            v = 'x{}'.format(i) * 512
            workspace.put('/myzenoh/big/{}'.format(i),
                          Value(v, encoding=Encoding.STRING))

        dataset = workspace.get('/myzenoh/big/**')
        self.assertEqual(len(dataset), 100)
        admin.remove_storage(stid)
        y.logout()

    def test_sub_unsub(self):
        y = Zenoh.login(ZSERVER)
        admin = y.admin()
        stid = '123'
        admin.add_storage(stid, {'selector': '/myzenoh/**'})
        time.sleep(1)  # TODO remove
        workspace = y.workspace('/myzenoh')
        local_var = mvar.MVar()

        def cb(kvs):
            v = Value('123', encoding=Encoding.STRING)

            self.assertEqual(kvs[0].get_value(), v)
            self.assertEqual(kvs[0].get_path(), '/myzenoh/key1')
            self.assertEqual(kvs[0].get_kind(), ChangeKind.PUT)
            local_var.put(kvs)

        sid = workspace.subscribe('/myzenoh/key1', cb)
        workspace.put('/myzenoh/key1', Value('123', encoding=Encoding.STRING))
        self.assertTrue(workspace.unsubscribe(sid))
        admin.remove_storage(stid)
        y.logout()

    def test_sub_remove(self):
        y = Zenoh.login(ZSERVER)
        admin = y.admin()
        stid = '123'
        admin.add_storage(stid, {'selector': '/myzenoh/**'})
        time.sleep(1)  # TODO remove
        workspace = y.workspace('/myzenoh')
        local_var = mvar.MVar()
        workspace.put('/myzenoh/key1', Value('123', encoding=Encoding.STRING))

        def cb(kvs):
            self.assertEqual(kvs[0].get_path(), '/myzenoh/key1')
            self.assertEqual(kvs[0].get_kind(), ChangeKind.REMOVE)
            local_var.put(kvs)

        sid = workspace.subscribe('/myzenoh/key1', cb)
        workspace.remove('/myzenoh/key1')
        self.assertTrue(workspace.unsubscribe(sid))
        admin.remove_storage(stid)
        y.logout()

    def test_eval(self):
        y = Zenoh.login(ZSERVER)
        admin = y.admin()
        stid = '123'
        admin.add_storage(stid, {'selector': '/myzenoh/**'})
        time.sleep(1)  # TODO remove
        workspace = y.workspace('/myzenoh')

        def cb(path, args):
            return Value('{} World!'.format(args['hello']),
                         encoding=Encoding.STRING)

        workspace.register_eval('/myzenoh/key1', cb)
        dataset = workspace.get('/myzenoh/key1?(hello=mondo)')
        self.assertEqual(dataset[0].get_path(), '/myzenoh/key1')
        self.assertEqual(dataset[0].get_value(),
                         Value('mondo World!', encoding=Encoding.STRING))
        workspace.unregister_eval('/myzenoh/key1')
        admin.remove_storage(stid)
        y.logout()
