from enum import Enum, auto
from typing import Any, Self

from zenoh.key_expr import KeyExpr
from zenoh.prelude import IntoEncoding

class Priority(Enum):
    """The Priority of zenoh messages."""

    REAL_TIME = auto()
    INTERACTIVE_HIGH = auto()
    INTERACTIVE_LOW = auto()
    DATA_HIGH = auto()
    DATA = auto()
    DATA_LOW = auto()
    BACKGROUND = auto()

    DEFAULT = DATA
    MIN = BACKGROUND
    MAX = REAL_TIME

class CongestionControl(Enum):
    """The kind of congestion control."""

    DROP = auto()
    BLOCK = auto()

    DEFAULT = DROP

class Publisher:
    """A publisher that allows to send data through a stream.
    Publishers are automatically undeclared when dropped."""

    key_expr: KeyExpr

    def __enter__(self) -> Self: ...
    def __exit__(self, exc_type, exc_val, exc_tb): ...
    @property
    def congestion_control(self) -> CongestionControl: ...
    @congestion_control.setter
    def congestion_control(self, congestion_control: CongestionControl): ...
    @property
    def priority(self) -> Priority: ...
    @priority.setter
    def priority(self, priority: Priority): ...
    def put(self, payload: Any, *, encoding: IntoEncoding | None = None):
        """Put data."""

    def delete(self):
        """Put data."""

    def undeclare(self):
        """Undeclares the Publisher, informing the network that it needn't optimize publications for its key expression anymore."""
