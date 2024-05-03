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
from typing import Union, Tuple, Optional, List, Dict, TypeVar, Iterable, Iterator, overload
import json

from .enums import Encoding, SampleKind, Priority, CongestionControl
from .zenoh import _Value, _Encoding, _Sample, _SampleKind, _Reply, _ZenohId, _Timestamp, _Hello, _QoS, _Attachment
from .keyexpr import KeyExpr, IntoKeyExpr

T = TypeVar("T")


class IValue:
    "The IValue interface exposes how to recover a value's payload in a binary-serialized format, as well as that format's encoding."

    @property
    @abc.abstractmethod
    def payload(self) -> bytes:
        "The value itself, as an array of bytes"
        ...

    @property
    @abc.abstractmethod
    def encoding(self) -> Encoding:
        "The value's encoding"
        ...


IntoValue = Union[IValue, bytes, str, int, float, object]


class Value(_Value, IValue):
    """
    A Value is a pair of a binary payload, and a mime-type-like encoding string.

    When constructed with ``encoding==None``, the encoding will be selected depending on the payload's type.
    """

    def __new__(cls, payload: IntoValue, encoding: Encoding = None):
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
        "Automatically encodes the value based on its type"
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
    def get_time(self) -> int:
        """
        Returns the time part, as generated by the Zenoh HLC in NTP64 format (See https://datatracker.ietf.org/doc/html/rfc5905#section-6).
        """
        return super().time

    @property
    def seconds_since_unix_epoch(self) -> float:
        """
        Returns the number of seconds since the Unix Epoch.

        Considering the large number of seconds since the Unix Epoch, the precision of the resulting f64 is in the order of microseconds.
        Therefore, it should not be used for comparison. Directly comparing Timestamp objects is preferable.
        """
        return super().seconds_since_unix_epoch


class QoS(_QoS):
    """
    Quality of Service settings.
    """

    def __new__(cls):
        return super().new()

    @property
    def priority(self) -> Priority:
        "Priority"
        return Priority(super().priority)

    @property
    def congestion_control(self) -> CongestionControl:
        "Congestion control"
        return CongestionControl(super().congestion_control)

    @property
    def express(self) -> bool:
        "Express flag: if True, the message is not batched during transmission, in order to reduce latency."
        return super().express

    @staticmethod
    def _upgrade_(inner: _QoS) -> 'QoS':
        if isinstance(inner, QoS):
            return inner
        return _QoS.__new__(QoS, inner)


QoS.DEFAULT = QoS()

IntoAttachment = (
    Dict[Union[str, bytes], Union[str, bytes]]
    | Iterable[Tuple[Union[str, bytes], Union[str, bytes]]]
)


def _into_bytes(v: Union[str, bytes]) -> bytes:
    return v if isinstance(v, bytes) else v.encode()


class Attachment(_Attachment):
    def __new__(cls, into: IntoAttachment, **kw):
        self = Attachment._upgrade_(super().new())
        self.update(into, **kw)
        return self

    @overload
    def get(self, key: Union[str, bytes], default: T) -> Union[bytes, T]: ...
    @overload
    def get(self, key: Union[str, bytes]) -> Optional[bytes]: ...

    def get(self, key: Union[str, bytes], default=None):
        value = super().get(_into_bytes(key))
        return value if value is not None else default

    def insert(self, key: Union[str, bytes], value: Union[str, bytes]):
        return super().insert(_into_bytes(key), _into_bytes(value))

    def update(self, into: IntoAttachment, **kw):
        if isinstance(into, dict):
            into = into.items()
        super().update([(_into_bytes(k), _into_bytes(v)) for k, v in into])
        if kw:
            self.update(kw)

    def keys(self) -> List[bytes]:
        return super().keys()

    def values(self) -> List[bytes]:
        return super().values()

    def items(self) -> List[Tuple[bytes, bytes]]:
        return super().items()

    def __len__(self) -> int:
        return super().__len__()

    def __bool__(self) -> bool:
        return super().__bool__()

    def __getitem__(self, key: Union[str, bytes]) -> bytes:
        value = self.get(key)
        if value is None:
            raise KeyError(key)
        return value

    def __setitem__(self, key: Union[str, bytes], value: Union[str, bytes]):
        self.insert(key, value)

    def __iter__(self) -> Iterator[bytes]:
        return iter(self.keys())

    @staticmethod
    def _upgrade_(inner: _Attachment) -> "Attachment":
        if isinstance(inner, Attachment):
            return inner
        return _Attachment.__new__(Attachment, inner)


IntoSample = Union[_Sample, Tuple[IntoKeyExpr, IntoValue, SampleKind], Tuple[KeyExpr, IntoValue]]


class Sample(_Sample):
    """
    A KeyExpr-Value pair, annotated with the kind (PUT or DELETE) of publication used to emit it and a timestamp.
    """

    def __new__(cls, key: IntoKeyExpr, value: IntoValue, kind: SampleKind = None, qos: QoS = None,
                timestamp: Timestamp = None, attachment: IntoAttachment = None):
        kind = _SampleKind.PUT if kind is None else kind
        qos = QoS.DEFAULT if qos is None else qos
        attachment = Attachment(attachment) if attachment is not None else attachment
        return Sample._upgrade_(super().new(KeyExpr(key), Value(value), qos, kind, timestamp, attachment))

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
        "A shortcut to ``self.value.payload``"
        return super().payload

    @property
    def encoding(self) -> Encoding:
        "A shortcut to ``self.value.encoding``"
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

    @property
    def qos(self) -> QoS:
        "Quality of service settings the sample was sent with"
        return QoS._upgrade_(super().qos)

    @property
    def attachment(self) -> Optional[Attachment]:
        """The sample attachment"""
        attachment = super().attachment
        return Attachment._upgrade_(attachment) if attachment is not None else None

    @staticmethod
    def _upgrade_(inner: _Sample) -> 'Sample':
        if isinstance(inner, Sample):
            return inner
        return _Sample.__new__(Sample, inner)


class Reply(_Reply):
    """
    A reply to a query (``Session.get``).
    
    A single query can result in multiple replies from multiple queryables.
    """

    def __new__(cls, inner: _Reply):
        return super().__new__(cls, inner)

    @property
    def replier_id(self) -> ZenohId:
        "The reply's sender's id."
        return ZenohId._upgrade_(super().replier_id)

    @property
    def is_ok(self) -> bool:
        """
        Checks if the reply is `ok`.

        Returns `True` if the reply is `ok`, `False` otherwise
        """
        return super.is_ok()

    @property
    def ok(self) -> Sample:
        """
        The reply's inner data sample.

        Raises a ``ZError`` if the ``self`` is actually an ``err`` reply.
        """
        return Sample._upgrade_(super().ok)

    @property
    def err(self) -> Value:
        """
        The reply's error value.

        Raises a ``ZError`` if the ``self`` is actually an ``ok`` reply.
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
