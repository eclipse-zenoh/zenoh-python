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
import abc
from typing import Union, Tuple, Optional, List
import json

from .enums import Encoding, SampleKind
from .zenoh import _Value, _Encoding, _Sample, _SampleKind, _Reply, _ZenohId, _Timestamp, _Hello
from .keyexpr import KeyExpr, IntoKeyExpr

class IValue:
    "The IValue interface exposes how to recover a value's payload in a binary-serialized format, as well as that format's encoding."
    @property
    @abc.abstractmethod
    def payload(self) -> bytes:
        ...
    
    @property
    @abc.abstractmethod
    def encoding(self) -> Encoding:
        ...

IntoValue = Union[IValue, bytes, str, int, float, object]

class Value(_Value, IValue):
    """
    A Value is a pair of a binary payload, and a mime-type-like encoding string.
    
    When constructed with `encoding==None`, the encoding will be selected depending on the payload's type.
    """
    def __new__(cls, payload: IntoValue, encoding: Encoding=None):
        if encoding is None:
            if isinstance(payload, Value):
                return payload
            return Value.autoencode(payload)
        else:
            if not isinstance(payload, bytes):
                raise TypeError("`encoding` was passed, but `payload` is not of type `bytes`")
            return Value.new(payload, encoding)
    
    @staticmethod
    def autoencode(value: IntoValue) -> 'Value':
        if isinstance(value, IValue):
            return Value.new(value.payload, value.encoding)
        if isinstance(value, bytes):
            return Value.new(value, Encoding.APP_OCTET_STREAM())
        if isinstance(value, str):
            return Value.new(value.encode(), Encoding.TEXT_PLAIN())
        if isinstance(value, int):
            return Value.new(f"{value}".encode(), Encoding.APP_INTEGER())
        if isinstance(value, float):
            return Value.new(f"{value}".encode(), Encoding.APP_FLOAT())
        return Value.new(json.dumps(value).encode(), Encoding.APP_JSON())
    
    @staticmethod
    def new(payload: bytes, encoding: Encoding = None) -> 'Value':
        return Value._upgrade_(_Value.new(payload, encoding))

    @property
    def payload(self) -> bytes:
        return super().payload

    @payload.setter
    def payload(self, payload: bytes):
        super().with_payload(payload)

    @property
    def encoding(self) -> Encoding:
        return Encoding(super().encoding)

    @encoding.setter
    def encoding(self, encoding: Encoding):
        super().with_encoding(encoding)

    @staticmethod
    def _upgrade_(inner: _Value) -> 'Value':
        if inner is None:
            return None
        if isinstance(inner, Value):
            return inner
        return _Value.__new__(Value, inner)

class ZenohId(_ZenohId):
    """A Zenoh UUID"""
    @staticmethod
    def _upgrade_(this: _ZenohId) -> 'ZenohId':
        return _ZenohId.__new__(ZenohId, this)
    def __str__(self) -> str:
        return super().__str__()
    def __repr__(self) -> str:
        return str(self)

class Timestamp(_Timestamp):
    """
    A timestamp taken from the Zenoh HLC (Hybrid Logical Clock).

    These timestamps are guaranteed to be unique, as each machine annotates its perceived time with a UUID, which is used as the least significant part of the comparison operation.
    """
    @staticmethod
    def _upgrade_(this: _Timestamp) -> 'Timestamp':
        return _Timestamp.__new__(Timestamp, this)
    @property
    def seconds_since_unix_epoch(self) -> float:
        """
        Returns the number of seconds since the Unix Epoch.

        You shouldn't use this for comparison though, and rely on comparison operators between members of this class.
        """
        return super().seconds_since_unix_epoch


IntoSample = Union[_Sample, Tuple[IntoKeyExpr, IntoValue, SampleKind], Tuple[KeyExpr, IntoValue]]
class Sample(_Sample):
    """
    A KeyExpr-Value pair, annotated with the kind (PUT or DELETE) of publication used to emit it and a timestamp.
    """
    def __new__(cls, key: IntoKeyExpr, value: IntoValue, kind: SampleKind = None, timestamp: Timestamp = None):
        kind = _SampleKind.PUT if kind is None else kind
        return Sample._upgrade_(super().new(KeyExpr(key), Value(value), kind, timestamp))
    @property
    def key_expr(self) -> KeyExpr:
        "The sample's key expression"
        return KeyExpr(super().key_expr)
    @property
    def value(self) -> Value:
        "The sample's value"
        return Value._upgrade_(super().value)
    @property
    def payload(self) -> bytes:
        "A shortcut to `self.value.payload`"
        return super().payload
    @property
    def encoding(self) -> Encoding:
        "A shortcut to `self.value.encoding`"
        return Encoding(super().encoding)
    @property
    def kind(self) -> SampleKind:
        "The sample's kind"
        return SampleKind(super().kind)
    @property
    def timestamp(self) -> Optional[Timestamp]:
        "The sample's  timestamp. May be None."
        ts = super().timestamp
        return None if ts is None else Timestamp._upgrade_(ts)
    @staticmethod
    def _upgrade_(inner: _Sample) -> 'Sample':
        if isinstance(inner, Sample):
            return inner
        return _Sample.__new__(Sample, inner)

class Reply(_Reply):
    def __new__(cls, inner: _Reply):
        return super().__new__(cls, inner)
    @property
    def replier_id(self) -> ZenohId:
        "The reply's sender's id."
        return ZenohId._upgrade_(super().replier_id)
    @property
    def ok(self) -> Sample:
        """
        The reply's inner data sample.

        Raises a ZError if the `self` is actually an `err` reply.
        """
        return Sample._upgrade_(super().ok)
    @property
    def err(self) -> Value:
        """
        The reply's error value.

        Raises a ZError if the `self` is actually an `ok` reply.
        """
        return Value._upgrade_(super().err)

class Hello(_Hello):
    "Represents a single Zenoh node discovered through scouting."
    @property
    def zid(self) -> ZenohId:
        "The node's Zenoh UUID."
        zid = super().zid
        return None if zid is None else ZenohId._upgrade_(zid)
    @property
    def whatami(self) -> str:
        "The node's type, returning either None, 'peer', 'router', or 'client'."
        return super().whatami
    @property
    def locators(self) -> List[str]:
        "The locators through which this node may be adressed."
        return super().locators
    @staticmethod
    def _upgrade_(inner: _Hello) -> 'Sample':
        if isinstance(inner, Hello):
            return inner
        return _Hello.__new__(Hello, inner)
    def __str__(self):
        return super().__str__()