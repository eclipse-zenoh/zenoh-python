from enum import Enum, auto

from zenoh.config import ZenohId
from zenoh.sample import Sample
from zenoh.value import Value

class QueryTarget(Enum):
    """The kind of consolidation."""

    BEST_MATCHING = auto()
    ALL = auto()
    ALL_COMPLETE = auto()

    DEFAULT = BEST_MATCHING

class ConsolidationMode(Enum):
    """The kind of consolidation."""

    AUTO = auto()
    NONE = auto()
    MONOTONIC = auto()
    LATEST = auto()

    DEFAULT = AUTO

class Reply:
    """Structs returned by a get."""

    result: Sample | Value
    ok: Sample | None
    err: Value | None
    replier_id: ZenohId
