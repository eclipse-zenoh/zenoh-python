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
    `Priority.REAL_TIME() < Priority.INTERACTIVE_HIGH() < Priority.INTERACTIVE_LOW() < Priority.DATA() < Priority.BACKGROUND()`
    """
    def __new__(cls, inner: _SampleKind):
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

class Encoding(_Encoding):
    def __new__(cls, inner: _Encoding):
        return super().__new__(cls, inner)
    @staticmethod
    def from_str(s: str) -> 'Encoding':
        return super(Encoding, Encoding).from_str(s)
    def append(self, s: str):
        super().append(s)
    @staticmethod
    def EMPTY() -> 'Encoding':
        return Encoding(_Encoding.EMPTY )
    @staticmethod
    def APP_OCTET_STREAM() -> 'Encoding':
        return Encoding(_Encoding.APP_OCTET_STREAM)
    @staticmethod
    def APP_CUSTOM() -> 'Encoding':
        return Encoding(_Encoding.APP_CUSTOM)
    @staticmethod
    def TEXT_PLAIN() -> 'Encoding':
        return Encoding(_Encoding.TEXT_PLAIN)
    @staticmethod
    def APP_PROPERTIES() -> 'Encoding':
        return Encoding(_Encoding.APP_PROPERTIES)
    @staticmethod
    def APP_JSON() -> 'Encoding':
        return Encoding(_Encoding.APP_JSON)
    @staticmethod
    def APP_SQL() -> 'Encoding':
        return Encoding(_Encoding.APP_SQL)
    @staticmethod
    def APP_INTEGER() -> 'Encoding':
        return Encoding(_Encoding.APP_INTEGER)
    @staticmethod
    def APP_FLOAT() -> 'Encoding':
        return Encoding(_Encoding.APP_FLOAT)
    @staticmethod
    def APP_XML() -> 'Encoding':
        return Encoding(_Encoding.APP_XML)
    @staticmethod
    def APP_XHTML_XML() -> 'Encoding':
        return Encoding(_Encoding.APP_XHTML_XML)
    @staticmethod
    def APP_X_WWW_FORM_URLENCODED() -> 'Encoding':
        return Encoding(_Encoding.APP_X_WWW_FORM_URLENCODED)
    @staticmethod
    def TEXT_JSON() -> 'Encoding':
        return Encoding(_Encoding.TEXT_JSON)
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
    def TEXT_CSV() -> 'Encoding':
        return Encoding(_Encoding.TEXT_CSV)
    @staticmethod
    def TEXT_JAVASCRIPT() -> 'Encoding':
        return Encoding(_Encoding.TEXT_JAVASCRIPT)
    @staticmethod
    def IMAGE_JPEG() -> 'Encoding':
        return Encoding(_Encoding.IMAGE_JPEG)
    @staticmethod
    def IMAGE_PNG() -> 'Encoding':
        return Encoding(_Encoding.IMAGE_PNG)
    @staticmethod
    def IMAGE_GIF() -> 'Encoding':
        return Encoding(_Encoding.IMAGE_GIF)
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

        Note that if a publisher puts a sample with the `CongestionControl.DROP()` option, this reliability
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