#
# Copyright (c) 2024 ZettaScale Technology
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
"""Configuration to pass to zenoh::open() and zenoh::scout() functions and associated constants."""

from collections.abc import Iterable
from enum import Enum, auto
from pathlib import Path
from typing import Any, Self

from zenoh.key_expr import KeyExpr

class Config:
    """The main configuration structure for Zenoh."""

    def id(self) -> ZenohId:
        """The Zenoh ID of the instance. This ID MUST be unique throughout your Zenoh infrastructure and cannot exceed 16 bytes of length. If left unset, a random u128 will be generated."""

    @classmethod
    def from_env(cls) -> Self: ...
    @classmethod
    def from_file(cls, path: str | Path) -> Self: ...
    @classmethod
    def from_json5(cls) -> Self: ...
    def get_json(self, key: str) -> Any: ...
    def insert_json5(self, key: str, value: Any): ...

class WhatAmI(Enum):
    ROUTER = auto()
    PEER = auto()
    CLIENT = auto()

class WhatAmIMatcher:
    @classmethod
    def empty(cls) -> Self: ...
    def router(self) -> Self: ...
    def peer(self) -> Self: ...
    def client(self) -> Self: ...
    def is_empty(self) -> bool: ...
    def matches(self, whatami: WhatAmI) -> bool: ...

class ZenohId:
    """The global unique id of a zenoh peer."""

    def into_keyexpr(self) -> KeyExpr: ...

def client(peers: Iterable[str]) -> Config: ...
def default() -> Config: ...
def empty() -> Config: ...
def peer() -> Config: ...

IntoWhatAmIMatcher = WhatAmIMatcher | str
