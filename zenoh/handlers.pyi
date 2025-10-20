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
    """A callback handler that invokes a user-defined function for each received item.

    The `Callback` class provides a way to handle asynchronous data reception by
    calling a user-provided callback function for each item received. It can also
    optionally call a drop function when the associated Zenoh primitive
    (:class:`zenoh.Subscriber`, :class:`zenoh.Querier`, etc.) is undeclared.
    
    When a `Callback` handler is used, the associated Zenoh primitive runs in
    background mode, meaning the callback continues to execute even if the object
    goes out of scope. For more information about channels and callbacks, see
    :ref:`channels-and-callbacks`.
    
    Args:
        callback: 
            A callable that will be invoked for each received item.
        
        drop: 
            An optional callable that will be invoked when the associated Zenoh
            primitive is undeclared and the callback handler is being cleaned up.
            
        indirect: 
            Controls the threading behavior of callback execution. If `True`
            (default), the callback is executed in a separate Python thread,
            allowing for long-running callbacks without blocking Zenoh's internal
            processing. If `False`, the callback is executed directly in the same
            thread that receives the data, which is more efficient but requires
            callbacks to complete quickly to avoid blocking.
    """
    def __new__(
        cls,
        callback: Callable[[_T], Any],
        drop: Callable[[], Any] | None = None,
        *,
        indirect: bool = True,
    ) -> Self: ...
    def __call__(self, arg: _T, /) -> Any: ...
    @property
    def callback(self) -> Callable[[_T], Any]: 
        """The callback function that will be invoked for each received item."""
    @property
    def drop(self) -> Callable[[], Any] | None: 
        """The optional drop function that will be invoked when the handler is cleaned up."""
    @property
    def indirect(self) -> bool: 
        """Whether the callback executes in a separate thread (True) or same thread (False)."""
