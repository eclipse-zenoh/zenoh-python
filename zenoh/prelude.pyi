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

from enum import Enum, auto
from typing import Self

from zenoh.config import Config, WhatAmI, ZenohId
from zenoh.key_expr import KeyExpr
from zenoh.publication import CongestionControl, Priority
from zenoh.query import ConsolidationMode, QueryTarget
from zenoh.sample import Sample, SampleKind
from zenoh.selector import Selector
from zenoh.session import Session
from zenoh.subscriber import Reliability
from zenoh.value import Value

class Encoding(Enum):
    """Default encoding values used by Zenoh.
    An encoding has a similar role to Content-type in HTTP: it indicates, when present, how data should be interpreted by the application.
    Please note the Zenoh protocol does not impose any encoding value nor it operates on it. It can be seen as some optional metadata that is carried over by Zenoh in such a way the application may perform different operations depending on the encoding value.
    A set of associated constants are provided to cover the most common encodings for user convenience. This is parcticular useful in helping Zenoh to perform additional network optimizations.
    """

    def __new__(cls, arg: IntoEncoding) -> Self: ...

    ZENOH_BYTES = auto()
    ZENOH_INT = auto()
    ZENOH_UINT = auto()
    ZENOH_FLOAT = auto()
    ZENOH_BOOL = auto()
    ZENOH_STRING = auto()
    ZENOH_ERROR = auto()
    APPLICATION_OCTET_STREAM = auto()
    TEXT_PLAIN = auto()
    APPLICATION_JSON = auto()
    TEXT_JSON = auto()
    APPLICATION_CDR = auto()
    APPLICATION_CBOR = auto()
    APPLICATION_YAML = auto()
    TEXT_YAML = auto()
    TEST_JSON5 = auto()
    APPLICATION_PYTHON_SERIALIZED_OBJECT = auto()
    APPLICATION_PROTOBUF = auto()
    APPLICATION_JAVA_SERIALIZED_OBJECT = auto()
    APPLICATION_OPENMETRICS_TEXT = auto()
    IMAGE_PNG = auto()
    IMAGE_JPEG = auto()
    IMAGE_GIF = auto()
    IMAGE_BMP = auto()
    IMAGE_WEBP = auto()
    APPLICATION_XML = auto()
    APPLICATION_X_WWW_FORM_URLENCODED = auto()
    TEXT_HTML = auto()
    TEXT_XML = auto()
    TEXT_CSS = auto()
    TEXT_JAVASCRIPT = auto()
    TEXT_MARKDOWN = auto()
    TEXT_CSV = auto()
    APPLICATION_SQL = auto()
    APPLICATION_COAP_PAYLOAD = auto()
    APPLICATION_JSON_PATCH_JSON = auto()
    APPLICATION_JSON_SEQ = auto()
    APPLICATION_JSONPATH = auto()
    APPLICATION_JWT = auto()
    APPLICATION_MP4 = auto()
    APPLICATION_SOAP_XML = auto()
    APPLICATION_YANG = auto()
    AUDIO_AAC = auto()
    AUDIO_FLAC = auto()
    AUDIO_MP4 = auto()
    AUDIO_OGG = auto()
    AUDIO_VORBIS = auto()
    VIDEO_H261 = auto()
    VIDEO_H263 = auto()
    VIDEO_H264 = auto()
    VIDEO_H265 = auto()
    VIDEO_H266 = auto()
    VIDEO_MP4 = auto()
    VIDEO_OGG = auto()
    VIDEO_RAW = auto()
    VIDEO_VP8 = auto()
    VIDEO_VP9 = auto()

    def with_schema(self, schema: str) -> Self:
        """Set a schema to this encoding. Zenoh does not define what a schema is and its semantichs is left to the implementer. E.g. a common schema for text/plain encoding is utf-8."""

IntoEncoding = Encoding | str
