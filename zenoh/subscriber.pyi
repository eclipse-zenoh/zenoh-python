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
from enum import Enum, auto
from typing import Generic, Never, Self, TypeVar, overload

from zenoh.handlers import Handler
from zenoh.key_expr import KeyExpr
from zenoh.sample import Sample

_H = TypeVar("_H")

class Reliability(Enum):
    BEST_EFFORT = auto()
    RELIABLE = auto()

class Subscriber(Generic[_H]):
    """A subscriber that provides data through a Handler.
    Subscribers can be created from a zenoh Session with the declare_subscriber function and the with function of the resulting builder.
    Subscribers are automatically undeclared when dropped."""

    def __enter__(self) -> Self: ...
    def __exit__(self, exc_type, exc_val, exc_tb): ...

    key_expr: KeyExpr
    handler: _H
    def undeclare(self):
        """Close a Subscriber.
        Subscribers are automatically closed when dropped, but you may want to use this function to handle errors or close the Subscriber asynchronously.
        """

    @overload
    def try_recv(self: Subscriber[Handler[Sample]]) -> Sample | None: ...
    @overload
    def try_recv(self) -> Never: ...
    @overload
    def recv(self: Subscriber[Handler[Sample]]) -> Sample: ...
    @overload
    def recv(self) -> Never: ...
    @overload
    def __iter__(self: Subscriber[Handler[Sample]]) -> Handler[Sample]: ...
    @overload
    def __iter__(self) -> Never: ...
