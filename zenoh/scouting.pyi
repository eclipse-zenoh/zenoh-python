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
"""Scouting primitives."""

from typing import Generic, Never, Self, TypeVar, overload, Any

from zenoh.config import WhatAmI, ZenohId
from zenoh.handlers import Handler

H = TypeVar("H")

class Hello:
    version: int
    whatAmI: WhatAmI
    zid: ZenohId
    locators: list[str]

class Scout(Generic[H]):
    handler: H

    def __enter__(self) -> Self: ...
    def __exit__(self, exc_type, exc_val, exc_tb): ...
    def stop(self): ...
    @overload
    def try_recv(self: Scout[Handler[Hello]]) -> Hello | None: ...
    @overload
    def try_recv(self: Scout[Any]) -> Never: ...
    @overload
    def recv(self: Scout[Handler[Hello]]) -> Hello: ...
    @overload
    def recv(self) -> Never: ...
    @overload
    def __iter__(self: Scout[Handler[Hello]]) -> Handler[Hello]: ...
    @overload
    def __iter__(self) -> Never: ...
