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

    Advanced publishers are created via :func:`declare_advanced_publisher` and
    works alongside :class:`AdvancedSubscriber`. Its features include:

    * **Caching** - Store last published samples for retrieval via subscriber history mechanisms.
      Configure via :class:`CacheConfig`.

    * **Sample miss detection** - Identify gaps in publications to detect missed samples.
      Configure via :class:`MissDetectionConfig`. Subscribers can monitor misses via :meth:`AdvancedSubscriber.sample_miss_listener`.

    * **Publisher detection** - Assert presence through liveliness tokens.
      Subscribers can detect publishers via :meth:`AdvancedSubscriber.detect_publishers`.
    """

    def __enter__(self) -> Self: ...
    def __exit__(self, *_args, **_kwargs): ...
    @_unstable
    @property
    def id(self) -> EntityGlobalId:
        """The globally unique id of this AdvancedPublisher. See :meth:`zenoh.Publisher.id`."""

    @property
    def key_expr(self) -> KeyExpr:
        """The key expression this AdvancedPublisher publishes to. See :meth:`zenoh.Publisher.key_expr`."""

    @property
    def encoding(self) -> Encoding:
        """The encoding used for published data. See :meth:`zenoh.Publisher.encoding`."""

    @property
    def congestion_control(self) -> CongestionControl:
        """The congestion control policy applied to published data.
        See :meth:`zenoh.Publisher.congestion_control`.
        """

    @property
    def priority(self) -> Priority:
        """The priority level of published data. See :meth:`zenoh.Publisher.priority`."""

    def put(
        self,
        payload: _IntoZBytes,
        *,
        encoding: _IntoEncoding | None = None,
        attachment: _IntoZBytes | None = None,
        timestamp: Timestamp | None = None,
    ):
        """Publish data to the key expression. See :meth:`zenoh.Publisher.put`."""

    def delete(
        self,
        *,
        attachment: _IntoZBytes | None = None,
        timestamp: Timestamp | None = None,
    ):
        """Delete the value associated with the key expression. See :meth:`zenoh.Publisher.delete`."""

    def undeclare(self):
        """Undeclare the AdvancedPublisher. See :meth:`zenoh.Publisher.undeclare`."""

@_unstable
@final
class AdvancedSubscriber(Generic[_H]):
    """An extension to Subscriber providing advanced functionalities.

    AdvancedSubscriber is created with :func:`declare_advanced_subscriber` and works alongside
    :class:`AdvancedPublisher`. Its features include:

    * missing samples detection using periodic queries or heartbeat subscription
      configurable via :class:`RecoveryConfig`. Notification about missed samples is done
      via :meth:`AdvancedSubscriber.sample_miss_listener`.

    * recovering missing samples with max age, sample count and late joiner detection
      configurable via :class:`HistoryConfig`.

    * matching publishers detection using liveliness mechanisms.
      Use :meth:`AdvancedSubscriber.detect_publishers` to find active publishers.
    """

    def __enter__(self) -> Self: ...
    def __exit__(self, *_args, **_kwargs): ...
    @_unstable
    @property
    def id(self) -> EntityGlobalId:
        """The globally unique id of this AdvancedSubscriber. See :meth:`zenoh.Subscriber.id`."""

    @property
    def key_expr(self) -> KeyExpr:
        """The key expression this AdvancedSubscriber subscribes to. See :meth:`zenoh.Subscriber.key_expr`."""

    @property
    def handler(self) -> _H:
        """The handler used to process received samples. See :meth:`zenoh.Subscriber.handler`."""

    @overload
    def sample_miss_listener(
        self, handler: _RustHandler[Miss] | None = None
    ) -> SampleMissListener[Handler[Miss]]:
        """Declare a listener to detect missed samples.

        Missed samples can only be detected from :class:`AdvancedPublisher` instances
        that enable `sample_miss_detection`. The listener will receive :class:`Miss`
        notifications indicating the source and number of missed samples.
        """

    @overload
    def sample_miss_listener(
        self, handler: _PythonHandler[Miss, _H]
    ) -> SampleMissListener[_H]: ...
    @overload
    def sample_miss_listener(
        self, handler: _PythonCallback[Miss]
    ) -> SampleMissListener[None]: ...
    @overload
    def detect_publishers(
        self,
        handler: _RustHandler[Sample] | None = None,
        *,
        history: bool | None = None,
    ) -> Subscriber[Handler[Sample]]:
        """Declare a listener to detect matching publishers.

        Only :class:`AdvancedPublisher` instances that enable `publisher_detection`
        in the :meth:`declare_advanced_publisher` can be detected. This uses
        :ref:`liveliness` to track publisher presence.

        :param handler: Optional handler for receiving publisher detection events.
        :param history: If `True`, the already present publishers will be reported upon declaration. Uses `history` feature of :meth:`zenoh.Liveliness.declare_subscriber`.
        """

    @overload
    def detect_publishers(
        self, handler: _PythonHandler[Sample, _H], *, history: bool | None = None
    ) -> Subscriber[_H]: ...
    @overload
    def detect_publishers(
        self, handler: _PythonCallback[Sample], *, history: bool | None = None
    ) -> Subscriber[None]: ...
    def undeclare(self):
        """Undeclare the AdvancedSubscriber. See :meth:`zenoh.Subscriber.undeclare`."""

    def try_recv(self: AdvancedSubscriber[Handler[Sample]]) -> Sample | None:
        """Try to receive a sample without blocking. See :meth:`zenoh.Subscriber.try_recv`."""

    def recv(self: AdvancedSubscriber[Handler[Sample]]) -> Sample:
        """Receive a sample, blocking until one is available. See :meth:`zenoh.Subscriber.recv`."""

    def __iter__(self: AdvancedSubscriber[Handler[Sample]]) -> Handler[Sample]: ...

@_unstable
@final
class CacheConfig:
    """
    Configure caching behavior for an :class:`AdvancedPublisher`.

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
    Configure history retrieval behavior for an :class:`AdvancedSubscriber`.

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
    """Notification about missed samples detected by an :class:`AdvancedSubscriber`.

    A Miss indicates that one or more samples from an :class:`AdvancedPublisher` were not
    received by the subscriber. This can occur due to network congestion, packet loss, or
    when the subscriber cannot keep up with the publication rate.

    Miss detection requires the publisher to enable `sample_miss_detection` in
    :meth:`declare_advanced_publisher` and the subscriber to have a :class:`SampleMissListener`
    via :meth:`AdvancedSubscriber.sample_miss_listener`.
    """

    @property
    def source(self) -> EntityGlobalId:
        """The globally unique identifier of the :class:`AdvancedPublisher` that published the missed samples."""

    @property
    def nb(self) -> int:
        """The number of consecutive samples that were missed from this source."""

@_unstable
@final
class MissDetectionConfig:
    """
    Configure miss detection behavior for an :class:`AdvancedPublisher`.

    :param heartbeat: period in seconds, allow last sample miss detection through periodic heartbeat;
        periodically send the last published :class:`zenoh.Sample`'s sequence number to allow last sample recovery.
        :class:`zenoh.ext.AdvancedSubscriber` can only recover the last sample with the `heartbeat` option enabled.

        **This option can not be enabled simultaneously with `sporadic_heartbeat`.**

    :param sporadic_heartbeat: period in seconds, allow last sample miss detection through sporadic heartbeat;
        each period, the last published :class:`zenoh.Sample`'s sequence number is sent
        with `zenoh.CongestionControl.Block` but only if it has changed since the last period.
        :class:`zenoh.ext.AdvancedSubscriber` can only recover the last sample with the `heartbeat` option enabled.

        **This option can not be enabled simultaneously with `heartbeat`.**
    """

    def __new__(
        cls, *, heartbeat: float | int | None, sporadic_heartbeat: float | int | None
    ) -> Self: ...

@_unstable
@final
class RecoveryConfig:
    """
    Configure recovery behavior for an :class:`AdvancedSubscriber`.

    :param periodic_queries: enable periodic queries for not yet received Samples and specify their period;
        it allows retrieving the last Sample(s) if the last Sample(s) is/are lost,
        so it is useful for sporadic publications but useless for periodic publications
        with a period smaller or equal to this period.
        Retransmission can only be achieved by :class:`AdvancedPublisher` that enable `cache` and `sample_miss_detection`.

        **This option can not be enabled simultaneously with `heartbeat`.**

    :param heartbeat: subscribe to heartbeats of :class:`AdvancedPublisher`;
        it allows receiving the last published Sample's sequence number and check for misses.
        Heartbeat subscriber must be paired with :class:`AdvancedPublisher` that enable `cache` and
        `sample_miss_detection` with `heartbeat` or `sporadic_heartbeat`.

        **This option can not be enabled simultaneously with `periodic_queries`.**
    """

    def __new__(
        cls, *, periodic_queries: float | int | None, heartbeat: Literal[True] | None
    ) -> Self: ...

@_unstable
@final
class RepliesConfig:
    """
    Configure QoS settings for replies from a :class:`AdvancedSubscriber`. Parameter
    in :class:`zenoh.CacheConfig`.
    """

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
    """Listener for detecting missed samples from an :class:`AdvancedSubscriber`.

    Instances are created via :meth:`AdvancedSubscriber.sample_miss_listener`.

    This listener receives :class:`Miss` notifications when gaps are detected in the
    sequence of samples from :class:`AdvancedPublisher` instances. This works for publishers that enable
    `sample_miss_detection` in :meth:`declare_advanced_publisher`.
    """

    def undeclare(self):
        """Undeclare the SampleMissListener."""

    def try_recv(self: SampleMissListener[Handler[Miss]]) -> Miss | None:
        """Try to receive a miss notification without blocking."""

    def recv(self: SampleMissListener[Handler[Miss]]) -> Miss:
        """Receive a miss notification, blocking until one is available."""

    def __iter__(self: SampleMissListener[Handler[Miss]]) -> Handler[Miss]: ...

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
    """Declare an :class:`AdvancedPublisher` for the given key expression."""

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
    """Declare an :class:`AdvancedSubscriber` for the given key expression."""

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
) -> AdvancedSubscriber[_H]: ...
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
) -> AdvancedSubscriber[None]: ...
