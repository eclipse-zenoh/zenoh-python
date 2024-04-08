from collections.abc import Callable
from typing import Any, Generic, Protocol, Self, TypeVar

_T = TypeVar("_T", contravariant=True)
_T2 = TypeVar("_T2", covariant=True)
_H = TypeVar("_H")

class Handler(Generic[_T2]):
    """Handler for `DefaultHandler`/`FifoHandler`/`RingHandler`."""

    def try_recv(self) -> _T2 | None: ...
    def recv(self) -> _T2: ...
    def __iter__(self) -> Self: ...
    def __next__(self) -> _T2: ...

class DefaultHandler(Generic[_T]):
    """The default handler in Zenoh is a FIFO queue."""

    ...

class FifoChannel(Generic[_T]):
    """The default handler in Zenoh is a FIFO queue."""

    def __new__(cls, capacity: int) -> Self: ...

class RingChannel(Generic[_T]):
    """A synchrounous ring channel with a limited size that allows users to keep the last N data."""

    def __new__(cls, capacity: int) -> Self: ...

RustHandler = DefaultHandler[_T] | FifoChannel[_T] | RingChannel[_T]

class CallbackDrop(Generic[_T]):
    def __new__(
        cls, callback: Callable[[_T], Any], drop: Callable[[], Any]
    ) -> Self: ...
    def __call__(self, arg: _T, /) -> Any: ...
    def drop(self) -> Any: ...

class _PythonCallback(Protocol[_T]):
    def __call__(self, arg: _T, /) -> Any: ...
    def drop(self) -> Any: ...

PythonCallback = Callable[[_T], Any] | _PythonCallback[_T]
PythonHandler = tuple[PythonCallback[_T], _H]
