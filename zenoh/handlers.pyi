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
from typing import Any, Generic, Protocol, Self, TypeVar, final

_T = TypeVar("_T")

@final
class Handler(Generic[_T]):
    """Handler for `DefaultHandler`/`FifoHandler`/`RingHandler`."""

    def try_recv(self) -> _T | None: ...
    def recv(self) -> _T: ...
    def __iter__(self) -> Self: ...
    def __next__(self) -> _T: ...

@final
class DefaultHandler(Generic[_T]):
    """The default handler in Zenoh is a FIFO queue."""

    ...

@final
class FifoChannel(Generic[_T]):
    """The default handler in Zenoh is a FIFO queue."""

    def __new__(cls, capacity: int) -> Self: ...

@final
class RingChannel(Generic[_T]):
    """A synchrounous ring channel with a limited size that allows users to keep the last N data."""

    def __new__(cls, capacity: int) -> Self: ...

@final
class Callback(Generic[_T]):
    def __new__(
        cls,
        callback: Callable[[_T], Any],
        drop: Callable[[], Any] | None = None,
        *,
        indirect: bool = True,
    ) -> Self: ...
    def __call__(self, arg: _T, /) -> Any: ...
    @property
    def callback(self) -> Callable[[_T], Any]: ...
    @property
    def drop(self) -> Callable[[], Any] | None: ...
    @property
    def indirect(self) -> bool: ...
