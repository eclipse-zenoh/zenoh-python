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

import zenoh.net


class Data(object):
    '''
    A zenoh data returned by a :func:`Workspace.get` query.
    The Data objects are comparable according to their :class:`core.Timestamp`.
    Note that zenoh makes sure that each published path/value has a unique
    timestamp accross the system.
    '''

    def __init__(self, path, value, timestamp):
        self.path = path
        self.value = value
        self.timestamp = timestamp

    def __hash__(self):
        # As timestamp is unique per data, only hash the timestamp.
        return hash(self.timestamp)

    def __eq__(self, other):
        # As timestamp is unique per data, only compare timestamps.
        return self.timestamp.__eq__(other.timestamp)

    def __lt__(self, other):
        # order data according to timestamps
        return self.timestamp.__lt__(other.timestamp)

    def get_path(self):
        '''
        Returns the path of resource that changed.
        '''
        return self.path

    def get_value(self):
        '''
        Returns the :class:`Value` of the data.
        '''
        return self.value

    def get_timestamp(self):
        '''
        Returns the :class:`core.Timestamp` of the data.
        '''
        return self.timestamp
