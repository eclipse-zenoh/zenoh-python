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
