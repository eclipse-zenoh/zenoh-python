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
from typing import Any, Generic, Never, Self, TypeVar, overload

from zenoh.handlers import Handler
from zenoh.key_expr import IntoKeyExpr, KeyExpr
from zenoh.prelude import Encoding, IntoEncoding
from zenoh.publication import CongestionControl, Priority
from zenoh.selector import Selector
from zenoh.value import Value

_H = TypeVar("_H")

class Query:
    """Structs received by a Queryable."""

    selector: Selector
    key_expr: KeyExpr
    value: Value
    payload: bytes
    encoding: Encoding

    def reply(
        self,
        key_expr: IntoKeyExpr,
        payload: Any,
        *,
        encoding: IntoEncoding | None = None,
        congestion_control: CongestionControl | None = None,
        priority: Priority | None = None,
        express: bool | None = None,
    ):
        """Sends a reply to this Query.
        By default, queries only accept replies whose key expression intersects with the query's. Unless the query has enabled disjoint replies (you can check this through Query::accepts_replies), replying on a disjoint key expression will result in an error when resolving the reply.
        """

        def reply_err(payload: Any, *, encoding: IntoEncoding | None = None):
            """ "Sends a error reply to this Query."""

        def reply_del(
            key_expr: IntoKeyExpr,
            *,
            congestion_control: CongestionControl | None = None,
            priority: Priority | None = None,
            express: bool | None = None,
        ):
            """Sends a delete reply to this Query.
            By default, queries only accept replies whose key expression intersects with the query's. Unless the query has enabled disjoint replies (you can check this through Query::accepts_replies), replying on a disjoint key expression will result in an error when resolving the reply.
            """

class Queryable(Generic[_H]):
    """A queryable that provides data through a Handler.
    Queryables can be created from a zenoh Session with the declare_queryable function and the with function of the resulting builder.
    Queryables are automatically undeclared when dropped."""

    handler: _H

    def __enter__(self) -> Self: ...
    def __exit__(self, exc_type, exc_val, exc_tb): ...
    def undeclare(self): ...
    @overload
    def try_recv(self: Queryable[Handler[Query]]) -> Query | None: ...
    @overload
    def try_recv(self) -> Never: ...
    @overload
    def recv(self: Queryable[Handler[Query]]) -> Query: ...
    @overload
    def recv(self) -> Never: ...
    @overload
    def __iter__(self: Queryable[Handler[Query]]) -> Handler[Query]: ...
    @overload
    def __iter__(self) -> Never: ...
