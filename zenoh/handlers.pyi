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
    """Provides access to received Zenoh data.

    `Handler` instances are returned by Zenoh operations that receive data asynchronously.
    Each instance provides methods to access the received data items of type ``_T``.

    `Handler` serves as a common interface for different channel implementations:
    :class:`DefaultHandler`, :class:`FifoChannel`, and :class:`RingChannel`.
    Regardless of which channel type is used, `Handler` provides the same methods
    for data access.

    `Handler` instances are returned by several Zenoh operations:

    - :meth:`zenoh.Session.get` returns `Handler[Reply]` for accessing query replies
    - :meth:`zenoh.Querier.get` returns `Handler[Reply]` for accessing querier replies
    - :meth:`zenoh.Session.declare_subscriber` returns `Subscriber[Handler[Sample]]`
      for accessing received samples

    `Handler` provides both blocking and non-blocking methods to receive data,
    as well as iteration support. The underlying implementation determines the
    specific behavior (FIFO blocking, ring buffer dropping, etc.).
    """

    def try_recv(self) -> _T | None:
        """Attempt to receive an item without blocking.

        Returns the next available item if one is ready, otherwise returns None.
        This method never blocks and is useful for polling or non-blocking loops.

        Returns:
            The next item if available, None otherwise.
        """

    def recv(self) -> _T:
        """Receive an item, blocking if necessary.

        Waits until an item is available and returns it. This method will block
        the calling thread until data arrives.

        Returns:
            The next available item.
        """

    def __iter__(self) -> Self: ...
    def __next__(self) -> _T: ...

@final
class DefaultHandler(Generic[_T]):
    """The default handler type used by Zenoh when no explicit handler is provided.

    `DefaultHandler` serves as an opaque wrapper around :class:`FifoChannel` with default
    settings. When no channel or callback is specified for subscribers or queries,
    Zenoh automatically uses this handler.

    This type provides API stability by allowing the underlying default handler
    implementation to change without breaking existing code. Currently, it wraps
    a FIFO queue implementation.

    For more information about channels and callbacks, see
    :ref:`channels-and-callbacks`.
    """

    ...

@final
class FifoChannel(Generic[_T]):
    """A handler implementing FIFO semantics.

    `FifoChannel` provides a bounded FIFO (First-In-First-Out) queue for handling
    received data. When the channel reaches its capacity, pushing additional
    items will block until space becomes available.

    Note: A slow consumer can block the underlying Zenoh thread if it doesn't
    empty the FifoChannel fast enough. For applications where dropping old
    samples is preferable to blocking, consider using :class:`RingChannel`
    instead.

    For more information about channels and callbacks, see
    :ref:`channels-and-callbacks`.

    Args:
        capacity: The maximum number of items the channel can hold.
    """

    def __new__(cls, capacity: int) -> Self: ...

@final
class RingChannel(Generic[_T]):
    """A synchronous ring channel with a limited size that allows users to keep the last N data items.

    `RingChannel` implements FIFO semantics with a dropping strategy when full.
    When the channel reaches its capacity, the oldest elements are dropped to
    make room for newer ones, ensuring that only the most recent data is kept.

    This makes `RingChannel` ideal for applications that need to maintain a
    sliding window of recent data without blocking the producer.

    For applications where data loss is unacceptable and blocking is preferable
    to dropping old samples, consider using :class:`FifoChannel` instead.

    For more information about channels and callbacks, see
    :ref:`channels-and-callbacks`.

    Args:
        capacity: The maximum number of items the channel can hold.
    """

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
            *This feature is unstable and may change or be removed in future releases.*

            Controls the threading behavior of callback execution. If `True`
            (default), the callback is executed in a separate Python thread.
            If `False`, the callback is executed directly in the same
            thread that receives the data (the zenoh network thread).
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
        """*Unstable* Whether the callback executes in a separate thread (True) or same thread (False)."""
