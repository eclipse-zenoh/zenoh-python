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
from enum import Enum


class ChangeKind(Enum):
    '''
    The kind of Change: either PUT, UPDATE or REMOVE.
    '''
    PUT = zenoh.net.ZN_PUT
    '''
    PUT kind.
    '''
    UPDATE = zenoh.net.ZN_UPDATE
    '''
    UPDATE kind.
    '''
    REMOVE = zenoh.net.ZN_REMOVE
    '''
    REMOVE kind.
    '''


class Change(object):
    '''
    The notification of a change for a resource in zenoh.
    See :func:`Workspace.subscribe`.
    '''
    kind_map = {
            zenoh.net.ZN_PUT: ChangeKind.PUT,
            zenoh.net.ZN_UPDATE: ChangeKind.UPDATE,
            zenoh.net.ZN_REMOVE: ChangeKind.REMOVE
    }

    def __init__(self, path, kind, timestamp, value=None):
        self.path = path
        self.timestamp = timestamp
        self.value = value
        if kind is None:
            self.kind = ChangeKind.PUT
        elif isinstance(kind, ChangeKind):
            self.kind = kind
        else:
            self.kind = Change.kind_map[kind]

    def get_path(self):
        '''
        Returns the path of resource that changed.
        '''
        return self.path

    def get_kind(self):
        '''
        Returns the :class:`ChangeKind`.
        '''
        return self.kind

    def get_timestamp(self):
        '''
        Returns the :class:`core.Timestamp` when change occured.
        '''
        return self.timestamp

    def get_value(self):
        '''
        Depending of the :class:`ChangeKind`, returns:
           * if kind is :class:`ChangeKind.PUT`: the new value
           * if kind is :class:`ChangeKind.UPDATE`: the delta value
           * if kind is :class:`ChangeKind.REMOVE`: ``None``

        :returns: a :class:`Value` or ``None``
        '''
        return self.value

    def __str__(self):
        return 'Path: {} Kind: {} Time: {} Value: {}'.format(
            self.path,
            self.kind,
            self.timestamp,
            self.value)

    def __repr__(self):
        return self.__str__()
