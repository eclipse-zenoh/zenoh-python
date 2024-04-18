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
from collections.abc import Callable
from typing import TypeVar, overload

_F = TypeVar("_F", bound=Callable)

@overload
def serializer(self, func: _F) -> _F:
    """Register a serializer for a given type, which will be used to serialize payloads.

    If the function is type-annotated, it will use the type of the first argument.
    Otherwise, the type has to be passed."""

@overload
def serializer(self, tp: type) -> Callable[[_F], _F]: ...
@overload
def deserializer(self, func: _F) -> _F:
    """Register a deserializer for a given type, which will be used to deserialize payload.

    If the function is type-annotated, it will use the return type.
    Otherwise, the type has to be passed."""

@overload
def deserializer(self, tp: type) -> Callable[[_F], _F]: ...
