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
