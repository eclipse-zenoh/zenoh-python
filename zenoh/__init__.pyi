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
"""Zenoh /zeno/ is a stack that unifies data in motion, data at rest and computations. It elegantly blends traditional pub/sub with geo distributed storage, queries and computations, while retaining a level of time and space efficiency that is well beyond any of the mainstream stacks.

Before delving into the examples, we need to introduce few Zenoh concepts. First off, in Zenoh you will deal with Resources, where a resource is made up of a key and a value. The other concept you’ll have to familiarize yourself with are key expressions, such as robot/sensor/temp, robot/sensor/*, robot/**, etc. As you can gather, the above key expression denotes set of keys, while the * and ** are wildcards representing respectively (1) an arbitrary string of characters, with the exclusion of the / separator, and (2) an arbitrary sequence of characters including separators."""
from collections.abc import Callable, Iterable
from datetime import datetime
from enum import Enum, auto
from pathlib import Path
from typing import Any, Generic, Never, Self, TypeVar, final, overload

from . import handlers as handlers
from .handlers import Handler as Handler

_T = TypeVar("_T")
_H = TypeVar("_H")
_F = TypeVar("_F", bound=Callable)

_RustHandler = (
    handlers.DefaultHandler[_T] | handlers.FifoChannel[_T] | handlers.RingChannel[_T]
)

_PythonCallback = Callable[[_T], Any]
_PythonHandler = tuple[_PythonCallback[_T], _H]

@final
class ZError(Exception): ...

@final
class Config:
    """The main configuration structure for Zenoh."""

    def __new__(cls) -> Self: ...
    @classmethod
    def client(cls, peers: Iterable[str]) -> Self: ...
    @classmethod
    def empty(cls) -> Self: ...
    @classmethod
    def peer(cls) -> Self: ...
    @classmethod
    def from_env(cls) -> Self: ...
    @classmethod
    def from_file(cls, path: str | Path) -> Self: ...
    @classmethod
    def from_json5(cls, obj: Any) -> Self: ...
    @property
    def id(self) -> ZenohId:
        """The Zenoh ID of the instance. This ID MUST be unique throughout your Zenoh infrastructure and cannot exceed 16 bytes of length. If left unset, a random u128 will be generated."""

    def get_json(self, key: str) -> Any: ...
    def insert_json5(self, key: str, value: Any): ...
    def __str__(self) -> str: ...

@final
class CongestionControl(Enum):
    """The kind of congestion control."""

    DROP = auto()
    BLOCK = auto()

    DEFAULT = DROP

@final
class ConsolidationMode(Enum):
    """The kind of consolidation."""

    AUTO = auto()
    NONE = auto()
    MONOTONIC = auto()
    LATEST = auto()

    DEFAULT = AUTO

@final
class Encoding:
    """Default encoding values used by Zenoh.
    An encoding has a similar role to Content-type in HTTP: it indicates, when present, how data should be interpreted by the application.
    Please note the Zenoh protocol does not impose any encoding value nor it operates on it. It can be seen as some optional metadata that is carried over by Zenoh in such a way the application may perform different operations depending on the encoding value.
    A set of associated constants are provided to cover the most common encodings for user convenience. This is parcticular useful in helping Zenoh to perform additional network optimizations.
    """

    def __new__(cls, encoding: str | None = None) -> Self: ...
    def with_schema(self, schema: str) -> Self:
        """Set a schema to this encoding. Zenoh does not define what a schema is and its semantichs is left to the implementer. E.g. a common schema for text/plain encoding is utf-8."""

    def __eq__(self, other: _IntoEncoding) -> bool: ...
    def __hash__(self) -> int: ...
    def __str__(self) -> str: ...

    ZENOH_BYTES: Self
    """Just some bytes.
    
    Constant alias for string: "zenoh/bytes"."""
    ZENOH_INT8: Self
    """A VLE-encoded signed little-endian 8bit integer. Binary representation uses two's complement.
    
    Constant alias for string: "zenoh/int8"."""
    ZENOH_INT16: Self
    """A VLE-encoded signed little-endian 16bit integer. Binary representation uses two's complement.
    
    Constant alias for string: "zenoh/int16"."""
    ZENOH_INT32: Self
    """A VLE-encoded signed little-endian 32bit integer. Binary representation uses two's complement.
    
    Constant alias for string: "zenoh/int32"."""
    ZENOH_INT64: Self
    """A VLE-encoded signed little-endian 64bit integer. Binary representation uses two's complement.
    
    Constant alias for string: "zenoh/int64"."""
    ZENOH_INT128: Self
    """A VLE-encoded signed little-endian 128bit integer. Binary representation uses two's complement.
    
    Constant alias for string: "zenoh/int128"."""
    ZENOH_UINT8: Self
    """A VLE-encoded unsigned little-endian 8bit integer.

    Constant alias for string: "zenoh/uint8"."""
    ZENOH_UINT16: Self
    """A VLE-encoded unsigned little-endian 16bit integer.

    Constant alias for string: "zenoh/uint16"."""
    ZENOH_UINT32: Self
    """A VLE-encoded unsigned little-endian 32bit integer.

    Constant alias for string: "zenoh/uint32"."""
    ZENOH_UINT64: Self
    """A VLE-encoded unsigned little-endian 64bit integer.

    Constant alias for string: "zenoh/uint64"."""
    ZENOH_UINT128: Self
    """A VLE-encoded unsigned little-endian 128bit integer.

    Constant alias for string: "zenoh/uint128"."""
    ZENOH_FLOAT32: Self
    """A VLE-encoded 32bit float. Binary representation uses IEEE 754-2008 binary32 .
    
    Constant alias for string: "zenoh/ float32"."""
    ZENOH_FLOAT64: Self
    """A VLE-encoded 64bit float. Binary representation uses IEEE 754-2008 binary64 .
    
    Constant alias for string: "zenoh/ float64"."""
    ZENOH_BOOL: Self
    """A boolean. 0 is false, 1 is true. Other values are invalid.

    Constant alias for string: "zenoh/bool"."""
    ZENOH_STRING: Self
    """A UTF-8 string.

    Constant alias for string: "zenoh/string"."""
    ZENOH_ERROR: Self
    """A zenoh error.

    Constant alias for string: "zenoh/error"."""
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
    """JSON5 encoded data that are human readable.

    Constant alias for string: "text/json5"."""
    APPLICATION_PYTHON_SERIALIZED_OBJECT: Self
    """A Python object serialized using pickle .

    Constant alias for string: "application/python-serialized-object"."""
    APPLICATION_PROTOBUF: Self
    """An application-specific protobuf-encoded data.

    Constant alias for string: "application/protobuf"."""
    APPLICATION_JAVA_SERIALIZED_OBJECT: Self
    """A Java serialized object.

    Constant alias for string: "application/java-serialized-object"."""
    APPLICATION_OPENMETRICS_TEXT: Self
    """An openmetrics  data, common used by Prometheus .

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
    """An XML file intended to be consumed by an application..

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
    """A h261-encoded video stream.

    Constant alias for string: "video/h261"."""
    VIDEO_H263: Self
    """A h263-encoded video stream.

    Constant alias for string: "video/h263"."""
    VIDEO_H264: Self
    """A h264-encoded video stream.

    Constant alias for string: "video/h264"."""
    VIDEO_H265: Self
    """A h265-encoded video stream.

    Constant alias for string: "video/h265"."""
    VIDEO_H266: Self
    """A h266-encoded video stream.

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

@final
class Hello:
    @property
    def whatami(self) -> WhatAmI: ...
    @property
    def zid(self) -> ZenohId: ...
    @property
    def locators(self) -> list[str]: ...
    def __str__(self) -> str: ...

@final
class KeyExpr:
    """A possibly-owned version of keyexpr that may carry optimisations for use with a Session that may have declared it.
    Check keyexpr's documentation for detailed explainations of the Key Expression Language
    """

    def __new__(cls, key_expr: str) -> Self: ...
    @classmethod
    def autocanonize(cls, key_expr: str) -> Self:
        """Canonizes the passed value before returning it as a KeyExpr.
        Will return Err if the passed value isn't a valid key expression despite canonization.
        """

    def intersects(self, other: _IntoKeyExpr) -> bool:
        """Returns true if the keyexprs intersect, i.e. there exists at least one key which is contained in both of the sets defined by self and other."""

    def includes(self, other: _IntoKeyExpr) -> bool:
        """Returns true if self includes other, i.e. the set defined by self contains every key belonging to the set defined by other."""

    def relation_to(self, other: _IntoKeyExpr) -> SetIntersectionLevel:
        """Returns the relation between self and other from self's point of view (SetIntersectionLevel::Includes signifies that self includes other).
        Note that this is slower than keyexpr::intersects and keyexpr::includes, so you should favor these methods for most applications.
        """

    def join(self, other: str) -> KeyExpr:
        """Joins both sides, inserting a / in between them.
        This should be your prefered method when concatenating path segments."""

    def concat(self, other: str) -> KeyExpr:
        """Performs string concatenation and returns the result as a KeyExpr if possible.
        You should probably prefer KeyExpr::join as Zenoh may then take advantage of the hierachical separation it ins  erts.
        """

    def __str__(self) -> str: ...

_IntoKeyExpr = KeyExpr | str

@final
class Parameters:
    def __new__(cls, parameters: dict[str, str] | str): ...
    def is_empty(self) -> bool:
        """Returns true if properties does not contain anything."""

    def contains_key(self, key: str) -> bool:
        """Returns true if properties contains the specified key."""

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

@final
class Publisher:
    """A publisher that allows to send data through a stream.
    Publishers are automatically undeclared when dropped."""

    def __enter__(self) -> Self: ...
    def __exit__(self, *_args, **_kwargs): ...
    @property
    def key_expr(self) -> KeyExpr: ...
    @property
    def encoding(self) -> Encoding: ...
    @property
    def congestion_control(self) -> CongestionControl: ...
    @property
    def priority(self) -> Priority: ...
    @property
    def reliability(self) -> Reliability: ...
    def put(
        self,
        payload: _IntoZBytes,
        *,
        encoding: _IntoEncoding | None = None,
        attachment: _IntoZBytes | None = None,
    ):
        """Put data."""

    def delete(self, *, attachment: _IntoZBytes | None = None):
        """Delete data."""

    def undeclare(self):
        """Undeclares the Publisher, informing the network that it needn't optimize publications for its key expression anymore."""

@final
class Query:
    """Structs received by a Queryable."""

    @property
    def selector(self) -> Selector: ...
    @property
    def key_expr(self) -> KeyExpr: ...
    @property
    def payload(self) -> ZBytes | None: ...
    @property
    def encoding(self) -> Encoding | None: ...
    @property
    def attachment(self) -> ZBytes | None: ...
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
    ):
        """Sends a reply to this Query.
        By default, queries only accept replies whose key expression intersects with the query's. Unless the query has enabled disjoint replies (you can check this through Query::accepts_replies), replying on a disjoint key expression will result in an error when resolving the reply.
        """

    def reply_err(self, payload: _IntoZBytes, *, encoding: _IntoEncoding | None = None):
        """Sends an error reply to this Query."""

    def reply_del(
        self,
        key_expr: _IntoKeyExpr,
        *,
        congestion_control: CongestionControl | None = None,
        priority: Priority | None = None,
        express: bool | None = None,
        attachment: _IntoZBytes | None = None,
    ):
        """Sends a delete reply to this Query.
        By default, queries only accept replies whose key expression intersects with the query's. Unless the query has enabled disjoint replies (you can check this through Query::accepts_replies), replying on a disjoint key expression will result in an error when resolving the reply.
        """

    def __str__(self) -> str: ...

@final
class Queryable(Generic[_H]):
    """A queryable that provides data through a Handler.
    Queryables can be created from a zenoh Session with the declare_queryable function and the with function of the resulting builder.
    Queryables are automatically undeclared when dropped."""

    def __enter__(self) -> Self: ...
    def __exit__(self, *_args, **_kwargs): ...
    @property
    def handler(self) -> _H: ...
    def undeclare(self): ...
    @overload
    def try_recv(self: Queryable[handlers.Handler[Query]]) -> Query | None: ...
    @overload
    def try_recv(self) -> Never: ...
    @overload
    def recv(self: Queryable[handlers.Handler[Query]]) -> Query: ...
    @overload
    def recv(self) -> Never: ...
    @overload
    def __iter__(self: Queryable[Handler[Query]]) -> Handler[Query]: ...
    @overload
    def __iter__(self) -> Never: ...

@final
class QueryTarget(Enum):
    """The kind of consolidation."""

    BEST_MATCHING = auto()
    ALL = auto()
    ALL_COMPLETE = auto()

    DEFAULT = BEST_MATCHING

@final
class Reliability(Enum):
    BEST_EFFORT = auto()
    RELIABLE = auto()

@final
class Reply:
    """Structs returned by a get."""

    @property
    def result(self) -> Sample | ReplyError: ...
    @property
    def ok(self) -> Sample | None: ...
    @property
    def err(self) -> ReplyError | None: ...
    @property
    def replier_id(self) -> ZenohId | None: ...

@final
class ReplyError:
    @property
    def payload(self) -> ZBytes: ...
    @property
    def encoding(self) -> Encoding: ...

@final
class SampleKind(Enum):
    PUT = auto()
    DELETE = auto()

@final
class Sample:
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
    def timestamp(self) -> Timestamp:
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
    def attachment(self) -> ZBytes | None: ...

@final
class Scout(Generic[_H]):
    def __enter__(self) -> Self: ...
    def __exit__(self, *_args, **_kwargs): ...
    @property
    def handler(self) -> _H: ...
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

@final
class Selector:
    """A selector is the combination of a Key Expression, which defines the set of keys that are relevant to an operation, and a set of parameters with a few intendend uses:
    specifying arguments to a queryable, allowing the passing of Remote Procedure Call parameters
    filtering by value,
    filtering by metadata, such as the timestamp of a value,
    specifying arguments to zenoh when using the REST API.
    When in string form, selectors look a lot like a URI, with similar semantics:
    the key_expr before the first ? must be a valid key expression.
    the parameters after the first ? should be encoded like the query section of a URL:
    parameters are separated by &,
    the parameter name and value are separated by the first =,
    in the absence of =, the parameter value is considered to be the empty string,
    both name and value should use percent-encoding to escape characters,
    defining a value for the same parameter name twice is considered undefined behavior, with the encouraged behaviour being to reject operations when a duplicate parameter is detected.
    Zenoh intends to standardize the usage of a set of parameter names. To avoid conflicting with RPC parameters, the Zenoh team has settled on reserving the set of parameter names that start with non-alphanumeric characters.
    The full specification for selectors is available here , it includes standardized parameters.
    Queryable implementers are encouraged to prefer these standardized parameter names when implementing their associated features, and to prefix their own parameter names to avoid having conflicting parameter names with other queryables.
    Here are the currently standardized parameters for Zenoh (check the specification page for the exhaustive list):
    _time: used to express interest in only values dated within a certain time range, values for this parameter must be readable by the Zenoh Time DSL for the value to be considered valid.
    [unstable] _anyke: used in queries to express interest in replies coming from any key expression. By default, only replies whose key expression match query's key expression are accepted. _anyke disables the query-reply key expression matching check.
    """

    def __new__(
        cls, arg: _IntoKeyExpr | str, /, parameters: _IntoParameters | None = None
    ): ...
    @property
    def key_expr(self) -> KeyExpr: ...
    @key_expr.setter
    def key_expr(self, key_expr: _IntoKeyExpr): ...
    @property
    def parameters(self) -> Parameters: ...
    @parameters.setter
    def parameters(self, parameters: _IntoParameters): ...
    def __str__(self) -> str: ...

_IntoSelector = Selector | _IntoKeyExpr

@final
class Session:
    """A zenoh session."""

    def __enter__(self) -> Self: ...
    def __exit__(self, *_args, **_kwargs): ...
    @property
    def info(self) -> SessionInfo: ...
    def zid(self) -> ZenohId:
        """Returns the identifier of the current session. zid() is a convenient shortcut."""

    def close(self):
        """Close the zenoh Session.
        Sessions are automatically closed when dropped, but you may want to use this function to handle errors or close the Session asynchronously.
        """

    def undeclare(self, obj: KeyExpr): ...
    def config(self) -> Config:
        """Get the current configuration of the zenoh Session.
        The returned configuration Notifier can be used to read the current zenoh configuration through the get function or modify the zenoh configuration through the insert, or insert_json5 funtion.
        """

    def declare_keyexpr(self, key_expr: _IntoKeyExpr):
        """Informs Zenoh that you intend to use key_expr multiple times and that it should optimize its transmission.
        The returned KeyExpr's internal structure may differ from what you would have obtained through a simple key_expr.try_into(), to save time on detecting the optimizations that have been associated with it.
        """

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
    ):
        """Put data."""

    def delete(
        self,
        key_expr: _IntoKeyExpr,
        *,
        congestion_control: CongestionControl | None = None,
        priority: Priority | None = None,
        express: bool | None = None,
        attachment: _IntoZBytes | None = None,
    ):
        """Delete data."""

    @overload
    def get(
        self,
        selector: _IntoSelector,
        handler: _RustHandler[Reply] | None = None,
        *,
        target: QueryTarget | None = None,
        consolidation: ConsolidationMode | None = None,
        timeout: float | int | None = None,
        congestion_control: CongestionControl | None = None,
        priority: Priority | None = None,
        express: bool | None = None,
        payload: _IntoZBytes = None,
        encoding: _IntoEncoding | None = None,
        attachment: _IntoZBytes | None = None,
    ) -> Handler[Reply]:
        """Query data from the matching queryables in the system.
        Unless explicitly requested via GetBuilder::accept_replies, replies are guaranteed to have key expressions that match the requested selector.
        """

    @overload
    def get(
        self,
        selector: _IntoSelector,
        handler: _PythonHandler[Reply, _H],
        *,
        target: QueryTarget | None = None,
        consolidation: ConsolidationMode | None = None,
        timeout: float | int | None = None,
        congestion_control: CongestionControl | None = None,
        priority: Priority | None = None,
        express: bool | None = None,
        payload: _IntoZBytes = None,
        encoding: _IntoEncoding | None = None,
        attachment: _IntoZBytes | None = None,
    ) -> _H:
        """Query data from the matching queryables in the system.
        Unless explicitly requested via GetBuilder::accept_replies, replies are guaranteed to have key expressions that match the requested selector.
        """

    @overload
    def get(
        self,
        selector: _IntoSelector,
        handler: _PythonCallback[Reply],
        *,
        target: QueryTarget | None = None,
        consolidation: ConsolidationMode | None = None,
        timeout: float | int | None = None,
        congestion_control: CongestionControl | None = None,
        priority: Priority | None = None,
        express: bool | None = None,
        payload: _IntoZBytes = None,
        encoding: _IntoEncoding | None = None,
        attachment: _IntoZBytes | None = None,
    ) -> None:
        """Query data from the matching queryables in the system.
        Unless explicitly requested via GetBuilder::accept_replies, replies are guaranteed to have key expressions that match the requested selector.
        """

    @overload
    def declare_subscriber(
        self, key_expr: _IntoKeyExpr, handler: _RustHandler[Sample] | None = None
    ) -> Subscriber[Handler[Sample]]:
        """Create a Subscriber for the given key expression."""

    @overload
    def declare_subscriber(
        self, key_expr: _IntoKeyExpr, handler: _PythonHandler[Sample, _H]
    ) -> Subscriber[_H]:
        """Create a Subscriber for the given key expression."""

    @overload
    def declare_subscriber(
        self, key_expr: _IntoKeyExpr, handler: _PythonCallback[Sample]
    ) -> Subscriber[None]:
        """Create a Subscriber for the given key expression."""

    @overload
    def declare_queryable(
        self,
        key_expr: _IntoKeyExpr,
        handler: _RustHandler[Query] | None = None,
        *,
        complete: bool | None = None,
    ) -> Queryable[Handler[Query]]:
        """Create a Queryable for the given key expression."""

    @overload
    def declare_queryable(
        self,
        key_expr: _IntoKeyExpr,
        handler: _PythonHandler[Query, _H],
        *,
        complete: bool | None = None,
    ) -> Queryable[_H]:
        """Create a Queryable for the given key expression."""

    @overload
    def declare_queryable(
        self,
        key_expr: _IntoKeyExpr,
        handler: _PythonCallback[Query],
        *,
        complete: bool | None = None,
    ) -> Queryable[None]:
        """Create a Queryable for the given key expression."""

    def declare_publisher(
        self,
        key_expr: _IntoKeyExpr,
        *,
        encoding: _IntoEncoding | None = None,
        congestion_control: CongestionControl | None = None,
        priority: Priority | None = None,
        express: bool | None = None,
        reliability: Reliability | None = None,
    ) -> Publisher:
        """Create a Publisher for the given key expression."""

@final
class SessionInfo:
    def zid(self) -> ZenohId:
        """Return the ZenohId of the current zenoh Session."""

    def routers_zid(self) -> list[ZenohId]:
        """Return the ZenohId of the zenoh routers this process is currently connected to or the ZenohId of the current router if this code is run from a router (plugin)."""

    def peers_zid(self) -> list[ZenohId]:
        """Return the ZenohId of the zenoh peers this process is currently connected to."""

@final
class SetIntersectionLevel(Enum):
    DISJOINT = auto()
    INTERSECTS = auto()
    INCLUDES = auto()
    EQUALS = auto()

@final
class Subscriber(Generic[_H]):
    """A subscriber that provides data through a Handler.
    Subscribers can be created from a zenoh Session with the declare_subscriber function and the with function of the resulting builder.
    Subscribers are automatically undeclared when dropped."""

    def __enter__(self) -> Self: ...
    def __exit__(self, *_args, **_kwargs): ...
    @property
    def key_expr(self) -> KeyExpr: ...
    @property
    def handler(self) -> _H: ...
    def undeclare(self):
        """Close a Subscriber.
        Subscribers are automatically closed when dropped, but you may want to use this function to handle errors or close the Subscriber asynchronously.
        """

    @overload
    def try_recv(self: Subscriber[Handler[Sample]]) -> Sample | None: ...
    @overload
    def try_recv(self) -> Never: ...
    @overload
    def recv(self: Subscriber[Handler[Sample]]) -> Sample: ...
    @overload
    def recv(self) -> Never: ...
    @overload
    def __iter__(self: Subscriber[Handler[Sample]]) -> Handler[Sample]: ...
    @overload
    def __iter__(self) -> Never: ...

@final
class Timestamp:
    def get_time(self) -> datetime: ...
    def __str__(self) -> str: ...

@final
class WhatAmI(Enum):
    ROUTER = auto()
    PEER = auto()
    CLIENT = auto()

    def __str__(self) -> str: ...

@final
class WhatAmIMatcher:
    @classmethod
    def empty(cls) -> Self: ...
    def router(self) -> Self: ...
    def peer(self) -> Self: ...
    def client(self) -> Self: ...
    def is_empty(self) -> bool: ...
    def matches(self, whatami: WhatAmI) -> bool: ...
    def __str__(self) -> str: ...

_IntoWhatAmIMatcher = WhatAmIMatcher | str

@final
class ZBytes:
    """ZBytes contains the serialized bytes of user data."""

    def __new__(cls, bytes: bytes = None) -> Self: ...
    @classmethod
    def serialize(cls, obj: Any) -> Self:
        """Serialize object according to its type,
        using default or registered serializer."""

    def deserialize(self, tp: type[_T]) -> _T:
        """Deserialize bytes to the given types,
        using default or registered deserializer."""

    def __bool__(self) -> bool: ...
    def __len__(self) -> int: ...
    def __bytes__(self) -> bytes: ...
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
    """Open a zenoh Session."""

@overload
def scout(
    handler: _RustHandler[Hello] | None = None,
    what: _IntoWhatAmIMatcher | None = None,
    config: Config | None = None,
) -> Scout[Handler[Hello]]:
    """Scout for routers and/or peers.

    scout spawns a task that periodically sends scout messages and waits for Hello replies.
    Drop the returned Scout to stop the scouting task."""

@overload
def scout(
    handler: _PythonHandler[Hello, _H],
    what: _IntoWhatAmIMatcher | None = None,
    config: Config | None = None,
) -> Scout[_H]:
    """Scout for routers and/or peers.

    scout spawns a task that periodically sends scout messages and waits for Hello replies.
    Drop the returned Scout to stop the scouting task."""

@overload
def scout(
    handler: _PythonCallback[Hello],
    what: _IntoWhatAmIMatcher | None = None,
    config: Config | None = None,
) -> Scout[None]:
    """Scout for routers and/or peers.

    scout spawns a task that periodically sends scout messages and waits for Hello replies.
    Drop the returned Scout to stop the scouting task."""

@overload
def serializer(func: _F, /) -> _F:
    """Register a serializer for a given type, which will be used to serialize payloads.

    If the function is type-annotated, it will use the type of the first argument.
    Otherwise, the type has to be passed with the 'target' parameter."""

@overload
def serializer(*, target: type) -> Callable[[_F], _F]:
    """Register a serializer for a given type, which will be used to serialize payloads.

    If the function is type-annotated, it will use the type of the first argument.
    Otherwise, the type has to be passed with the 'target' parameter."""

@overload
def deserializer(func: _F, /) -> _F:
    """Register a deserializer for a given type, which will be used to deserialize payload.

    If the function is type-annotated, it will use the return type.
    Otherwise, the type has to be passed with the 'target' parameter."""

@overload
def deserializer(*, target: type) -> Callable[[_F], _F]:
    """Register a deserializer for a given type, which will be used to deserialize payload.

    If the function is type-annotated, it will use the return type.
    Otherwise, the type has to be passed with the 'target' parameter."""
