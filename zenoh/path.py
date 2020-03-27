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


class Path(object):
    def __init__(self, path):
        self.__path_regex = re.compile('^[^?#*]+$')
        if not self.is_valid(path):
            raise ValueError("Invalid Path: {}".format(path))
        self._path = path

    @property
    def path(self):
        return self._path

    @staticmethod
    def to_path(p):
        if isinstance(p, Path):
            return p
        else:
            return Path(p)

    def is_valid(self, path):
        return self.__path_regex.match(path) is not None \
            and not path.startswith('//')

    def is_absolute(self):
        if self._path.startswith('/'):
            return True
        return False

    def is_prefix(self, prefix):
        return self._path.startswith(prefix)

    def to_string(self):
        return self._path

    def __len__(self):
        return len(self._path)

    def __eq__(self, second_path):
        if isinstance(second_path, self.__class__):
            return self._path == second_path._path
        return False

    def __str__(self):
        return self._path

    def __repr__(self):
        return self.__str__()

    def __hash__(self):
        return self._path.__hash__()
