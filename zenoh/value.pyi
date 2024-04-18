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
from typing import Self, Type, TypeVar, overload

from zenoh.prelude import Encoding, IntoEncoding

_T = TypeVar("_T")

class Value:
    @overload
    def __new__(cls, value: Value) -> Self: ...
    @overload
    def __new__(cls, payload, encoding: IntoEncoding | None = None) -> Self: ...
    @classmethod
    def empty(cls) -> Self:
        """Creates an empty Value."""

    def is_empty(self) -> bool:
        """Checks if the Value is empty. Value is considered empty if its payload is empty and encoding is default."""
    payload: bytes
    encoding: Encoding

    def payload_as(self, tp: Type[_T]) -> _T: ...
