from enum import Enum, auto
from typing import Self, Type, TypeVar

from zenoh.key_expr import KeyExpr
from zenoh.prelude import Encoding
from zenoh.publication import CongestionControl, Priority
from zenoh.time import Timestamp

_T = TypeVar("_T")

class QoS:
    priority: Priority
    congestion_control: CongestionControl
    express: bool

    def __new__(
        cls,
        priority: Priority | None = None,
        congestion_control: CongestionControl | None = None,
        express: bool | None = None,
    ) -> Self: ...

class SampleKind(Enum):
    PUT = auto()
    DELETE = auto()

class Sample:
    key_expr: KeyExpr
    payload: bytes
    kind: SampleKind
    encoding: Encoding
    timestamp: Timestamp
    qos: QoS

    def payload_as(self, tp: Type[_T]) -> _T: ...
