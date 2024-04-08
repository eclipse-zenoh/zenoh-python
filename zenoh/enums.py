#
# Copyright (c) 2022 ZettaScale Technology
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
from .zenoh import _Encoding, _SampleKind, _CongestionControl, _Priority, _Reliability, _QueryTarget, _QueryConsolidation

class Priority(_Priority):
    """
    The priority of a sending operation.

    They are ordered à la Linux priority:
    ``Priority.REAL_TIME() < Priority.INTERACTIVE_HIGH() < Priority.INTERACTIVE_LOW() < Priority.DATA() < Priority.BACKGROUND()``
    """
    def __new__(cls, inner: _Priority):
        return super().__new__(cls, inner)
    @staticmethod
    def REAL_TIME() -> 'Priority':
        return Priority(_Priority.REAL_TIME)
    @staticmethod
    def INTERACTIVE_HIGH() -> 'Priority':
        return Priority(_Priority.INTERACTIVE_HIGH)
    @staticmethod
    def INTERACTIVE_LOW() -> 'Priority':
        return Priority(_Priority.INTERACTIVE_LOW)
    @staticmethod
    def DATA_HIGH() -> 'Priority':
        return Priority(_Priority.DATA_HIGH)
    @staticmethod
    def DATA() -> 'Priority':
        return Priority(_Priority.DATA)
    @staticmethod
    def DATA_LOW() -> 'Priority':
        return Priority(_Priority.DATA_LOW)
    @staticmethod
    def BACKGROUND() -> 'Priority':
        return Priority(_Priority.BACKGROUND)
    def __eq__(self, other) -> bool:
        return super().__eq__(other)
    def __ne__(self, other) -> bool:
        return not self.__eq__(other)
    def __eq__(self, other) -> bool:
        return super().__eq__(other)
    def __lt__(self, other) -> bool:
        return super().__lt__(other)
    def __le__(self, other) -> bool:
        return super().__le__(other)
    def __gt__(self, other) -> bool:
        return super().__gt__(other)
    def __ge__(self, other) -> bool:
        return super().__ge__(other)
    
Priority.DEFAULT = Priority.DATA()

class SampleKind(_SampleKind):
    "Similar to an HTTP METHOD: only PUT and DELETE are currently supported."
    def __new__(cls, inner: _SampleKind):
        return super().__new__(cls, inner)
    @staticmethod
    def PUT() -> 'SampleKind':
        return SampleKind(_SampleKind.PUT)
    @staticmethod
    def DELETE() -> 'SampleKind':
        return SampleKind(_SampleKind.DELETE)
    def __eq__(self, other) -> bool:
        return super().__eq__(other)
    def __ne__(self, other) -> bool:
        return not self.__eq__(other)

class CongestionControl(_CongestionControl):
    """
    Defines the network's behaviour regarding a message when heavily congested.
    """
    def __new__(cls, inner: _CongestionControl):
        return super().__new__(cls, inner)
    @staticmethod
    def DROP() -> 'CongestionControl':
        "Allows the message to be dropped if all buffers are full."
        return CongestionControl(_CongestionControl.DROP)
    @staticmethod
    def BLOCK() -> 'CongestionControl':
        """
        Prevents the message from being dropped at all cost.
        In the face of heavy congestion on a part of the network, this could result in your publisher node blocking.
        """
        return CongestionControl(_CongestionControl.BLOCK)
    def __eq__(self, other) -> bool:
        return super().__eq__(other)
    def __ne__(self, other) -> bool:
        return not self.__eq__(other)
    
CongestionControl.DEFAULT = CongestionControl.DROP()

class Encoding(_Encoding):
    def __new__(cls, inner: _Encoding):
        return super().__new__(cls, inner)
    @staticmethod
    def from_str(s: str) -> 'Encoding':
        return super(Encoding, Encoding).from_str(s)
    def with_schema(self, s: str):
        super().with_schema(s)
    @staticmethod
    def ZENOH_BYTES() -> 'Encoding':
        return Encoding(_Encoding.ZENOH_BYTES)
    @staticmethod
    def ZENOH_INT() -> 'Encoding':
        return Encoding(_Encoding.ZENOH_INT)
    @staticmethod
    def ZENOH_UINT() -> 'Encoding':
        return Encoding(_Encoding.ZENOH_UINT)
    @staticmethod
    def ZENOH_FLOAT() -> 'Encoding':
        return Encoding(_Encoding.ZENOH_FLOAT)
    @staticmethod
    def ZENOH_BOOL() -> 'Encoding':
        return Encoding(_Encoding.ZENOH_BOOL)
    @staticmethod
    def ZENOH_STRING() -> 'Encoding':
        return Encoding(_Encoding.ZENOH_STRING)
    @staticmethod
    def ZENOH_ERROR() -> 'Encoding':
        return Encoding(_Encoding.ZENOH_ERROR)
    @staticmethod
    def APPLICATION_OCTET_STREAM() -> 'Encoding':
        return Encoding(_Encoding.APPLICATION_OCTET_STREAM)
    @staticmethod
    def TEXT_PLAIN() -> 'Encoding':
        return Encoding(_Encoding.TEXT_PLAIN)
    @staticmethod
    def APPLICATION_JSON() -> 'Encoding':
        return Encoding(_Encoding.APPLICATION_JSON)
    @staticmethod
    def TEXT_JSON() -> 'Encoding':
        return Encoding(_Encoding.TEXT_JSON)
    @staticmethod
    def APPLICATION_CDR() -> 'Encoding':
        return Encoding(_Encoding.APPLICATION_CDR)
    @staticmethod
    def APPLICATION_CBOR() -> 'Encoding':
        return Encoding(_Encoding.APPLICATION_CBOR)
    @staticmethod
    def APPLICATION_YAML() -> 'Encoding':
        return Encoding(_Encoding.APPLICATION_YAML)
    @staticmethod
    def TEXT_YAML() -> 'Encoding':
        return Encoding(_Encoding.TEXT_YAML)
    @staticmethod
    def TEXT_JSON5() -> 'Encoding':
        return Encoding(_Encoding.TEXT_JSON5)
    pub const APPLICATION_PYTHON_SERIALIZED_OBJECT: Self =
        Self(Encoding::APPLICATION_PYTHON_SERIALIZED_OBJECT);
    @staticmethod
    def APPLICATION_PROTOBUF() -> 'Encoding':
        return Encoding(_Encoding.APPLICATION_PROTOBUF)
    #[classattr]
    pub const APPLICATION_JAVA_SERIALIZED_OBJECT: Self =
        Self(Encoding::APPLICATION_JAVA_SERIALIZED_OBJECT);
    @staticmethod
    def APPLICATION_OPENMETRICS_TEXT() -> 'Encoding':
        return Encoding(_Encoding.APPLICATION_OPENMETRICS_TEXT)
    @staticmethod
    def IMAGE_PNG() -> 'Encoding':
        return Encoding(_Encoding.IMAGE_PNG)
    @staticmethod
    def IMAGE_JPEG() -> 'Encoding':
        return Encoding(_Encoding.IMAGE_JPEG)
    @staticmethod
    def IMAGE_GIF() -> 'Encoding':
        return Encoding(_Encoding.IMAGE_GIF)
    @staticmethod
    def IMAGE_BMP() -> 'Encoding':
        return Encoding(_Encoding.IMAGE_BMP)
    @staticmethod
    def IMAGE_WEBP() -> 'Encoding':
        return Encoding(_Encoding.IMAGE_WEBP)
    @staticmethod
    def APPLICATION_XML() -> 'Encoding':
        return Encoding(_Encoding.APPLICATION_XML)
    #[classattr]
    pub const APPLICATION_X_WWW_FORM_URLENCODED: Self =
        Self(Encoding::APPLICATION_X_WWW_FORM_URLENCODED);
    @staticmethod
    def TEXT_HTML() -> 'Encoding':
        return Encoding(_Encoding.TEXT_HTML)
    @staticmethod
    def TEXT_XML() -> 'Encoding':
        return Encoding(_Encoding.TEXT_XML)
    @staticmethod
    def TEXT_CSS() -> 'Encoding':
        return Encoding(_Encoding.TEXT_CSS)
    @staticmethod
    def TEXT_JAVASCRIPT() -> 'Encoding':
        return Encoding(_Encoding.TEXT_JAVASCRIPT)
    @staticmethod
    def TEXT_MARKDOWN() -> 'Encoding':
        return Encoding(_Encoding.TEXT_MARKDOWN)
    @staticmethod
    def TEXT_CSV() -> 'Encoding':
        return Encoding(_Encoding.TEXT_CSV)
    @staticmethod
    def APPLICATION_SQL() -> 'Encoding':
        return Encoding(_Encoding.APPLICATION_SQL)
    @staticmethod
    def APPLICATION_COAP_PAYLOAD() -> 'Encoding':
        return Encoding(_Encoding.APPLICATION_COAP_PAYLOAD)
    @staticmethod
    def APPLICATION_JSON_PATCH_JSON() -> 'Encoding':
        return Encoding(_Encoding.APPLICATION_JSON_PATCH_JSON)
    @staticmethod
    def APPLICATION_JSON_SEQ() -> 'Encoding':
        return Encoding(_Encoding.APPLICATION_JSON_SEQ)
    @staticmethod
    def APPLICATION_JSONPATH() -> 'Encoding':
        return Encoding(_Encoding.APPLICATION_JSONPATH)
    @staticmethod
    def APPLICATION_JWT() -> 'Encoding':
        return Encoding(_Encoding.APPLICATION_JWT)
    @staticmethod
    def APPLICATION_MP4() -> 'Encoding':
        return Encoding(_Encoding.APPLICATION_MP4)
    @staticmethod
    def APPLICATION_SOAP_XML() -> 'Encoding':
        return Encoding(_Encoding.APPLICATION_SOAP_XML)
    @staticmethod
    def APPLICATION_YANG() -> 'Encoding':
        return Encoding(_Encoding.APPLICATION_YANG)
    @staticmethod
    def AUDIO_AAC() -> 'Encoding':
        return Encoding(_Encoding.AUDIO_AAC)
    @staticmethod
    def AUDIO_FLAC() -> 'Encoding':
        return Encoding(_Encoding.AUDIO_FLAC)
    @staticmethod
    def AUDIO_MP4() -> 'Encoding':
        return Encoding(_Encoding.AUDIO_MP4)
    @staticmethod
    def AUDIO_OGG() -> 'Encoding':
        return Encoding(_Encoding.AUDIO_OGG)
    @staticmethod
    def AUDIO_VORBIS() -> 'Encoding':
        return Encoding(_Encoding.AUDIO_VORBIS)
    @staticmethod
    def VIDEO_H261() -> 'Encoding':
        return Encoding(_Encoding.VIDEO_H261)
    @staticmethod
    def VIDEO_H263() -> 'Encoding':
        return Encoding(_Encoding.VIDEO_H263)
    @staticmethod
    def VIDEO_H264() -> 'Encoding':
        return Encoding(_Encoding.VIDEO_H264)
    @staticmethod
    def VIDEO_H265() -> 'Encoding':
        return Encoding(_Encoding.VIDEO_H265)
    @staticmethod
    def VIDEO_H266() -> 'Encoding':
        return Encoding(_Encoding.VIDEO_H266)
    @staticmethod
    def VIDEO_MP4() -> 'Encoding':
        return Encoding(_Encoding.VIDEO_MP4)
    @staticmethod
    def VIDEO_OGG() -> 'Encoding':
        return Encoding(_Encoding.VIDEO_OGG)
    @staticmethod
    def VIDEO_RAW() -> 'Encoding':
        return Encoding(_Encoding.VIDEO_RAW)
    @staticmethod
    def VIDEO_VP8() -> 'Encoding':
        return Encoding(_Encoding.VIDEO_VP8)
    @staticmethod
    def VIDEO_VP9() -> 'Encoding':
        return Encoding(_Encoding.VIDEO_VP9)
    def __eq__(self, other) -> bool:
        return super().__eq__(other)
    def __ne__(self, other) -> bool:
        return not self.__eq__(other)

class Reliability(_Reliability):
    "Used by subscribers to inform the network of the reliability it wishes to obtain."
    def __new__(cls, inner: _Reliability):
        return super().__new__(cls, inner)
    @staticmethod
    def BEST_EFFORT() -> 'CongestionControl':
        "Informs the network that dropping some messages is acceptable"
        return Reliability(_Reliability.BEST_EFFORT)
    @staticmethod
    def RELIABLE() -> 'CongestionControl':
        """
        Informs the network that this subscriber wishes for all publications to reliably reach it.

        Note that if a publisher puts a sample with the ``CongestionControl.DROP()`` option, this reliability
        requirement may be infringed to prevent slow readers from blocking the network.
        """
        return Reliability(_Reliability.RELIABLE)
    def __eq__(self, other) -> bool:
        return super().__eq__(other)
    def __ne__(self, other) -> bool:
        return not self.__eq__(other)

class QueryTarget(_QueryTarget):
    def __new__(cls, inner: _QueryTarget):
        return super().__new__(cls, inner)
    @staticmethod
    def BEST_MATCHING() -> 'QueryTarget':
        return QueryTarget(_QueryTarget.BEST_MATCHING)
    @staticmethod
    def ALL() -> 'QueryTarget':
        return QueryTarget(_QueryTarget.ALL)
    @staticmethod
    def ALL_COMPLETE() -> 'QueryTarget':
        return QueryTarget(_QueryTarget.ALL_COMPLETE)
    def __eq__(self, other) -> bool:
        return super().__eq__(other)
    def __ne__(self, other) -> bool:
        return not self.__eq__(other)

class QueryConsolidation(_QueryConsolidation):
    def __new__(cls, inner: _QueryConsolidation):
        return super().__new__(cls, inner)
    @staticmethod
    def AUTO() -> 'QueryConsolidation':
        return QueryConsolidation(_QueryConsolidation.AUTO)
    @staticmethod
    def NONE() -> 'QueryConsolidation':
        return QueryConsolidation(_QueryConsolidation.NONE)
    @staticmethod
    def MONOTONIC() -> 'QueryConsolidation':
        return QueryConsolidation(_QueryConsolidation.MONOTONIC)
    @staticmethod
    def LATEST() -> 'QueryConsolidation':
        return QueryConsolidation(_QueryConsolidation.LATEST)
    def __eq__(self, other) -> bool:
        return super().__eq__(other)
    def __ne__(self, other) -> bool:
        return not self.__eq__(other)