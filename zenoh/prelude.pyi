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
__all__ = [
    "Config",
    "WhatAmI",
    "ZenohId",
    "Encoding",
    "KeyExpr",
    "CongestionControl",
    "Priority",
    "ConsolidationMode",
    "QueryTarget",
    "Sample",
    "SampleKind",
    "Selector",
    "Session",
    "Reliability",
    "Value",
]

from typing import Self

from zenoh.config import Config as Config, WhatAmI as WhatAmI, ZenohId as ZenohId
from zenoh.key_expr import KeyExpr as KeyExpr
from zenoh.publication import CongestionControl as CongestionControl, Priority as Priority
from zenoh.query import ConsolidationMode as ConsolidationMode, QueryTarget as QueryTarget
from zenoh.sample import Sample as Sample, SampleKind as SampleKind
from zenoh.selector import Selector as Selector
from zenoh.session import Session as Session
from zenoh.subscriber import Reliability as Reliability
from zenoh.value import Value as Value


class Encoding:
    """Default encoding values used by Zenoh.
    An encoding has a similar role to Content-type in HTTP: it indicates, when present, how data should be interpreted by the application.
    Please note the Zenoh protocol does not impose any encoding value nor it operates on it. It can be seen as some optional metadata that is carried over by Zenoh in such a way the application may perform different operations depending on the encoding value.
    A set of associated constants are provided to cover the most common encodings for user convenience. This is parcticular useful in helping Zenoh to perform additional network optimizations.
    """

    def __new__(cls, encoding: str | None = None) -> Self: ...

    ZENOH_BYTES: Self
    """Just some bytes.
    
    Constant alias for string: "zenoh/bytes"."""
    ZENOH_INT: Self
    """A VLE-encoded signed little-endian integer. Either 8bit, 16bit, 32bit, or 64bit. Binary reprensentation uses two's complement.
    
    Constant alias for string: "zenoh/int"."""
    ZENOH_UINT: Self
    """A VLE-encoded little-endian unsigned integer. Either 8bit, 16bit, 32bit, or 64bit.

    Constant alias for string: "zenoh/uint"."""
    ZENOH_FLOAT: Self
    """A VLE-encoded float. Either little-endian 32bit or 64bit. Binary representation uses IEEE 754-2008 binary32 or binary64, respectively.

    Constant alias for string: "zenoh/float"."""
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
    TEST_JSON5: Self
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

    def with_schema(self, schema: str) -> Self:
        """Set a schema to this encoding. Zenoh does not define what a schema is and its semantichs is left to the implementer. E.g. a common schema for text/plain encoding is utf-8."""


IntoEncoding = Encoding | str
