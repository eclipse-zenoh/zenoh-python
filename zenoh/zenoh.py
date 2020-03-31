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

from zenoh.workspace import Workspace
from zenoh.admin import *
import threading
from zenoh.net import Session, ZN_INFO_PEER_PID_KEY


class Zenoh(object):
    '''
    The Zenoh client API.
    '''

    ZENOH_DEFAULT_PORT = 7447

    def __init__(self, rt):
        self.rt = rt

    @staticmethod
    def login(locator, properties=None):
        '''
        Establish a zenoh session via a provided locator. Locator is a string
        representing the network endpoint to which establish the session. If
        the provided locator is ``None``, login will perform some dynamic
        discovery and try to establish the session automatically. When not
        ``None``, the locator must have the format: ``tcp/<ip>:<port>``
        (for instance ``tcp/127.0.0.1:7447``).

        :param locator: a Zenoh locator or ``None``.
        :param properties: the Properties to be used for this session
            (e.g. "user", "password", ...). Can be ``None``.
        :returns: a Zenoh object.
        '''
        zprops = {} if properties is None else {
            zenoh.net.ZN_USER_KEY if k == "user" else
            zenoh.net.ZN_PASSWD_KEY: val
            for k, val in properties.items()
            if k == "user" or k == "password"}

        return Zenoh(Session.open(locator, zprops))

    def logout(self):
        '''
        Terminates the Zenoh session.
        '''
        self.rt.close()

    def workspace(self, path='/', executor=None):
        '''
        Creates a :class:`Workspace` using the
        provided path. All relative Selector or Path used with this
        :class:`Workspace` will be relative to
        this path.

        :param path: the Workspace's path.
        :param executor: an executor of type
            :py:class:`concurrent.futures.Executor` or ``None``.
            If not ``None``, all subscription listeners and eval callbacks are
            executed by the provided executor. This is useful when listeners
            and/or callbacks need to perform long operations or need to call
            operations like :func:`Workspace.get`.
        :returns: a :class:`Workspace`.
        '''
        return Workspace(self.rt, path, executor)

    def admin(self):
        '''
        Returns the admin object that provides helper operations to
        administer Zenoh.

        :returns: a :class:`Admin`.
        '''
        return Admin(self.workspace(
            '/@/{}'.format(
                ''.join('{:02x}'.format(x) for x in
                        self.rt.info()[ZN_INFO_PEER_PID_KEY]))))
