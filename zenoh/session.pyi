from collections.abc import Callable
from typing import Any, Self, TypeVar, overload

from zenoh.config import Config, ZenohId
from zenoh.handlers import Handler, PythonHandler, RustHandler
from zenoh.info import SessionInfo
from zenoh.key_expr import IntoKeyExpr
from zenoh.prelude import IntoEncoding
from zenoh.publication import CongestionControl, Priority, Publisher
from zenoh.query import ConsolidationMode, QueryTarget, Reply
from zenoh.queryable import Query, Queryable
from zenoh.sample import Sample
from zenoh.selector import IntoSelector
from zenoh.subscriber import Reliability, Subscriber

_H = TypeVar("_H")

class Session:
    """A zenoh session."""

    info: SessionInfo

    def __enter__(self) -> Self: ...
    def __exit__(self, exc_type, exc_val, exc_tb): ...
    def zid(self) -> ZenohId:
        """Returns the identifier of the current session. zid() is a convenient shortcut."""

    def close(self):
        """Close the zenoh Session.
        Sessions are automatically closed when dropped, but you may want to use this function to handle errors or close the Session asynchronously.
        """

    def undeclare(self, obj): ...
    def config(self) -> Config:
        """Get the current configuration of the zenoh Session.
        The returned configuration Notifier can be used to read the current zenoh configuration through the get function or modify the zenoh configuration through the insert, or insert_json5 funtion.
        """

    def declare_keyexpr(self, key_expr: IntoKeyExpr):
        """Informs Zenoh that you intend to use key_expr multiple times and that it should optimize its transmission.
        The returned KeyExpr's internal structure may differ from what you would have obtained through a simple key_expr.try_into(), to save time on detecting the optimizations that have been associated with it.
        """

    def put(
        self,
        key_expr: IntoKeyExpr,
        payload: Any,
        *,
        encoding: IntoEncoding | None = None,
        congestion_control: CongestionControl | None = None,
        priority: Priority | None = None,
        express: bool | None = None,
    ):
        """Put data."""

    def delete(
        self,
        key_expr: IntoKeyExpr,
        *,
        congestion_control: CongestionControl | None = None,
        priority: Priority | None = None,
        express: bool | None = None,
    ):
        """Delete data."""

    @overload
    def get(
        self,
        selector: IntoSelector,
        *,
        target: QueryTarget | None = None,
        consolidation: ConsolidationMode | None = None,
        timeout: float | int | None = None,
        congestion_control: CongestionControl | None = None,
        priority: Priority | None = None,
        express: bool | None = None,
        payload: Any = None,
        encoding: IntoEncoding | None = None,
        handler: RustHandler[Reply] | None = None,
    ) -> Handler[Reply]:
        """Query data from the matching queryables in the system.
        Unless explicitly requested via GetBuilder::accept_replies, replies are guaranteed to have key expressions that match the requested selector.
        """

    @overload
    def get(
        self,
        selector: IntoSelector,
        *,
        target: QueryTarget | None = None,
        consolidation: ConsolidationMode | None = None,
        timeout: float | int | None = None,
        congestion_control: CongestionControl | None = None,
        priority: Priority | None = None,
        express: bool | None = None,
        payload: Any = None,
        encoding: IntoEncoding | None = None,
        handler: PythonHandler[Reply, _H],
    ) -> _H: ...
    @overload
    def get(
        self,
        selector: IntoSelector,
        *,
        target: QueryTarget | None = None,
        consolidation: ConsolidationMode | None = None,
        timeout: float | int | None = None,
        congestion_control: CongestionControl | None = None,
        priority: Priority | None = None,
        express: bool | None = None,
        payload: Any = None,
        encoding: IntoEncoding | None = None,
        handler: Callable[[Reply], Any],
    ) -> None: ...
    @overload
    def declare_subscriber(
        self,
        key_expr: IntoKeyExpr,
        *,
        reliability: Reliability | None = None,
        handler: RustHandler[Sample] | None = None,
    ) -> Subscriber[Handler[Sample]]:
        """Create a Subscriber for the given key expression."""

    @overload
    def declare_subscriber(
        self,
        key_expr: IntoKeyExpr,
        *,
        reliability: Reliability | None = None,
        handler: PythonHandler[Sample, _H],
    ) -> Subscriber[_H]: ...
    @overload
    def declare_subscriber(
        self,
        key_expr: IntoKeyExpr,
        *,
        reliability: Reliability | None = None,
        handler: Callable[[Sample], Any],
    ) -> Subscriber[None]: ...
    @overload
    def declare_queryable(
        self,
        key_expr: IntoKeyExpr,
        *,
        complete=None,
        handler: RustHandler[Query] | None = None,
    ) -> Queryable[Handler[Query]]:
        """Create a Queryable for the given key expression."""

    @overload
    def declare_queryable(
        self,
        key_expr: IntoKeyExpr,
        *,
        complete=None,
        handler: PythonHandler[Query, _H],
    ) -> Queryable[_H]: ...
    @overload
    def declare_queryable(
        self,
        key_expr: IntoKeyExpr,
        *,
        complete=None,
        handler: Callable[[Query], Any],
    ) -> Queryable[None]: ...
    def declare_publisher(
        self,
        key_expr: IntoKeyExpr,
        *,
        congestion_control: CongestionControl | None = None,
        priority: Priority | None = None,
        express: bool | None = None,
    ) -> Publisher:
        """Create a Publisher for the given key expression."""
