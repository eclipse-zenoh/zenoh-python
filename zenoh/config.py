#
# Copyright (c) 2022 ZettaScale Technology
#
# This program and the accompanying materials are made available under the
# terms of the Eclipse Public License 2.0 which is available at
# http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
# which is available at https://www.apache.org/licenses/LICENSE-2.0.
#
# SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
#
# Contributors:
#   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
#
from typing import Union
from .zenoh import _Config
import json

class Config(_Config):
    def __init__(self):
        super().__init__()
    @staticmethod
    def from_file(filename: str):
        """
        Reads the configuration from a file.
        The file's extension must be json, json5 or yaml.
        """
        c = super(Config, Config).from_file(filename)
        return c
    @staticmethod
    def from_obj(obj):
        """
        Reads the configuration from `obj` as if it was a JSON file.
        """
        c = Config.from_json5(json.dumps(obj))
        return c
    @staticmethod
    def from_json5(json: str):
        """
        Reads the configuration from a JSON5 string.

        JSON5 is a superset of JSON, so any JSON string is a valid input for this function.
        """
        c =  super(Config, Config).from_json5(json)
        return c
    
    def get_json(self, path: str) -> str:
        """
        Returns the part of the configuration at `path`,
        in a JSON-serialized form.
        """
        return super().get_json(path)
    
    def insert_json5(self, path: str, value: str) -> str:
        """
        Inserts the provided value (read from JSON) at the given path in the configuration.
        """
        return super().insert_json5(path, value)

MODE_KEY = "mode"
CONNECT_KEY = "connect/endpoints"
LISTEN_KEY = "listen/endpoints"