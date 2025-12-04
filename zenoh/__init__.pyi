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
from datetime import datetime, timedelta
from enum import Enum, auto
from pathlib import Path
from typing import Any, Generic, Self, TypeVar, final, overload

from . import ext as ext
from . import handlers as handlers
from . import shm as shm
from .handlers import Handler as Handler

_T = TypeVar("_T")
_H = TypeVar("_H")

_RustHandler = (
    handlers.DefaultHandler[_T] | handlers.FifoChannel[_T] | handlers.RingChannel[_T]
)
_PythonCallback = Callable[[_T], Any]
_PythonHandler = tuple[_PythonCallback[_T], _H]

def _unstable(item: _T) -> _T:
    """marker for unstable functionality"""

@final
class ZError(Exception):
    """Exception raised for Zenoh-related errors.

    This exception is raised by various Zenoh operations when they encounter errors,
    such as invalid :class:`KeyExpr` instances or connection failures.

    To handle ZError, wrap Zenoh operations in try-except blocks:

    .. code-block:: python

        try:
            ke = KeyExpr("invalid/key")
        except ZError as e:
            print(f"Error: {e}")  # Get error message

    The error message can be accessed via str(e) or by printing the exception directly.
    """

    ...

@_unstable
@final
class CancellationToken:
    """Cancellation token that can be used for interrupting GET queries."""

    def __new__(cls) -> Self: ...
    def cancel(self):
        """Interrupt all associated GET queries. If the direct query callback is being executed,
        the call blocks until execution of callback finishes and its corresponding drop method returns (if any).

        Once token is cancelled, all new associated GET queries will cancel automatically.
        """

    @property
    def is_cancelled(self) -> bool:
        """Return true if token was cancelled, false otherwise."""

@final
class Config:
    """The main configuration structure for Zenoh.

    Zenoh configuration can be loaded from various sources:

    - From a specific file path using :meth:`from_file`.
    - From a file specified by the ZENOH_CONFIG environment variable using :meth:`from_env`.
    - From a JSON5 string using :meth:`from_json5`.

    Configuration values can be retrieved as JSON using :meth:`get_json` and modified using :meth:`insert_json5`.

    For detailed format information, see: https://docs.rs/zenoh/latest/zenoh/config/struct.Config.html
    """

    def __new__(cls) -> Self: ...
    @classmethod
    def from_env(cls) -> Self:
        """Load configuration from the file path specified in the ZENOH_CONFIG environment variable."""

    @classmethod
    def from_file(cls, path: str | Path) -> Self:
        """Load configuration from the file at path."""

    @classmethod
    def from_json5(cls, json: str) -> Self:
        """Load configuration from the JSON5 string json."""

    def get_json(self, key: str) -> Any:
        """Returns a JSON string containing the configuration at key."""

    def insert_json5(self, key: str, value: Any):
        """Inserts configuration value value at key."""

    def __str__(self) -> str:
        """Returns a string representation of the configuration."""

@final
class CongestionControl(Enum):
    """Congestion control strategy.

    This parameter controls how messages are handled when a node's transmission queue is full.
    Set this when declaring publishers/queriers or in put, get, delete, and reply operations to specify congestion behavior.
    It can also be retrieved from :class:`Sample` and :class:`Publisher` objects.

    See also:
        - Parameters in: :meth:`Session.declare_publisher`, :meth:`Session.declare_querier`,
          :meth:`Session.put`, :meth:`Session.delete`, :meth:`Session.get`,
          :meth:`Query.reply`, :meth:`Query.reply_del`
        - Properties in: :attr:`Sample.congestion_control`, :attr:`Publisher.congestion_control`
    """

    DROP = auto()
    BLOCK = auto()
    BLOCK_FIRST = _unstable(auto())
    DEFAULT = DROP

CongestionControl.DROP.__doc__ = """When transmitting a message in a node with a full queue, the node may drop the message."""
CongestionControl.BLOCK.__doc__ = """When transmitting a message in a node with a full queue, the node will wait for queue to progress."""
CongestionControl.BLOCK_FIRST.__doc__ = """When transmitting a message in a node with a full queue, the node will wait for queue to progress, but only for the first message sent with this strategy; other messages will be dropped."""

@final
class ConsolidationMode(Enum):
    """The kind of consolidation to apply to a query.

    Consolidation determines how multiple samples for the same key are handled
    during query operations, balancing between latency and bandwidth efficiency.

    See also:
        - Parameters in: :meth:`Session.get`, :meth:`Session.declare_querier`
    """

    AUTO = auto()
    NONE = auto()
    MONOTONIC = auto()
    LATEST = auto()
    DEFAULT = AUTO

ConsolidationMode.AUTO.__doc__ = (
    """Apply automatic consolidation based on queryable's preferences."""
)
ConsolidationMode.NONE.__doc__ = """No consolidation applied: multiple samples may be received for the same key-timestamp."""
ConsolidationMode.MONOTONIC.__doc__ = """Monotonic consolidation immediately forwards samples, except if one with an equal or more recent timestamp has already been sent with the same key.

This optimizes latency while potentially reducing bandwidth. Note that this doesn't cause re-ordering, but drops the samples for which a more recent timestamp has already been observed with the same key.
"""
ConsolidationMode.LATEST.__doc__ = """Holds back samples to only send the set of samples that had the highest timestamp for their key."""

@final
class Encoding:
    """Encoding information for Zenoh payloads.

    Represents how data should be interpreted by the application, similar to HTTP Content-type.
    Encodings use MIME-like format: ``type/subtype[;schema]``.

    Predefined class attributes are provided for common encodings (e.g., :attr:`ZENOH_BYTES`,
    :attr:`APPLICATION_JSON`). Using these is more efficient than custom strings.

    See also: :ref:`encoding`
    """

    def __new__(cls, encoding: str | None = None) -> Self: ...
    def with_schema(self, schema: str) -> Self:
        """Set a schema to this encoding. Zenoh does not define what a schema is and its semantics are left to the implementer. E.g. a common schema for text/plain encoding is utf-8."""

    def __eq__(self, other: _IntoEncoding) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...

    ZENOH_BYTES: Self
    """Just some bytes.
    
    Constant alias for string: "zenoh/bytes"."""
    ZENOH_STRING: Self
    """A UTF-8 string.

    Constant alias for string: "zenoh/string"."""
    ZENOH_SERIALIZED: Self
    """Zenoh serialized data.

    Constant alias for string: "zenoh/serialized"."""
    APPLICATION_OCTET_STREAM: Self
    """An application-specific stream of bytes.

    Constant alias for string: "application/octet-stream"."""
    TEXT_PLAIN: Self
    """A textual file.

    Constant alias for string: "text/plain"."""
    APPLICATION_JSON: Self
    """JSON data intended to be consumed by an application.

    Constant alias for string: "application/json"."""
    TEXT_JSON: Self
    """JSON data intended to be human readable.

    Constant alias for string: "text/json"."""
    APPLICATION_CDR: Self
    """A Common Data Representation (CDR)-encoded data.

    Constant alias for string: "application/cdr"""
    APPLICATION_CBOR: Self
    """A Concise Binary Object Representation (CBOR)-encoded data.

    Constant alias for string: "application/cbor"."""
    APPLICATION_YAML: Self
    """YAML data intended to be consumed by an application.

    Constant alias for string: "application/yaml"."""
    TEXT_YAML: Self
    """YAML data intended to be human readable.

    Constant alias for string: "text/yaml"."""
    TEXT_JSON5: Self
    """JSON5 encoded data intended to be human readable.

    Constant alias for string: "text/json5"."""
    APPLICATION_PYTHON_SERIALIZED_OBJECT: Self
    """A Python object serialized using pickle.

    Constant alias for string: "application/python-serialized-object"."""
    APPLICATION_PROTOBUF: Self
    """An application-specific protobuf-encoded data.

    Constant alias for string: "application/protobuf"."""
    APPLICATION_JAVA_SERIALIZED_OBJECT: Self
    """A Java serialized object.

    Constant alias for string: "application/java-serialized-object"."""
    APPLICATION_OPENMETRICS_TEXT: Self
    """An openmetrics text data representation, commonly used by Prometheus.

    Constant alias for string: "application/openmetrics-text"."""
    IMAGE_PNG: Self
    """A Portable Network Graphics (PNG) image.

    Constant alias for string: "image/png"."""
    IMAGE_JPEG: Self
    """A Joint Photographic Experts Group (JPEG) image.

    Constant alias for string: "image/jpeg"."""
    IMAGE_GIF: Self
    """A Graphics Interchange Format (GIF) image.

    Constant alias for string: "image/gif"."""
    IMAGE_BMP: Self
    """A BitMap (BMP) image.

    Constant alias for string: "image/bmp"."""
    IMAGE_WEBP: Self
    """A Web Protable (WebP) image.

    Constant alias for string: "image/webp"""
    APPLICATION_XML: Self
    """An XML file intended to be consumed by an application.

    Constant alias for string: "application/xml"."""
    APPLICATION_X_WWW_FORM_URLENCODED: Self
    """An encoded a list of tuples, each consisting of a name and a value.

    Constant alias for string: "application/x-www-form-urlencoded"."""
    TEXT_HTML: Self
    """An HTML file.

    Constant alias for string: "text/html"."""
    TEXT_XML: Self
    """An XML file that is human readable.

    Constant alias for string: "text/xml"."""
    TEXT_CSS: Self
    """A CSS file.

    Constant alias for string: "text/css"."""
    TEXT_JAVASCRIPT: Self
    """A JavaScript file.

    Constant alias for string: "text/javascript"."""
    TEXT_MARKDOWN: Self
    """A MarkDown file.

    Constant alias for string: "text/markdown"."""
    TEXT_CSV: Self
    """A CSV file.

    Constant alias for string: "text/csv"."""
    APPLICATION_SQL: Self
    """An application-specific SQL query.

    Constant alias for string: "application/sql"."""
    APPLICATION_COAP_PAYLOAD: Self
    """Constrained Application Protocol (CoAP) data intended for CoAP-to-HTTP and HTTP-to-CoAP proxies.

    Constant alias for string: "application/coap-payload"."""
    APPLICATION_JSON_PATCH_JSON: Self
    """Defines a JSON document structure for expressing a sequence of operations to apply to a JSON document.

    Constant alias for string: "application/json-patch+json"."""
    APPLICATION_JSON_SEQ: Self
    """A JSON text sequence consists of any number of JSON texts, all encoded in UTF-8.

    Constant alias for string: "application/json-seq"."""
    APPLICATION_JSONPATH: Self
    """A JSONPath defines a string syntax for selecting and extracting JSON values from within a given JSON value.

    Constant alias for string: "application/jsonpath"."""
    APPLICATION_JWT: Self
    """A JSON Web Token (JWT).

    Constant alias for string: "application/jwt"."""
    APPLICATION_MP4: Self
    """An application-specific MPEG-4 encoded data, either audio or video.

    Constant alias for string: "application/mp4"."""
    APPLICATION_SOAP_XML: Self
    """A SOAP 1.2 message serialized as XML 1.0.

    Constant alias for string: "application/soap+xml"."""
    APPLICATION_YANG: Self
    """A YANG-encoded data commonly used by the Network Configuration Protocol (NETCONF).

    Constant alias for string: "application/yang"."""
    AUDIO_AAC: Self
    """A MPEG-4 Advanced Audio Coding (AAC) media.

    Constant alias for string: "audio/aac"."""
    AUDIO_FLAC: Self
    """A Free Lossless Audio Codec (FLAC) media.

    Constant alias for string: "audio/flac"."""
    AUDIO_MP4: Self
    """An audio codec defined in MPEG-1, MPEG-2, MPEG-4, or registered at the MP4 registration authority.

    Constant alias for string: "audio/mp4"."""
    AUDIO_OGG: Self
    """An Ogg-encapsulated audio stream.

    Constant alias for string: "audio/ogg"."""
    AUDIO_VORBIS: Self
    """A Vorbis-encoded audio stream.

    Constant alias for string: "audio/vorbis"."""
    VIDEO_H261: Self
    """An h261-encoded video stream.

    Constant alias for string: "video/h261"."""
    VIDEO_H263: Self
    """An h263-encoded video stream.

    Constant alias for string: "video/h263"."""
    VIDEO_H264: Self
    """An h264-encoded video stream.

    Constant alias for string: "video/h264"."""
    VIDEO_H265: Self
    """An h265-encoded video stream.

    Constant alias for string: "video/h265"."""
    VIDEO_H266: Self
    """An h266-encoded video stream.

    Constant alias for string: "video/h266"."""
    VIDEO_MP4: Self
    """A video codec defined in MPEG-1, MPEG-2, MPEG-4, or registered at the MP4 registration authority.

    Constant alias for string: "video/mp4"."""
    VIDEO_OGG: Self
    """An Ogg-encapsulated video stream.

    Constant alias for string: "video/ogg"."""
    VIDEO_RAW: Self
    """An uncompressed, studio-quality video stream.

    Constant alias for string: "video/raw"."""
    VIDEO_VP8: Self
    """A VP8-encoded video stream.

    Constant alias for string: "video/vp8"."""
    VIDEO_VP9: Self
    """A VP9-encoded video stream.

    Constant alias for string: "video/vp9"."""

_IntoEncoding = Encoding | str

EntityId = int

@_unstable
@final
class EntityGlobalId:
    """
    The ID globally identifying an entity in a zenoh system.
    """

    @property
    def zid(self) -> ZenohId:
        """Returns the `ZenohId`, i.e. the Zenoh session, this ID is associated to."""

    @property
    def eid(self) -> EntityId:
        """Returns the `EntityId` used to identify the entity in a Zenoh session."""

@final
class Hello:
    """A zenoh Hello message.

    A Hello message is returned in the ::ref::`scouting` process for each found Zenoh node on the network. It contains information about the node's
    identity and its addresses in `locators <https://docs.rs/zenoh/latest/zenoh/config/struct.Locator.html>`_ format.
    """

    @property
    def whatami(self) -> WhatAmI:
        """Get the `WhatAmI` type of the Zenoh node."""

    @property
    def zid(self) -> ZenohId:
        """Get the `ZenohId` of the Zenoh node."""

    @property
    def locators(self) -> list[str]:
        """Get the locators (network addresses) of the Zenoh node."""

    def __str__(self) -> str:
        """Returns a string representation of the Hello message."""

@final
class KeyExpr:
    """Key expressions are Zenoh's address space for data routing and selection.

    A KeyExpr represents a set of keys in a hierarchical namespace using slash-separated paths
    with wildcard support (``*`` and ``**``). It may carry optimizations for use with a Session that has declared it.

    For detailed information about key expressions, see :ref:`key-expressions`.
    """

    def __new__(cls, key_expr: str) -> Self:
        """Creates a new KeyExpr from a string.
        Raises :exc:`ZError` if the key_expr is not a valid key expression.
        """

    @classmethod
    def autocanonize(cls, key_expr: str) -> Self:
        """`Canonizes <https://github.com/eclipse-zenoh/roadmap/blob/main/rfcs/ALL/Key%20Expressions.md#canon-forms>`_ the passed value before returning it as a KeyExpr.
        Raises :exc:`ZError` if the passed value isn't a valid key expression despite canonization.
        """

    def intersects(self, other: _IntoKeyExpr) -> bool:
        """Returns true if the keyexprs intersect, i.e. there exists at least one key which is contained in both of the sets defined by self and other."""

    def includes(self, other: _IntoKeyExpr) -> bool:
        """Returns true if self includes other, i.e. the set defined by self contains every key belonging to the set defined by other."""

    @_unstable
    def relation_to(self, other: _IntoKeyExpr) -> SetIntersectionLevel:
        """Returns the relation between self and other from self's point of view (:attr:`SetIntersectionLevel.INCLUDES` signifies that self includes other).
        Note that this is slower than :meth:`intersects` and :meth:`includes`, so you should favor these methods for most applications.
        """

    def join(self, other: str) -> KeyExpr:
        """Joins both sides, inserting a / in between them.
        This should be your prefered method when concatenating path segments."""

    def concat(self, other: str) -> KeyExpr:
        """Performs string concatenation and returns the result as a KeyExpr.
        Raises :exc:`ZError` if the result is not a valid key expression.
        You should probably prefer :meth:`join` as Zenoh may then take advantage of the hierachical separation it inserts.
        """

    def __str__(self) -> str: ...

_IntoKeyExpr = KeyExpr | str

@final
class Liveliness:
    """A structure with functions to declare a :class:`LivelinessToken`, query existing :class:`LivelinessToken`\\s and subscribe to liveliness changes.

    A :class:`LivelinessToken` is a token whose liveliness is tied to the Zenoh :class:`Session` and can be monitored by remote applications.

    The Liveliness structure can be obtained with the :meth:`Session.liveliness` method of the :class:`Session` class.

    For more information, see :ref:`liveliness`.
    """

    def declare_token(self, key_expr: _IntoKeyExpr) -> LivelinessToken:
        """Create a :class:`LivelinessToken` for the given key expression."""

    @overload
    def get(
        self,
        key_expr: _IntoKeyExpr,
        handler: _RustHandler[Reply] | None = None,
        *,
        timeout: float | int | None = None,
        cancellation_token: CancellationToken | None = None,
    ) -> Handler[Reply]:
        """Query :class:`LivelinessToken` with matching key expressions."""

    @overload
    def get(
        self,
        key_expr: _IntoKeyExpr,
        handler: _PythonHandler[Reply, _H],
        *,
        timeout: float | int | None = None,
        cancellation_token: CancellationToken | None = None,
    ) -> _H:
        """Query :class:`LivelinessToken` with matching key expressions."""

    @overload
    def get(
        self,
        key_expr: _IntoKeyExpr,
        handler: _PythonCallback[Reply],
        *,
        timeout: float | int | None = None,
        cancellation_token: CancellationToken | None = None,
    ) -> None:
        """Query :class:`LivelinessToken` with matching key expressions."""

    @overload
    def declare_subscriber(
        self,
        key_expr: _IntoKeyExpr,
        handler: _RustHandler[Sample] | None = None,
        *,
        history: bool | None = None,
    ) -> Subscriber[Handler[Sample]]:
        """Create a :class:`Subscriber` for liveliness changes matching the given key expression.

        :param key_expr: The :class:`LivelinessToken` key expression.
        :param handler: The handler for receiving liveliness samples (see :ref:`channels-and-callbacks`).
        :param history: If True, the already present liveliness tokens will be reported upon declaration.
        """

    @overload
    def declare_subscriber(
        self,
        key_expr: _IntoKeyExpr,
        handler: _PythonHandler[Sample, _H],
        *,
        history: bool | None = None,
    ) -> Subscriber[_H]: ...
    @overload
    def declare_subscriber(
        self,
        key_expr: _IntoKeyExpr,
        handler: _PythonCallback[Sample],
        *,
        history: bool | None = None,
    ) -> Subscriber[None]: ...

@final
class LivelinessToken:
    """A token whose liveliness is tied to the Zenoh :class:`Session` and can be monitored by
    remote applications using the :class:`Liveliness` structure. The token is declared using
    :meth:`Liveliness.declare_token` with a specific key expression.
    """

    def __enter__(self) -> Self: ...
    def __exit__(self, *_args, **_kwargs): ...
    def undeclare(self):
        """Undeclare the :class:`LivelinessToken`."""

@final
class NTP64:
    """A NTP 64-bits format as specified in RFC-5909 <https://tools.ietf.org/html/rfc5905#section-6>

    The 1st 32-bits part is the number of second since the EPOCH of the physical clock,
    and the 2nd 32-bits part is the fraction of second."""

    def __new__(cls, seconds: int, nanos: int) -> Self: ...
    def as_secs_f64(self) -> float:
        """Returns this NTP64 as a f64 in seconds.

        The integer part of the f64 is the NTP64's Seconds part.
        The decimal part of the f64 is the result of a division of NTP64's Fraction part divided by 2^32.
        Considering the probable large number of Seconds (for a time relative to UNIX_EPOCH), the precision of the resulting f64 might be in the order of microseconds.
        Therefore, it should not be used for comparison. Directly comparing [NTP64] objects is preferable.
        """

    def as_secs(self) -> int:
        """Returns the 32-bits seconds part."""

    def as_nanos(self) -> int:
        """Returns the total duration converted to nanoseconds."""

    def subsec_nanos(self) -> int:
        """Returns the 32-bits fraction of second part converted to nanoseconds."""

    def to_datetime(self) -> datetime:
        """Returns this NTP64 as a datetime."""

    def to_string_rfc3339_lossy(self) -> str:
        """Convert to a RFC3339 time representation with nanoseconds precision.

        e.g.: `"2024-07-01T13:51:12.129693000Z"``"""

    @classmethod
    def parse_rfc3339(cls, s: str) -> Self:
        """Parse a RFC3339 time representation into a NTP64."""

    def __eq__(self, other: Any) -> bool: ...
    def __ge__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...

@final
class Parameters:
    """A collection of key-value parameters used in query operations.

    Parameters allow attaching additional metadata to queries.
    They can be constructed from dictionaries, strings, or built programmatically.
    When combined with a key expression, they form a :class:`Selector` for query operations.

    See also: :ref:`query-parameters`
    """

    def __new__(cls, parameters: dict[str, str] | str | None = None): ...
    def is_empty(self) -> bool:
        """Returns true if properties does not contain anything."""

    def get(self, key: str, default: str | None = None) -> str | None:
        """Returns the value corresponding to the key."""

    def values(self, key: str) -> list[str]:
        """Returns the list of values corresponding to the key."""

    def insert(self, key: str, value: str):
        """Inserts a key-value pair into the map. If the map did not have this key present, None` is returned. If the map did have this key present, the value is updated, and the old value is returned."""

    def remove(self, key: str):
        """Removes a key from the map, returning the value at the key if the key was previously in the properties."""

    def extend(self, parameters: _IntoParameters):
        """Extend these properties with other properties."""

    def is_ordered(self) -> bool:
        """Returns `true` if all keys are sorted in alphabetical order."""

    def __bool__(self) -> bool: ...
    def __contains__(self, item: str) -> bool: ...
    def __getitem__(self, item: str) -> str | None: ...
    def __iter__(self) -> list[tuple[str, str]]: ...
    def __str__(self) -> str: ...

_IntoParameters = Parameters | dict[str, str] | str

@final
class Priority(Enum):
    """The priority of Zenoh messages.

    Priority determines the transmission priority of messages, with higher priority messages
    being delivered before lower priority ones when network congestion occurs.

    See also: https://docs.rs/zenoh/latest/zenoh/qos/enum.Priority.html

    Used in:
        - :meth:`Query.reply`
        - :meth:`Query.reply_del`
        - :meth:`Session.put`
        - :meth:`Session.delete`
        - :meth:`Session.get`
    """

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

@final
class Publisher:
    """A publisher that allows sending data through a stream.

    Publishers are automatically undeclared when dropped.

    A publisher is created using :meth:`zenoh.Session.declare_publisher` and is used to publish
    data to :class:`Subscriber` instances matching the publisher's key expression.

    Publishers can declare :class:`MatchingListener` instances to monitor if subscribers
    matching the publisher's key expression are present in the network.

    For more information about publish/subscribe operations, see :ref:`publish-subscribe`.
    """

    def __enter__(self) -> Self: ...
    def __exit__(self, *_args, **_kwargs): ...
    @_unstable
    @property
    def id(self) -> EntityGlobalId:
        """The global ID of this publisher."""

    @property
    def key_expr(self) -> KeyExpr:
        """The key expression this publisher publishes to."""

    @property
    def encoding(self) -> Encoding:
        """The encoding used when publishing data."""

    @property
    def congestion_control(self) -> CongestionControl:
        """The congestion control strategy applied when routing data."""

    @property
    def priority(self) -> Priority:
        """The priority of the published data."""

    @property
    @_unstable
    def reliability(self) -> Reliability:
        """The reliability applied when routing data."""

    @property
    def matching_status(self) -> bool:
        """Whether there are subscribers matching this publisher's key expression."""

    def put(
        self,
        payload: _IntoZBytes,
        *,
        encoding: _IntoEncoding | None = None,
        attachment: _IntoZBytes | None = None,
        timestamp: Timestamp | None = None,
        source_info: SourceInfo | None = None,
    ):
        """Publish data to :class:`Subscriber` instances matching this publisher's key expression.

        Subscribers will receive the data as a :class:`zenoh.Sample` with
        :attr:`zenoh.SampleKind.PUT` kind.
        """

    def delete(
        self,
        *,
        attachment: _IntoZBytes | None = None,
        timestamp: Timestamp | None = None,
        source_info: SourceInfo | None = None,
    ):
        """Declare that data associated with this publisher's key expression is deleted.

        :class:`Subscriber` instances will receive a :class:`zenoh.Sample` with :attr:`zenoh.SampleKind.DELETE` kind,
        indicating that the data is no longer associated with the key expression.
        """

    def undeclare(self):
        """Undeclare the publisher, informing the network that it needn't optimize publications for its key expression anymore."""

    @overload
    def declare_matching_listener(
        self, handler: _RustHandler[MatchingStatus] | None = None
    ) -> MatchingListener[Handler[MatchingStatus]]:
        """Create a :class:`MatchingListener`. It will send notifications each time the matching status of this publisher changes."""

    @overload
    def declare_matching_listener(
        self, handler: _PythonHandler[MatchingStatus, _H]
    ) -> MatchingListener[_H]:
        """Create a :class:`MatchingListener`. It will send notifications each time the matching status of this publisher changes."""

    @overload
    def declare_matching_listener(
        self, handler: _PythonCallback[MatchingStatus]
    ) -> MatchingListener[None]:
        """Create a :class:`MatchingListener`. It will send notifications each time the matching status of this publisher changes."""

@final
class Query:
    """A request received by a :class:`Queryable`.

    It contains the key expression, parameters, payload, and attachment sent by a querier
    via :meth:`Session.get` or :meth:`Querier.get`. Use its methods to send replies.

    .. note::
       The Query's :attr:`key_expr` is **not** the key expression which should be used as the parameter
       of :meth:`reply`, because it may contain globs. The :class:`Queryable`'s key expression
       is the one that should be used.

       This parameter is not set automatically because :class:`Queryable` itself may serve
       glob key expressions and send replies on different concrete key expressions matching
       this glob. For example, a :class:`Queryable` serving ``foo/*`` may receive a :class:`Query`
       with key expression ``foo/bar`` and another one with ``foo/baz``, and it should reply
       respectively on ``foo/bar`` and ``foo/baz``.

    .. note::
        By default, queries only accept replies whose key expression intersects with the query's.
        I.e. it's not possible to send reply with key expression ``foo/bar`` to a query with
        key expression ``baz/*``.
        The query may contain special unstable parameter ``_anyke`` which enables disjoint replies.
        See the :class:`Selector` documentation for more information about this parameter.

    See :ref:`query-reply` for more information on the query/reply paradigm.
    """

    def __enter__(self) -> Self: ...
    def __exit__(self, *_args, **_kwargs): ...
    @property
    def selector(self) -> Selector:
        """The full :class:`Selector` of this query."""

    @property
    def key_expr(self) -> KeyExpr:
        """The key expression part of this query."""

    @property
    def parameters(self) -> Parameters:
        """The selector parameters of this query."""

    @property
    def payload(self) -> ZBytes | None:
        """The payload of this query, if any."""

    @property
    def encoding(self) -> Encoding | None:
        """The encoding of this query's payload, if any."""

    @property
    def attachment(self) -> ZBytes | None:
        """The attachment of this query, if any."""

    def reply(
        self,
        key_expr: _IntoKeyExpr,
        payload: _IntoZBytes,
        *,
        encoding: _IntoEncoding | None = None,
        congestion_control: CongestionControl | None = None,
        priority: Priority | None = None,
        express: bool | None = None,
        attachment: _IntoZBytes | None = None,
        timestamp: Timestamp | None = None,
    ):
        """Sends a :class:`Sample` of kind :attr:`SampleKind.PUT` as a reply to this query.

        .. note::
           See the class documentation for important details about which key expression to use for replies.
        """

    def reply_err(self, payload: _IntoZBytes, *, encoding: _IntoEncoding | None = None):
        """Sends a :class:`ReplyError` as a reply to this query."""

    def reply_del(
        self,
        key_expr: _IntoKeyExpr,
        *,
        congestion_control: CongestionControl | None = None,
        priority: Priority | None = None,
        express: bool | None = None,
        attachment: _IntoZBytes | None = None,
        timestamp: Timestamp | None = None,
    ):
        """Sends a :class:`Sample` of kind :attr:`SampleKind.DELETE` as a reply to this query.

        By default, queries only accept replies whose key expression intersects with the query's. Unless the query has enabled disjoint replies (you can check this through :meth:`accepts_replies`), replying on a disjoint key expression will result in an error when resolving the reply.

        .. note::
           See the class documentation for important details about which key expression to use for replies.
        """

    @_unstable
    @property
    def source_info(self) -> SourceInfo | None:
        """Gets info on the source of this Query."""

    def drop(self):
        """Drop the instance of a query.
        The query will only be finalized when all query instances (one per queryable
        matched) are dropped. Finalization is required to not have query hanging
        on the querier side.

        This method should be called after handling the query, as Python finalizers
        are not reliable, especially when it comes to loop variables. It is also
        possible, and advised, to use query context manager, which calls `drop` on
        exit. Once a query is dropped, it's no more possible to use it, and its
        methods will raise an exception.
        """

    def __str__(self) -> str:
        """Returns a string representation of this query."""

@final
class Queryable(Generic[_H]):
    """A Queryable is an entity that implements :ref:`query-reply` pattern.

    It is declared by the :meth:`Session.declare_queryable` method and provides
    :class:`Query`es using callback or channel. The Queryable receives :class:`Query`
    requests from :meth:`Querier.get` or :meth:`Session.get` and sends back replies
    with the methods of :meth:`Query.reply`, :meth:`Query.reply_err`,
    or :meth:`Query.reply_del`."""

    def __enter__(self) -> Self: ...
    def __exit__(self, *_args, **_kwargs): ...
    @_unstable
    @property
    def id(self) -> EntityGlobalId:
        """Returns the :class:`EntityGlobalId` of this Queryable."""

    @property
    def key_expr(self) -> KeyExpr:
        """Returns the :class:`KeyExpr` this queryable responds to."""

    @property
    def handler(self) -> _H:
        """The handler associated with this Queryable instance.

        See :ref:`channels-and-callbacks` for more information on handlers."""

    def undeclare(self):
        """Undeclare the Queryable."""

    def try_recv(self: Queryable[handlers.Handler[Query]]) -> Query | None:
        """Try to receive a :class:`Query` from the handler without blocking."""

    def recv(self: Queryable[handlers.Handler[Query]]) -> Query:
        """Receive a :class:`Query` from the handler, blocking if necessary."""

    def __iter__(self: Queryable[Handler[Query]]) -> Handler[Query]:
        """Iterate over :class:`Query` received by the handler."""

@final
class Querier:
    """A querier that allows sending queries to a :class:`Queryable`.

    The querier is a preconfigured object that can be used to send multiple queries
    to a given key expression. It is declared using :meth:`Session.declare_querier`.
    Queriers are automatically undeclared when dropped.

    See :ref:`query-reply` for more information on the query/reply paradigm."""

    def __enter__(self) -> Self: ...
    def __exit__(self, *_args, **_kwargs): ...
    @_unstable
    @property
    def id(self) -> EntityGlobalId:
        """Returns the :class:`EntityGlobalId` of this Querier."""

    @property
    def key_expr(self) -> KeyExpr:
        """Returns the :class:`KeyExpr` this querier sends queries on."""

    @property
    def matching_status(self) -> bool:
        """Returns true if there are :class:`Queryable`\\s matching the Querier's key expression and target, false otherwise."""

    @overload
    def get(
        self,
        handler: _RustHandler[Reply] | None = None,
        *,
        parameters: _IntoParameters | None = None,
        payload: _IntoZBytes | None = None,
        encoding: _IntoEncoding | None = None,
        attachment: _IntoZBytes | None = None,
        source_info: SourceInfo | None = None,
        cancellation_token: CancellationToken | None = None,
    ) -> Handler[Reply]:
        """Sends a query and returns a channel for processing replies.

        See :ref:`channels-and-callbacks` for more information on handlers."""

    @overload
    def get(
        self,
        handler: _PythonHandler[Reply, _H],
        *,
        parameters: _IntoParameters | None = None,
        payload: _IntoZBytes | None = None,
        encoding: _IntoEncoding | None = None,
        attachment: _IntoZBytes | None = None,
        source_info: SourceInfo | None = None,
        cancellation_token: CancellationToken | None = None,
    ) -> _H:
        """Sends a query and returns a channel for processing replies.

        See :ref:`channels-and-callbacks` for more information on handlers."""

    @overload
    def get(
        self,
        handler: _PythonCallback[Reply],
        *,
        parameters: _IntoParameters | None = None,
        payload: _IntoZBytes | None = None,
        encoding: _IntoEncoding | None = None,
        attachment: _IntoZBytes | None = None,
        source_info: SourceInfo | None = None,
        cancellation_token: CancellationToken | None = None,
    ) -> None:
        """Sends a query and processes replies using the provided callback.

        See :ref:`channels-and-callbacks` for more information on callbacks."""

    def undeclare(self):
        """Undeclares the Querier, informing the network that it needn't optimize queries for its key expression anymore."""

    @overload
    def declare_matching_listener(
        self, handler: _RustHandler[MatchingStatus] | None = None
    ) -> MatchingListener[Handler[MatchingStatus]]:
        """Returns a :class:`MatchingListener` for this Querier.

        The :class:`MatchingListener` will send a notification each time the :class:`MatchingStatus` of the Querier changes.
        """

    @overload
    def declare_matching_listener(
        self, handler: _PythonHandler[MatchingStatus, _H]
    ) -> MatchingListener[_H]:
        """Returns a :class:`MatchingListener` for this Querier.

        The :class:`MatchingListener` will send a notification each time the :class:`MatchingStatus` of the Querier changes.
        """

    @overload
    def declare_matching_listener(
        self, handler: _PythonCallback[MatchingStatus]
    ) -> MatchingListener[None]:
        """Returns a :class:`MatchingListener` for this Querier.

        The :class:`MatchingListener` will send a notification each time the :class:`MatchingStatus` of the Querier changes.
        """

@final
class QueryConsolidation:
    """The reply consolidation strategy to apply to replies to a get.

    By default, the consolidation strategy is :attr:`QueryConsolidation.AUTO`, which lets the implementation
    choose the best strategy depending on the query parameters and the number of responders. Other
    strategies can be selected by using a specific :class:`ConsolidationMode` as a parameter of the
    :meth:`Session.declare_querier` or :meth:`Session.get` methods.

    See the documentation of :class:`ConsolidationMode` for more details about each strategy.
    """

    AUTO: Self
    DEFAULT: Self
    def __new__(cls, mode: ConsolidationMode, /) -> Self: ...
    @property
    def mode(self) -> ConsolidationMode: ...

_IntoQueryConsolidation = ConsolidationMode

@final
class QueryTarget(Enum):
    """The Queryables to which a query from :meth:`Session.get` or :meth:`Session.declare_querier` is delivered.

    :attr:`QueryTarget.ALL` makes the query be delivered to all the matching queryables.
    :attr:`QueryTarget.ALL_COMPLETE` makes the query be delivered to all the matching queryables which are marked as "complete".
    :attr:`QueryTarget.BEST_MATCHING` (default) makes the data to be requested from the queryable(s) selected by zenoh to get the fastest and most complete reply.

    It is set by the target parameter of :meth:`Session.get` or :meth:`Session.declare_querier` methods.
    """

    BEST_MATCHING = auto()
    ALL = auto()
    ALL_COMPLETE = auto()

    DEFAULT = BEST_MATCHING

QueryTarget.BEST_MATCHING.__doc__ = (
    """Let Zenoh find the best matching queryable capable of serving the query."""
)
QueryTarget.ALL.__doc__ = (
    """Deliver the query to all queryables matching the query's key expression."""
)
QueryTarget.ALL_COMPLETE.__doc__ = """Deliver the query to all queryables matching the query's key expression that are declared as complete."""

@final
@_unstable
class Reliability(Enum):
    """Reliability guarantees for message delivery.

    Used when declaring publishers with :meth:`Session.declare_publisher` and
    accessible via the :attr:`Publisher.reliability` property.
    """

    BEST_EFFORT = auto()
    RELIABLE = auto()

Reliability.BEST_EFFORT.__doc__ = """Messages may be lost."""
Reliability.RELIABLE.__doc__ = """Messages are guaranteed to be delivered."""

@final
class Locality(Enum):
    """The locality of samples/queries to be received by subscribers/queryables or targeted by publishers/queriers.

    This enum controls whether data is exchanged only with local entities (in the same session),
    only with remote entities, or with both. It is used in the following settings:

    - :meth:`Session.declare_subscriber` (allowed_origin)
    - :meth:`Session.declare_queryable` (allowed_origin)
    - :meth:`Session.declare_publisher` (allowed_destination)
    - :meth:`Session.declare_querier` (allowed_destination)
    """

    SESSION_LOCAL = auto()
    REMOTE = auto()
    ANY = auto()

    DEFAULT = ANY

Locality.SESSION_LOCAL.__doc__ = (
    """Request/serve data only to entities in the same session."""
)
Locality.REMOTE.__doc__ = (
    """Request/serve data only to remote entities (not in the same session)."""
)
Locality.ANY.__doc__ = """Request/serve data to both local and remote entities."""

@final
class Reply:
    """An answer received from a :class:`Queryable`.

    Contains the result of a request to a :class:`Queryable` by :meth:`Session.get`
    or :meth:`Querier.get`. May be either a successful result with a :class:`Sample`
    or an error with a :class:`ReplyError`.
    """

    @property
    def result(self) -> Sample | ReplyError:
        """Gets the result of this reply which may be either a successful :class:`Sample` or an error :class:`ReplyError`."""

    @property
    def ok(self) -> Sample | None:
        """Returns the successful result if this reply is successful, `None` otherwise."""

    @property
    def err(self) -> ReplyError | None:
        """Returns the error if this reply failed, `None` otherwise."""

    @property
    @_unstable
    def replier_id(self) -> EntityGlobalId | None:
        """Returns the ID of the zenoh instance that answered this reply."""

@final
class ReplyError:
    """An error reply received from a :class:`Queryable` and available in the :class:`Reply` structure."""

    @property
    def payload(self) -> ZBytes:
        """Gets the payload of this `ReplyError`, usually an error message."""

    @property
    def encoding(self) -> Encoding:
        """Gets the encoding of this `ReplyError`."""

@final
class SampleKind(Enum):
    """The kind of a :class:`Sample`, indicating whether it contains data or indicates deletion."""

    PUT = auto()
    DELETE = auto()

SampleKind.PUT.__doc__ = """A `PUT` sample containing data."""
SampleKind.DELETE.__doc__ = """A `DELETE` sample indicating data removal."""

@final
class Sample:
    """The Sample structure is the data unit received by :class:`Subscriber`, or by :class:`Querier` or :meth:`Session.get` as part of the :class:`Reply`.

    It contains the payload and all metadata associated with the data.
    """

    @property
    def key_expr(self) -> KeyExpr:
        """Gets the key expression on which this Sample was published."""

    @property
    def payload(self) -> ZBytes:
        """Gets the payload of this Sample."""

    @property
    def kind(self) -> SampleKind:
        """Gets the kind of this Sample."""

    @property
    def encoding(self) -> Encoding:
        """Gets the encoding of this sample."""

    @property
    def timestamp(self) -> Timestamp | None:
        """Gets the timestamp of this Sample."""

    @property
    def congestion_control(self) -> CongestionControl:
        """Gets the congestion control of this Sample."""

    @property
    def priority(self) -> Priority:
        """Gets the priority of this Sample."""

    @property
    def express(self) -> bool:
        """Gets the express flag value.

        If true, the message is not batched during transmission, in order to reduce latency.
        """

    @property
    def attachment(self) -> ZBytes | None:
        """Gets the sample attachment: a map of key-value pairs."""

    @_unstable
    @property
    def source_info(self) -> SourceInfo | None:
        """Gets info on the source of this Sample."""

@final
class Scout(Generic[_H]):
    """A Scout object that yields :class:`zenoh.Hello` messages for discovered Zenoh nodes on the network.

    Scout is returned by the :func:`zenoh.scout` function and is used to discover
    Zenoh nodes (routers and peers) on the network. It yields :class:`zenoh.Hello`
    messages containing information about each discovered node.

    See :ref:`scouting` for more details on the scouting process.
    """

    def __enter__(self) -> Self:
        """Enter the Scout context manager."""

    def __exit__(self, *_args, **_kwargs):
        """Exit the Scout context manager and stop scouting."""

    @property
    def handler(self) -> _H:
        """The handler associated with this Scout instance.

        See :ref:`channels-and-callbacks` for more information on handlers."""

    def stop(self):
        """Stop the scouting process."""

    def try_recv(self: Scout[Handler[Hello]]) -> Hello | None:
        """Try to receive a :class:`zenoh.Hello` message without blocking. Returns None if no message is available."""

    def recv(self: Scout[Handler[Hello]]) -> Hello:
        """Receive a :class:`zenoh.Hello` message, blocking until one is available."""

    def __iter__(self: Scout[Handler[Hello]]) -> Handler[Hello]:
        """Iterate over received :class:`zenoh.Hello` messages."""

@final
class Selector:
    """A selector is the combination of a :class:`KeyExpr`, which defines the set of keys that are relevant to an operation, and a set of :class:`Parameters` with a few intended uses.

    **Creating a Selector**

    A :class:`Selector` can be created from a key expression and optional parameters:

    .. code-block:: python

        selector = zenoh.Selector("key/expression", parameters)

    Or from a complete selector string:

    .. code-block:: python

        selector = zenoh.Selector("key/expression?param1=value1;param2=value2")

    If first parameter is already a complete selector string, the `parameters` should be omitted.

    A selector is the combination of a :class:`KeyExpr`, which defines the set of keys that are relevant to an operation, and a set of :class:`Parameters` with a few intended uses:

    - specifying arguments to a :class:`Queryable`, allowing the passing of Remote Procedure Call parameters
    - filtering by value
    - filtering by metadata, such as the timestamp of a value
    - specifying arguments to zenoh when using the `REST API <https://zenoh.io/docs/apis/rest/>`_

    When in string form, selectors look a lot like a URI, with similar semantics:

    - the ``key_expr`` before the first ``?`` must be a valid key expression.
    - the ``parameters`` after the first ``?`` should be encoded like the query section of a URL:

      - parameters are separated by ``;``
      - the parameter name and value are separated by the first ``=``
      - in the absence of ``=``, the parameter value is considered to be the empty string
      - both name and value should use percent-encoding (URL-encoding) to escape characters
      - defining a value for the same parameter name twice is considered undefined behavior, with the encouraged behaviour being to reject operations when a duplicate parameter is detected

    Zenoh intends to standardize the usage of a set of parameter names. To avoid conflicting with RPC parameters, the Zenoh team has settled on reserving the set of parameter names that start with non-alphanumeric characters.

    :class:`Queryable` implementers are encouraged to prefer these standardized parameter names when implementing their associated features, and to prefix their own parameter names to avoid having conflicting parameter names with other queryables.

    Here are the currently standardized parameters for Zenoh (check the
    `specification page <https://github.com/eclipse-zenoh/roadmap/tree/main/rfcs/ALL/Selectors>`_,
    for the exhaustive list):

    - ``[unstable]`` ``_time``: used to express interest in only values dated within a certain time range, values for this parameter must be readable by the Zenoh Time DSL for the value to be considered valid.
    - ``[unstable]`` ``_anyke``: used in queries to express interest in replies coming from any key expression. By default, only replies whose key expression match query's key expression are accepted. ``_anyke`` disables the query-reply key expression matching check.

    See also: :ref:`key-expressions`, :ref:`query-parameters`
    """

    def __new__(
        cls, arg: _IntoKeyExpr | str, /, parameters: _IntoParameters | None = None
    ): ...
    @property
    def key_expr(self) -> KeyExpr:
        """The key expression part of this selector."""

    @key_expr.setter
    def key_expr(self, key_expr: _IntoKeyExpr): ...
    @property
    def parameters(self) -> Parameters:
        """The parameters part of this selector."""

    @parameters.setter
    def parameters(self, parameters: _IntoParameters): ...
    def __str__(self) -> str: ...

_IntoSelector = Selector | _IntoKeyExpr

@final
class Session:
    """The Session is the main component of Zenoh. It holds the zenoh runtime object, which maintains the state of the connection of the node to the Zenoh network.

    The session allows declaring other zenoh entities like :class:`Publisher`, :class:`Subscriber`, :class:`Querier`, :class:`Queryable`, and obtaining :class:`Liveliness` instances, and keeps them functioning. Closing the session will undeclare all objects declared by it.

    A Zenoh session is instantiated using :func:`open` with parameters specified in the :class:`Config` object.
    """

    def __enter__(self) -> Self: ...
    def __exit__(self, *_args, **_kwargs): ...
    @property
    def info(self) -> SessionInfo:
        """Get information about the session: the session id, the connected nodes."""

    @_unstable
    @property
    def id(self) -> EntityGlobalId:
        """Returns the global identifier of the session object."""

    def zid(self) -> ZenohId:
        """Returns the identifier of the current session."""

    def close(self):
        """Close the zenoh Session.

        Every :class:`Subscriber` and :class:`Queryable` declared will stop receiving data, and further
        attempts to publish or query will result in an
        error. Undeclaring an entity after session closing is a no-op. Session state can be
        checked with :meth:`is_closed`.

        Sessions are automatically closed when all their instances are dropped. But it can
        be useful to close the session explicitly.
        """

    def is_closed(self) -> bool:
        """Check if the session has been closed."""

    def undeclare(self, obj: KeyExpr):
        """Undeclare a zenoh entity declared by the session."""

    def new_timestamp(self) -> Timestamp:
        """Get a new :class:`Timestamp` from a Zenoh session.

        The returned timestamp has the current time, with the session's runtime :class:`ZenohId`
        as the unique identifier. This ensures that timestamps from different sessions are unique
        even when created at the same time.

        Returns:
            A new :class:`Timestamp` with current time and session's unique ID.
        """

    def declare_keyexpr(self, key_expr: _IntoKeyExpr):
        """Informs Zenoh that you intend to use the provided key_expr multiple times and that it should optimize its transmission."""

    def put(
        self,
        key_expr: _IntoKeyExpr,
        payload: _IntoZBytes,
        *,
        encoding: _IntoEncoding | None = None,
        congestion_control: CongestionControl | None = None,
        priority: Priority | None = None,
        express: bool | None = None,
        attachment: _IntoZBytes | None = None,
        timestamp: Timestamp | None = None,
        allowed_destination: Locality | None = None,
        source_info: SourceInfo | None = None,
    ):
        """Publish data directly from the session.

        This is a shortcut for declaring a :class:`Publisher` and calling put on it.
        """

    def delete(
        self,
        key_expr: _IntoKeyExpr,
        *,
        congestion_control: CongestionControl | None = None,
        priority: Priority | None = None,
        express: bool | None = None,
        attachment: _IntoZBytes | None = None,
        timestamp: Timestamp | None = None,
        allowed_destination: Locality | None = None,
        source_info: SourceInfo | None = None,
    ):
        """Publish a delete sample directly from the session.

        This is a shortcut for declaring a :class:`Publisher` and calling delete on it.
        """

    @overload
    def get(
        self,
        selector: _IntoSelector,
        handler: _RustHandler[Reply] | None = None,
        *,
        target: QueryTarget | None = None,
        consolidation: _IntoQueryConsolidation | None = None,
        timeout: float | int | None = None,
        congestion_control: CongestionControl | None = None,
        priority: Priority | None = None,
        express: bool | None = None,
        payload: _IntoZBytes = None,
        encoding: _IntoEncoding | None = None,
        attachment: _IntoZBytes | None = None,
        allowed_destination: Locality | None = None,
        source_info: SourceInfo | None = None,
        cancellation_token: CancellationToken | None = None,
    ) -> Handler[Reply]:
        """Query data from the matching queryables in the system.

        This is a shortcut for declaring a :class:`Querier` and calling get on it.
        """

    @overload
    def get(
        self,
        selector: _IntoSelector,
        handler: _PythonHandler[Reply, _H],
        *,
        target: QueryTarget | None = None,
        consolidation: _IntoQueryConsolidation | None = None,
        timeout: float | int | None = None,
        congestion_control: CongestionControl | None = None,
        priority: Priority | None = None,
        express: bool | None = None,
        payload: _IntoZBytes = None,
        encoding: _IntoEncoding | None = None,
        attachment: _IntoZBytes | None = None,
        allowed_destination: Locality | None = None,
        source_info: SourceInfo | None = None,
        cancellation_token: CancellationToken | None = None,
    ) -> _H:
        """Query data from the matching queryables in the system.

        This is a shortcut for declaring a :class:`Querier` and calling get on it.
        """

    @overload
    def get(
        self,
        selector: _IntoSelector,
        handler: _PythonCallback[Reply],
        *,
        target: QueryTarget | None = None,
        consolidation: _IntoQueryConsolidation | None = None,
        timeout: float | int | None = None,
        congestion_control: CongestionControl | None = None,
        priority: Priority | None = None,
        express: bool | None = None,
        payload: _IntoZBytes = None,
        encoding: _IntoEncoding | None = None,
        attachment: _IntoZBytes | None = None,
        allowed_destination: Locality | None = None,
        source_info: SourceInfo | None = None,
        cancellation_token: CancellationToken | None = None,
    ) -> None:
        """Query data from the matching queryables in the system.

        This is a shortcut for declaring a :class:`Querier` and calling get on it.
        """

    @overload
    def declare_subscriber(
        self,
        key_expr: _IntoKeyExpr,
        handler: _RustHandler[Sample] | None = None,
        *,
        allowed_origin: Locality | None = None,
    ) -> Subscriber[Handler[Sample]]:
        """Create a :class:`Subscriber` for the given key expression."""

    @overload
    def declare_subscriber(
        self,
        key_expr: _IntoKeyExpr,
        handler: _PythonHandler[Sample, _H],
        *,
        allowed_origin: Locality | None = None,
    ) -> Subscriber[_H]:
        """Create a :class:`Subscriber` for the given key expression."""

    @overload
    def declare_subscriber(
        self,
        key_expr: _IntoKeyExpr,
        handler: _PythonCallback[Sample],
        *,
        allowed_origin: Locality | None = None,
    ) -> Subscriber[None]:
        """Create a :class:`Subscriber` for the given key expression."""

    @overload
    def declare_queryable(
        self,
        key_expr: _IntoKeyExpr,
        handler: _RustHandler[Query] | None = None,
        *,
        complete: bool | None = None,
        allowed_origin: Locality | None = None,
    ) -> Queryable[Handler[Query]]:
        """Create a :class:`Queryable` for the given key expression."""

    @overload
    def declare_queryable(
        self,
        key_expr: _IntoKeyExpr,
        handler: _PythonHandler[Query, _H],
        *,
        complete: bool | None = None,
        allowed_origin: Locality | None = None,
    ) -> Queryable[_H]:
        """Create a :class:`Queryable` for the given key expression."""

    @overload
    def declare_queryable(
        self,
        key_expr: _IntoKeyExpr,
        handler: _PythonCallback[Query],
        *,
        complete: bool | None = None,
        allowed_origin: Locality | None = None,
    ) -> Queryable[None]:
        """Create a :class:`Queryable` for the given key expression."""

    def declare_publisher(
        self,
        key_expr: _IntoKeyExpr,
        *,
        encoding: _IntoEncoding | None = None,
        congestion_control: CongestionControl | None = None,
        priority: Priority | None = None,
        express: bool | None = None,
        reliability: Reliability | None = None,
        allowed_destination: Locality | None = None,
    ) -> Publisher:
        """Create a :class:`Publisher` for the given key expression."""

    def declare_querier(
        self,
        key_expr: _IntoKeyExpr,
        *,
        target: QueryTarget | None = None,
        consolidation: _IntoQueryConsolidation | None = None,
        timeout: float | int | None = None,
        congestion_control: CongestionControl | None = None,
        priority: Priority | None = None,
        express: bool | None = None,
        allowed_destination: Locality | None = None,
    ) -> Querier:
        """Create a :class:`Querier` for the given key expression."""

    def liveliness(self) -> Liveliness:
        """Obtain a :class:`Liveliness` instance tied to this Zenoh session."""

@final
class SessionInfo:
    """Struct returned by :class:`Session.info()` that allows access to information about the current zenoh :class:`Session`.

    This information includes the :class:`ZenohId` identifier of the current session, and the identifiers of the connected routers
    and peers (see also :class:`WhatAmI` for more information about peers and routers).
    """

    def zid(self) -> ZenohId:
        """Return the :class:`ZenohId` of the current zenoh Session."""

    def routers_zid(self) -> list[ZenohId]:
        """Return the :class:`ZenohId` of the zenoh routers this process is currently connected to."""

    def peers_zid(self) -> list[ZenohId]:
        """Return the :class:`ZenohId` of the zenoh peers this process is currently connected to."""

@_unstable
@final
class SetIntersectionLevel(Enum):
    """The possible relations between two sets of key expressions defined by glob patterns.

    Each glob key expression defines a set of possible concrete key expressions that it matches.
    This enum describes how two such sets relate to each other.

    Returned by :meth:`KeyExpr.relation_to`.

    Note that :attr:`EQUALS` implies :attr:`INCLUDES`, which itself implies :attr:`INTERSECTS`.

    You can check for intersection with `level >= SetIntersectionLevel.INTERSECTS` and for inclusion with `level >= SetIntersectionLevel.INCLUDES`.
    """

    DISJOINT = auto()
    INTERSECTS = auto()
    INCLUDES = auto()
    EQUALS = auto()

SetIntersectionLevel.DISJOINT.__doc__ = "The sets have no key expressions in common. Example: ``foo/*`` and ``bar/*`` - no overlap."
SetIntersectionLevel.INTERSECTS.__doc__ = "The sets have some key expressions in common, but neither fully contains the other. Example: ``foo/*`` and ``*/bar`` - ``foo/bar`` matches both."
SetIntersectionLevel.INCLUDES.__doc__ = "The first set fully contains the second set. Example: ``foo/**`` includes ``foo/*`` (where ``**`` matches any number of sections)."
SetIntersectionLevel.EQUALS.__doc__ = (
    "The sets are identical. Example: ``foo/*`` and ``foo/*``."
)

@final
class MatchingStatus:
    """A struct that indicates if there exist entities matching the key expression.

    See also: :ref:`matching`
    """

    @property
    def matching(self) -> bool:
        """Return true if there exist entities matching the target (i.e either Subscribers matching Publisher's key expression or Queryables matching Querier's key expression and target)."""

@final
class MatchingListener(Generic[_H]):
    """A listener that sends notifications when the :class:`MatchingStatus` of a
    corresponding Zenoh entity changes.

    The matching listener allows publishers and queriers to detect when there are
    matching subscribers or queryables on the network, enabling efficient resource usage.

    See also: :ref:`matching`
    """

    def __enter__(self) -> Self: ...
    def __exit__(self, *_args, **_kwargs): ...
    @property
    def handler(self) -> _H:
        """The handler associated with this MatchingListener instance.

        See :ref:`channels-and-callbacks` for more information on handlers."""

    def undeclare(self):
        """Close a Matching listener.
        Matching listeners are automatically closed when dropped, but you may want to use this function to handle errors or close the Matching listener asynchronously.
        """

    def try_recv(
        self: MatchingListener[Handler[MatchingStatus]],
    ) -> MatchingStatus | None: ...
    def recv(self: MatchingListener[Handler[MatchingStatus]]) -> MatchingStatus: ...
    def __iter__(
        self: MatchingListener[Handler[MatchingStatus]],
    ) -> Handler[MatchingStatus]: ...

@_unstable
@final
class SourceInfo:
    """Information on the source of a zenoh Sample.

    Contains metadata about the origin of a data sample, including the source entity's
    global identifier and sequence number.
    """

    def __new__(cls, source_id: EntityGlobalId, source_sn: SourceSn) -> Self: ...
    @property
    def source_id(self) -> EntityGlobalId:
        """The EntityGlobalId of the zenoh entity that published the Sample in question."""

    @property
    def source_sn(self) -> SourceSn:
        """The sequence number of the Sample from the source."""

SourceSn = int

@final
class Subscriber(Generic[_H]):
    """A subscriber that receives data from :class:`Publisher` instances matching its key expression.

    Subscribers are automatically undeclared when dropped.

    A subscriber is created using :meth:`zenoh.Session.declare_subscriber` and is used to receive
    data from :class:`Publisher` instances matching the subscriber's key expression.

    For more information about publish/subscribe operations, see :ref:`publish-subscribe`.
    """

    def __enter__(self) -> Self: ...
    def __exit__(self, *_args, **_kwargs): ...
    @_unstable
    @property
    def id(self) -> EntityGlobalId:
        """The global ID of this subscriber."""

    @property
    def key_expr(self) -> KeyExpr:
        """The key expression this subscriber subscribes to."""

    @property
    def handler(self) -> _H:
        """The handler associated with this Subscriber instance.

        See :ref:`channels-and-callbacks` for more information on handlers."""

    def undeclare(self):
        """Close a Subscriber.
        Subscribers are automatically closed when dropped, but you may want to use this function to handle errors or close the Subscriber asynchronously.
        """

    def try_recv(self: Subscriber[Handler[Sample]]) -> Sample | None:
        """Try to receive a :class:`Sample` without blocking.

        Returns the sample if available, or None if no sample is ready.
        """

    def recv(self: Subscriber[Handler[Sample]]) -> Sample:
        """Receive a :class:`Sample`, blocking until one is available."""

    def __iter__(self: Subscriber[Handler[Sample]]) -> Handler[Sample]:
        """Iterate over received :class:`Sample` instances."""

@final
class Timestamp:
    """A timestamp consisting of an `NTP64 <https://docs.rs/zenoh/latest/zenoh/time/struct.NTP64.html>`_
    time and a unique identifier.

    Timestamps are used to provide temporal ordering and uniqueness in Zenoh operations.
    They combine a high-precision NTP64 timestamp with a unique identifier to ensure
    causality and ordering in distributed systems.

    Timestamps can be created using :meth:`Session.new_timestamp`, which returns a
    timestamp with the current time and the session's unique runtime identifier.

    **String Representations:**

    Timestamps support two string formats:

    - **Default format**: ``"<ntp64_time>/<id_hex>"`` (e.g., ``"7386690599959157260/33"``)
      This is a lossless, machine-readable format.

    - **RFC3339 format**: ``"<rfc3339_time>/<id_hex>"`` (e.g., ``"2024-07-01T13:51:12.129693000Z/33"``)
      This is a human-readable format with nanosecond precision, but may lose some precision
      due to rounding when converting fractional seconds to nanoseconds.

    For detailed information about Timestamp, see: https://docs.rs/zenoh/latest/zenoh/time/struct.Timestamp.html
    """

    def __new__(cls, time: datetime | NTP64, id: _IntoTimestampId) -> Self: ...
    def get_time(self) -> datetime:
        """Returns the time component of the timestamp as a datetime object."""

    def get_time_as_ntp64(self) -> NTP64:
        """Returns the time component of the timestamp as a datetime object."""

    def get_id(self) -> TimestampId:
        """Returns the unique identifier component of the timestamp as a :class:`TimestampId`."""

    def get_diff_duration(self, other: Timestamp) -> timedelta:
        """Returns the duration difference between this timestamp and another.

        Args:
            other: The timestamp to compare against.

        Returns:
            A timedelta representing the time difference.
        """

    def to_string_rfc3339_lossy(self) -> str:
        """Convert to a RFC3339 time representation with nanoseconds precision.

        This format is human-readable but may lose precision due to rounding
        when converting fractional seconds to nanoseconds.

        Returns:
            A string in RFC3339 format (e.g., "2024-07-01T13:51:12.129693000Z/33").

        Note:
            This conversion is not bijective - converting to string and back may
            result in a slightly different timestamp due to precision loss.
        """

    @classmethod
    def parse_rfc3339(cls, s: str) -> Self:
        """Parse a RFC3339 time representation into a Timestamp.

        Args:
            s: A string in RFC3339 format with timestamp ID (e.g., "2024-07-01T13:51:12.129693000Z/33").

        Returns:
            A new Timestamp object.

        Raises:
            ZError: If the string cannot be parsed.
        """

    def __eq__(self, other: Any) -> bool: ...
    def __ge__(self, other) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str:
        """Returns the timestamp in default format: "<ntp64_time>/<id_hex>"."""

@final
class TimestampId:
    """A unique identifier used in :class:`Timestamp`.

    TimestampId represents a unique identifier that is part of every :class:`Timestamp`.
    It is typically derived from a :class:`ZenohId` (session identifier) to ensure that
    timestamps from different sessions remain unique even when created simultaneously.

    The identifier is stored as a fixed-size byte array and provides methods for
    comparison, hashing, and conversion to/from bytes.

    **Used in:**

    - :meth:`Timestamp.__new__` - accepts ``_IntoTimestampId`` (bytearray, bytes, or TimestampId)
    - :meth:`Timestamp.get_id` - returns a TimestampId
    - :meth:`Session.new_timestamp` - creates timestamps with session's ZenohId as TimestampId
    """

    def __new__(cls, bytes: bytearray | bytes) -> Self: ...
    def __bytes__(self) -> bytes:
        """Returns the identifier as bytes."""

    def __eq__(self, other: Any) -> bool: ...
    def __ge__(self, other) -> bool: ...
    def __hash__(self) -> int: ...

_IntoTimestampId = bytearray | bytes | TimestampId
"""Type alias for values that can be converted to a TimestampId.

Used in :meth:`Timestamp.__new__` to accept various byte representations
that can be converted to a :class:`TimestampId`.
"""

@final
class WhatAmI(Enum):
    """The type of the node in the Zenoh network.

    The zenoh application can work in three different modes: router, peer, and client.

    For detailed format information, see: https://docs.rs/zenoh/latest/zenoh/config/enum.WhatAmI.html
    """

    ROUTER = auto()
    PEER = auto()
    CLIENT = auto()

    def __str__(self) -> str: ...

WhatAmI.ROUTER.__doc__ = """Router mode: Used to run a zenoh router, which is a node that maintains a
predefined zenoh network topology. Unlike peers, routers do not discover other
nodes by themselves, but rely on static configuration."""

WhatAmI.PEER.__doc__ = """Peer mode: The application searches for other nodes and establishes direct
connections with them. This can work using multicast discovery and by getting
gossip information from the initial entry points. The peer mode is the default
mode."""

WhatAmI.CLIENT.__doc__ = """Client mode: The application remains connected to a single connection
point, which serves as a gateway to the rest of the network. This mode is useful for
constrained devices that cannot afford to maintain multiple connections."""

@final
class WhatAmIMatcher:
    """A helper type that allows matching combinations of WhatAmI values in scouting.

    WhatAmIMatcher can be created from a string specification like "peer|router" or "client",
    or built programmatically using methods like :meth:`router`, :meth:`peer`, and :meth:`client`.

    The :func:`scout` function accepts a WhatAmIMatcher to filter the nodes of the specified types.
    """

    def __new__(cls, matcher: str | None = None) -> Self:
        """Creates a matcher from a string specification or an empty matcher if None."""

    @classmethod
    def empty(cls) -> Self:
        """Creates an empty matcher that matches no node types."""

    def router(self) -> Self:
        """Adds :attr:`WhatAmI.ROUTER` to the matcher."""

    def peer(self) -> Self:
        """Adds :attr:`WhatAmI.PEER` to the matcher."""

    def client(self) -> Self:
        """Adds :attr:`WhatAmI.CLIENT` to the matcher."""

    def is_empty(self) -> bool:
        """Returns True if the matcher matches no node types."""

    def matches(self, whatami: WhatAmI) -> bool:
        """Returns True if the given WhatAmI value matches this matcher."""

    def __str__(self) -> str:
        """Returns a string representation of the matcher."""

_IntoWhatAmIMatcher = WhatAmIMatcher | str

@final
class ZBytes:
    """ZBytes represents raw bytes data that can be interpreted as strings or byte arrays.

    ZBytes is the fundamental data container in Zenoh for all payload and attachment data.
    It provides flexible access to the underlying bytes, allowing conversion to strings,
    byte arrays, or direct byte access.

    The Zenoh protocol treats ZBytes as opaque binary data and is completely unaware of
    its content or structure. This allows users to employ any data representation format
    of their choice, including JSON, protobuf, flatbuffers, MessagePack, or custom formats.

    For convenience, Zenoh provides built-in serialization functions :func:`zenoh.ext.z_serialize`
    and :func:`zenoh.ext.z_deserialize` in the :mod:`zenoh.ext` module, which support
    serialization of standard Python types (primitives, lists, dicts, etc.) using Zenoh's
    native binary format. However, these are not mandatory - users are encouraged to use
    any serialization approach that fits their needs."""

    def __new__(
        cls, bytes: bytearray | bytes | str | shm.ZShm | shm.ZShmMut | None = None
    ) -> Self: ...
    def to_bytes(self) -> bytes:
        """Return the underlying data as bytes.

        Returns:
            bytes: The raw byte data contained in this ZBytes instance.
        """

    def to_string(self) -> str:
        """Return the underlying data as a UTF-8 decoded string.

        Returns:
            str: The string representation of the byte data, decoded as UTF-8.

        Raises:
            ValueError: If the byte data cannot be decoded as valid UTF-8.
        """

    @_unstable
    def as_shm(self) -> shm.ZShm | None: ...
    def __bool__(self) -> bool: ...
    def __len__(self) -> int: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __eq__(self, other: Any) -> bool: ...
    def __hash__(self) -> int: ...

_IntoZBytes = Any

@final
class ZenohId:
    """The global unique id of a zenoh peer."""

    def __str__(self) -> str: ...

def try_init_log_from_env():
    """Redirect zenoh logs to stdout, according to the `RUST_LOG` environment variable.

    For example, `RUST_LOG=debug` will set the log level to DEBUG.
    If `RUST_LOG` is not set, then logging is not enabled."""

def init_log_from_env_or(level: str):
    """Redirect zenoh logs to stdout, according to the `RUST_LOG` environment variable.

    For example, `RUST_LOG=debug` will set the log level to DEBUG.
    If `RUST_LOG` is not set, then logging is set to the provided level."""

def open(config: Config) -> Session:
    """Open a zenoh :class:`zenoh.Session`.

    For more information about sessions and configuration, see :ref:`session-and-config`.
    """

# Common docstring for all scout function overloads
_SCOUT_DOC = """Scout for routers and/or peers.

`scout` spawns a task that periodically sends scout messages and waits for :class:`zenoh.Hello` replies.
The scouting process can be stopped by calling :meth:`zenoh.Scout.stop` on the returned :class:`zenoh.Scout` object,
or by letting the :class:`zenoh.Scout` object go out of scope (dropping it).

Args:
    handler: Optional handler for processing received :class:`zenoh.Hello` messages.
    what: Optional :class:`zenoh.WhatAmIMatcher` or string specifying which node types to scout for
    (e.g., "peer|router"). If None, scouts for all node types.
    config: Optional :class:`zenoh.Config` for the scouting session.

For more information about scouting, see :ref:`scouting`.
"""

@overload
def scout(
    handler: _RustHandler[Hello] | None = None,
    what: _IntoWhatAmIMatcher | None = None,
    config: Config | None = None,
) -> Scout[Handler[Hello]]: ...

scout.__doc__ = _SCOUT_DOC

@overload
def scout(
    handler: _PythonHandler[Hello, _H],
    what: _IntoWhatAmIMatcher | None = None,
    config: Config | None = None,
) -> Scout[_H]: ...
@overload
def scout(
    handler: _PythonCallback[Hello],
    what: _IntoWhatAmIMatcher | None = None,
    config: Config | None = None,
) -> Scout[None]: ...
