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

from zenoh.value import Value
from zenoh.encoding import Encoding
import zenoh.net


class Admin(object):
    '''
    The Administration helper class.
    '''

    def __init__(self, ws):
        self.ws = ws
        self.local = ''.join('{:02x}'.format(x) for x in
                             ws.rt.info()[zenoh.net.ZN_INFO_PEER_PID_KEY])

    def add_backend(self, beid, properties, zid=None):
        '''
        Add a backend in the specified zenoh router.

        :param beid: the Id of the backend.
        :param propertiers: some configuration for the backend.
        :param zid: the UUID of the zenoh router.
            If ``None``, it's the zenoh router you are directly connected to.
        '''
        if(zid is None):
            zid = self.local
        path = '/@/router/{}/plugin/storages/backend/{}'.format(
            zid, beid)
        value = Value(properties, encoding=Encoding.PROPERTIES)
        return self.ws.put(path, value)

    def get_backends(self, zid=None):
        '''
        Get all the backends from the specified zenoh router.

        :param zid: the UUID of the zenoh router.
            If ``None``, it's the zenoh router you are directly connected to.
        :returns: a list of (backend_id, properties) tuples.
        '''
        if(zid is None):
            zid = self.local
        s = '/@/router/{}/plugin/storages/backend/*'.format(
            zid)
        dataset = self.ws.get(s)
        return list(map(
            lambda d: (d.get_path().split('/')[-1],
                       d.get_value().value), dataset))

    def get_backend(self, beid, zid=None):
        '''
        Get backend's properties from the specified zenoh router.

        :param beid: the Id of the backend.
        :param zid: the UUID of the zenoh router.
            If ``None``, it's the zenoh router you are directly connected to.
        :returns: the backend properties.
        '''
        if(zid is None):
            zid = self.local
        s = '/@/router/{}/plugin/storages/backend/{}'.format(
            zid, beid)
        dataset = self.ws.get(s)
        if len(dataset) > 0:
            return dataset[0].get_value().value
        return None

    def remove_backend(self, beid, zid=None):
        '''
        Remove a backend from the specified zenoh router.

        :param beid: the Id of the backend.
        :param zid: the UUID of the zenoh router.
            If ``None``, it's the zenoh router you are directly connected to.
        '''
        if(zid is None):
            zid = self.local
        path = '/@/router/{}/plugin/storages/backend/{}'.format(
            zid, beid)
        return self.ws.remove(path)

    def add_storage(self, stid, properties, beid=None, zid=None):
        '''
        Adds a storage in the specified zenoh router, using the specified
        backend.

        :param stid: the Id of the storage.
        :param propertiers: some configuration for the storage.
        :param beid: the Id of the backend. If ``None``, a backend is
            automatically selected.
        :param zid: the UUID of the zenoh router.
            If ``None``, it's the zenoh router you are directly connected to.
        '''
        if(zid is None):
            zid = self.local
        if not beid:
            beid = 'auto'
        p = '/@/router/{}/plugin/storages/backend/{}/storage/{}'.format(
            zid, beid, stid)
        v = Value(properties, encoding=Encoding.PROPERTIES)
        return self.ws.put(p, v)

    def get_storages(self, beid=None, zid=None):
        '''
        Get all the storages from the specified zenoh router.

        :param beid: the Id of the backend. If ``None``, all backends.
        :param zid: the UUID of the zenoh router.
            If ``None``, it's the zenoh router you are directly connected to.
        :returns: a list of (storage_id, properties) tuples.
        '''
        if(zid is None):
            zid = self.local
        if not beid:
            beid = '*'
        s = '/@/router/{}/plugin/storages/backend/{}/storage/*'.format(
            zid, beid)
        dataset = self.ws.get(s)
        return list(map(
            lambda d: (d.get_path().split('/')[-1],
                       d.get_value().value), dataset))

    def get_storage(self, stid, zid=None):
        '''
        Get storage's properties from the specified zenoh router.

        :param stid: the Id of the storage.
        :param zid: the UUID of the zenoh router.
            If ``None``, it's the zenoh router you are directly connected to.
        :returns: the storage properties.
        '''
        if(zid is None):
            zid = self.local
        s = '/@/router/{}/plugin/storages/backend/*/storage/{}'.format(
            zid, stid)
        dataset = self.ws.get(s)
        if len(dataset) > 0:
            return dataset[0].get_value().value
        return None

    def remove_storage(self, stid, zid=None):
        '''
        Remove a backend from the specified zenoh router.

        :param stid: the Id of the storage.
        :param zid: the UUID of the zenoh router.
            If ``None``, it's the zenoh router you are directly connected to.
        '''
        if(zid is None):
            zid = self.local
        s = '/@/router/{}/plugin/storages/backend/*/storage/{}'.format(
            zid, stid)
        dataset = self.ws.get(s)
        if len(dataset) > 0:
            p = dataset[0].get_path()
            return self.ws.remove(p)
        return False
