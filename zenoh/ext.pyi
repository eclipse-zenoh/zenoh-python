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
from typing import Any, Generic, Literal, Never, Self, TypeVar, final, overload

from zenoh import (
    CongestionControl,
    Encoding,
    EntityGlobalId,
    Handler,
    KeyExpr,
    Locality,
    Priority,
    Reliability,
    Sample,
    Session,
    Subscriber,
    Timestamp,
    ZBytes,
    handlers,
)

_T = TypeVar("_T")
_H = TypeVar("_H")

_RustHandler = (
    handlers.DefaultHandler[_T] | handlers.FifoChannel[_T] | handlers.RingChannel[_T]
)
_PythonCallback = Callable[[_T], Any]
_PythonHandler = tuple[_PythonCallback[_T], _H]

def _unstable(item: _T) -> _T:
    """marker for unstable functionality"""

_IntoEncoding = Encoding | str
_IntoKeyExpr = KeyExpr | str
_IntoZBytes = Any

class Int8(int):
    """int subclass enabling to (de)serialize 8bit signed integer."""

class Int16(int):
    """int subclass enabling to (de)serialize 16bit signed integer."""

class Int32(int):
    """int subclass enabling to (de)serialize 32bit signed integer."""

class Int64(int):
    """int subclass enabling to (de)serialize 64bit signed integer."""

class Int128(int):
    """int subclass enabling to (de)serialize 128bit signed integer."""

class UInt8(int):
    """int subclass enabling to (de)serialize 8bit unsigned integer."""

class UInt16(int):
    """int subclass enabling to (de)serialize 16bit unsigned integer."""

class UInt32(int):
    """int subclass enabling to (de)serialize 32bit unsigned integer."""

class UInt64(int):
    """int subclass enabling to (de)serialize 64bit unsigned integer."""

class UInt128(int):
    """int subclass enabling to (de)serialize 128bit unsigned integer."""

class Float32(float):
    """float subclass enabling to (de)serialize 32bit floating point numbers."""

class Float64(float):
    """float subclass enabling to (de)serialize 64bit floating point numbers."""

class ZDeserializeError(Exception):
    """Exception raised when deserialization with :meth:`zenoh.ext.z_deserialize` fails."""
    pass

def z_serialize(obj: Any) -> ZBytes:
    """Serialize an object of supported type according to the `Zenoh serialization format <https://github.com/eclipse-zenoh/roadmap/blob/main/rfcs/ALL/Serialization.md>`_.

    Supported types are:

    * UInt8, UInt16, Uint32, UInt64, UInt128, Int8, Int16, Int32, Int64, Int128, int (handled as int32), Float32, Float64, float (handled as Float64), bool;

    * Str, Bytes, ByteArray;

    * List, Dict, Set, FrozenSet and Tuple of supported types.
    """
    pass

def z_deserialize(tp: type[_T], zbytes: ZBytes) -> _T:
    """Deserialize into an object of supported type according to the `Zenoh serialization format <https://github.com/eclipse-zenoh/roadmap/blob/main/rfcs/ALL/Serialization.md>`_.

    Supported types are:

    * UInt8, UInt16, Uint32, UInt64, UInt128, Int8, Int16, Int32, Int64, Int128, int (handled as int32), Float32, Float64, float (handled as Float64), bool;

    * Str, Bytes, ByteArray;

    * List, Dict, Set, FrozenSet and Tuple of supported types.
    """
    pass

@_unstable
@final
class AdvancedPublisher:
    """An extension to Publisher providing advanced functionalities.

    AdvancedPublisher works alongside :class:`AdvancedSubscriber` to enable:

    * **Caching** - Store last published samples for retrieval via subscriber history mechanisms
    * **Sample miss detection** - Identify gaps in publications to detect missed samples
    * **Publisher detection** - Assert presence through liveliness tokens

    Publishers are created via :func:`declare_advanced_publisher`.
    """

    def __enter__(self) -> Self: ...
    def __exit__(self, *_args, **_kwargs): ...
    @_unstable
    @property
    def id(self) -> EntityGlobalId:
        """The globally unique id of this AdvancedPublisher."""

    @property
    def key_expr(self) -> KeyExpr:
        """The key expression this AdvancedPublisher publishes to."""

    @property
    def encoding(self) -> Encoding:
        """The encoding used for published data."""

    @property
    def congestion_control(self) -> CongestionControl:
        """The congestion control policy applied to published data."""

    @property
    def priority(self) -> Priority:
        """The priority level of published data."""

    def matching_status(self) -> bool:
        """Check if there are currently matching subscribers.

        :return: True if matching subscribers exist, False otherwise
        """

    @overload
    def matching_listener(
        self, handler: _RustHandler[bool] | None = None
    ) -> Subscriber[Handler[bool]]:
        """Declare a listener to monitor changes in matching subscriber status.

        The listener will be called whenever the matching status changes
        (i.e., when subscribers appear or disappear).

        :param handler: Optional handler for receiving matching status updates
        :return: A Subscriber that receives boolean values indicating matching status
        """

    @overload
    def matching_listener(
        self, handler: _PythonHandler[bool, _H]
    ) -> Subscriber[_H]:
        """Declare a listener to monitor changes in matching subscriber status.

        The listener will be called whenever the matching status changes
        (i.e., when subscribers appear or disappear).

        :param handler: Optional handler for receiving matching status updates
        :return: A Subscriber that receives boolean values indicating matching status
        """

    @overload
    def matching_listener(
        self, handler: _PythonCallback[bool]
    ) -> Subscriber[None]:
        """Declare a listener to monitor changes in matching subscriber status.

        The listener will be called whenever the matching status changes
        (i.e., when subscribers appear or disappear).

        :param handler: Optional handler for receiving matching status updates
        :return: A Subscriber that receives boolean values indicating matching status
        """

    def put(
        self,
        payload: _IntoZBytes,
        *,
        encoding: _IntoEncoding | None = None,
        attachment: _IntoZBytes | None = None,
        timestamp: Timestamp | None = None,
    ):
        """Publish data to the key expression.

        :param payload: The data to publish
        :param encoding: Optional encoding override; if not specified, uses the publisher's default encoding
        :param attachment: Optional user attachment
        :param timestamp: Optional timestamp; if not specified, the session will generate one
        """

    def delete(
        self,
        *,
        attachment: _IntoZBytes | None = None,
        timestamp: Timestamp | None = None,
    ):
        """Delete the value associated with the key expression.

        :param attachment: Optional user attachment
        :param timestamp: Optional timestamp; if not specified, the session will generate one
        """

    def undeclare(self):
        """Undeclare the AdvancedPublisher.

        This informs the network that optimizations for this publisher are no longer needed.
        """

@_unstable
@final
class AdvancedSubscriber(Generic[_H]):
    """An extension to Subscriber providing advanced functionalities.

    AdvancedSubscriber works alongside :class:`AdvancedPublisher` to enable:

    * **Sample miss detection** - Identify gaps in received publications to detect missed samples
    * **Sample recovery** - Retrieve historical samples with configurable constraints (max age, sample count)
    * **Publisher detection** - Discover matching publishers via liveliness mechanisms
    * **Late joiner support** - Detect late-joining publishers and query for their historical data

    Subscribers are created via :func:`declare_advanced_subscriber`.
    """

    def __enter__(self) -> Self: ...
    def __exit__(self, *_args, **_kwargs): ...
    @_unstable
    @property
    def id(self) -> EntityGlobalId:
        """The globally unique id of this AdvancedSubscriber."""

    @property
    def key_expr(self) -> KeyExpr:
        """The key expression this AdvancedSubscriber subscribes to."""

    @property
    def handler(self) -> _H:
        """The handler used to process received samples."""
    @overload
    def sample_miss_listener(
        self, handler: _RustHandler[Miss] | None = None
    ) -> SampleMissListener[Handler[Miss]]:
        """Declare a listener to detect missed samples.

        Missed samples can only be detected from :class:`AdvancedPublisher` instances
        that enable ``sample_miss_detection``. The listener will receive :class:`Miss`
        notifications indicating the source and number of missed samples.

        :param handler: Optional handler for receiving miss notifications
        :return: A SampleMissListener for monitoring missed samples
        """

    @overload
    def sample_miss_listener(
        self, handler: _PythonHandler[Miss, _H]
    ) -> SampleMissListener[_H]:
        """Declare a listener to detect missed samples.

        Missed samples can only be detected from :class:`AdvancedPublisher` instances
        that enable ``sample_miss_detection``. The listener will receive :class:`Miss`
        notifications indicating the source and number of missed samples.

        :param handler: Handler tuple (callback, handler) for receiving miss notifications
        :return: A SampleMissListener for monitoring missed samples
        """

    @overload
    def sample_miss_listener(
        self, handler: _PythonCallback[Miss]
    ) -> SampleMissListener[None]:
        """Declare a listener to detect missed samples.

        Missed samples can only be detected from :class:`AdvancedPublisher` instances
        that enable ``sample_miss_detection``. The listener will receive :class:`Miss`
        notifications indicating the source and number of missed samples.

        :param handler: Callback function for receiving miss notifications
        :return: A SampleMissListener for monitoring missed samples
        """

    @overload
    def detect_publishers(
        self,
        handler: _RustHandler[Sample] | None = None,
        *,
        history: bool | None = None,
    ) -> Subscriber[Handler[Sample]]:
        """Declare a listener to detect matching publishers.

        Only :class:`AdvancedPublisher` instances that enable ``publisher_detection``
        can be detected. This uses liveliness mechanisms to track publisher presence.

        :param handler: Optional handler for receiving publisher detection events
        :param history: If True, query for historical publisher information
        :return: A Subscriber for monitoring publisher presence
        """

    @overload
    def detect_publishers(
        self, handler: _PythonHandler[Sample, _H], *, history: bool | None = None
    ) -> Subscriber[_H]:
        """Declare a listener to detect matching publishers.

        Only :class:`AdvancedPublisher` instances that enable ``publisher_detection``
        can be detected. This uses liveliness mechanisms to track publisher presence.

        :param handler: Handler tuple (callback, handler) for receiving publisher detection events
        :param history: If True, query for historical publisher information
        :return: A Subscriber for monitoring publisher presence
        """

    @overload
    def detect_publishers(
        self, handler: _PythonCallback[Sample], *, history: bool | None = None
    ) -> Subscriber[None]:
        """Declare a listener to detect matching publishers.

        Only :class:`AdvancedPublisher` instances that enable ``publisher_detection``
        can be detected. This uses liveliness mechanisms to track publisher presence.

        :param handler: Callback function for receiving publisher detection events
        :param history: If True, query for historical publisher information
        :return: A Subscriber for monitoring publisher presence
        """

    def undeclare(self):
        """Undeclare the AdvancedSubscriber.

        This removes the subscription and informs the network that resources
        for this subscriber can be released.
        """

    @overload
    def try_recv(self: AdvancedSubscriber[Handler[Sample]]) -> Sample | None:
        """Try to receive a sample without blocking.

        Only available when using a DefaultHandler, FifoChannel, or RingChannel.

        :return: A Sample if one is available, None otherwise
        """

    @overload
    def try_recv(self) -> Never:
        """Try to receive a sample without blocking.

        This method is only available when using channel-based handlers.
        """

    @overload
    def recv(self: AdvancedSubscriber[Handler[Sample]]) -> Sample:
        """Receive a sample, blocking until one is available.

        Only available when using a DefaultHandler, FifoChannel, or RingChannel.

        :return: The received Sample
        """

    @overload
    def recv(self) -> Never:
        """Receive a sample, blocking until one is available.

        This method is only available when using channel-based handlers.
        """

    @overload
    def __iter__(self: AdvancedSubscriber[Handler[Sample]]) -> Handler[Sample]:
        """Iterate over received samples.

        Only available when using a DefaultHandler, FifoChannel, or RingChannel.

        :return: An iterator over received samples
        """

    @overload
    def __iter__(self) -> Never:
        """Iterate over received samples.

        This method is only available when using channel-based handlers.
        """

@_unstable
@final
class CacheConfig:
    """
    :param max_samples: specify how many samples to keep for each resource, default to 1
    :param replies_config: the QoS to apply to replies
    """

    def __new__(
        cls,
        max_samples: int | None = None,
        *,
        replies_config: RepliesConfig | None = None,
    ) -> Self: ...

@_unstable
@final
class HistoryConfig:
    """
    :param detect_late_publishers: enable detection of late joiner publishers and query for their historical data;
        late joiner detection can only be achieved for `AdvancedPublisher` that enable `publisher_detection`
        history can only be retransmitted by `AdvancedPublisher` that enable `cache`
    :param max_samples: specify how many samples to query for each resource
    :param max_age: specify the maximum age of samples to query in seconds
    """

    def __new__(
        cls,
        *,
        detect_late_publishers: bool | None = None,
        max_samples: int | None = None,
        max_age: float | int | None = None,
    ) -> Self: ...

@_unstable
@final
class Miss:
    @property
    def source(self) -> EntityGlobalId:
        """The source of missed samples."""

    @property
    def nb(self) -> int:
        """The number of missed samples."""

@_unstable
@final
class MissDetectionConfig:
    """
    :param heartbeat: period in seconds, allow last sample miss detection through periodic heartbeat;
        periodically send the last published Sample's sequence number to allow last sample recovery.
        `AdvancedSubscriber can only recover the last sample with the `heartbeat` option enabled.

        **This option can not be enabled simultaneously with `sporadic_heartbeat`.**

    :param sporadic_heartbeat: period in seconds, allow last sample miss detection through sporadic heartbeat;
        each period, the last published Sample's sequence number is sent with `CongestionControl.Block` but only if
        it has changed since the last period.
        `AdvancedSubscriber can only recover the last sample with the `heartbeat` option enabled.

        **This option can not be enabled simultaneously with `heartbeat`.**
    """

    def __new__(
        cls, *, heartbeat: float | int | None, sporadic_heartbeat: float | int | None
    ) -> Self: ...

@_unstable
@final
class RecoveryConfig:
    """
    :param periodic_queries: enable periodic queries for not yet received Samples and specify their period;
        it allows retrieving the last Sample(s) if the last Sample(s) is/are lost,
        so it is useful for sporadic publications but useless for periodic publications
        with a period smaller or equal to this period.
        Retransmission can only be achieved by `AdvancedPublisher` that enable `cache` and `sample_miss_detection`.

        **This option can not be enabled simultaneously with `heartbeat`.**

    :param heartbeat: subscribe to heartbeats of `AdvancedPublisher`;
        it allows receiving the last published Sample's sequence number and check for misses.
        Heartbeat subscriber must be paired with `AdvancedPublishers` that enable `cache` and
        `sample_miss_detection` with `heartbeat` or `sporadic_heartbeat`.

        **This option can not be enabled simultaneously with `periodic_queries`.**
    """

    def __new__(
        cls, *, periodic_queries: float | int | None, heartbeat: Literal[True] | None
    ) -> Self: ...

@_unstable
@final
class RepliesConfig:
    def __new__(
        cls,
        *,
        congestion_control: CongestionControl | None = None,
        priority: Priority | None = None,
        express: bool | None = None,
    ) -> Self: ...

@_unstable
@final
class SampleMissListener(Generic[_H]):
    def undeclare(self): ...
    @overload
    def try_recv(self: SampleMissListener[Handler[Miss]]) -> Miss | None: ...
    @overload
    def try_recv(self) -> Never: ...
    @overload
    def recv(self: SampleMissListener[Handler[Miss]]) -> Miss: ...
    @overload
    def recv(self) -> Never: ...
    @overload
    def __iter__(self: SampleMissListener[Handler[Miss]]) -> Handler[Miss]: ...
    @overload
    def __iter__(self) -> Never: ...

@_unstable
def declare_advanced_publisher(
    session: Session,
    key_expr: _IntoKeyExpr,
    *,
    encoding: _IntoEncoding | None = None,
    congestion_control: CongestionControl | None = None,
    priority: Priority | None = None,
    express: bool | None = None,
    reliability: Reliability | None = None,
    allowed_destination: Locality | None = None,
    cache: CacheConfig | None = None,
    sample_miss_detection: MissDetectionConfig | None = None,
    publisher_detection: bool | None = None,
) -> AdvancedPublisher:
    """Create an AdvancedPublisher for the given key expression."""

@_unstable
@overload
def declare_advanced_subscriber(
    session: Session,
    key_expr: _IntoKeyExpr,
    handler: _RustHandler[Sample] | None = None,
    *,
    allowed_origin: Locality | None = None,
    history: HistoryConfig | None = None,
    recovery: RecoveryConfig | None = None,
    subscriber_detection: bool | None = None,
) -> AdvancedSubscriber[Handler[Sample]]:
    """Create an AdvancedSubscriber for the given key expression."""

@_unstable
@overload
def declare_advanced_subscriber(
    session: Session,
    key_expr: _IntoKeyExpr,
    handler: _PythonHandler[Sample, _H],
    *,
    allowed_origin: Locality | None = None,
    history: HistoryConfig | None = None,
    recovery: RecoveryConfig | None = None,
    subscriber_detection: bool | None = None,
) -> AdvancedSubscriber[_H]:
    """Create an AdvancedSubscriber for the given key expression."""

@_unstable
@overload
def declare_advanced_subscriber(
    session: Session,
    key_expr: _IntoKeyExpr,
    handler: _PythonCallback[Sample],
    *,
    allowed_origin: Locality | None = None,
    history: HistoryConfig | None = None,
    recovery: RecoveryConfig | None = None,
    subscriber_detection: bool | None = None,
) -> AdvancedSubscriber[None]:
    """Create an AdvancedSubscriber for the given key expression."""
