import abc
from typing import Union, Tuple
import json

from .enums import Encoding, SampleKind
from .zenoh import _Value, _Encoding, _Sample, _SampleKind, _Reply
from .keyexpr import KeyExpr, IntoKeyExpr

class IValue:
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
	def __new__(cls, payload: IntoValue, encoding: Encoding=None):
		if encoding is None:
			return Value.autoencode(payload)
		else:
			if not isinstance(payload, bytes):
				raise TypeError("`encoding` was passed, but `payload` is not of type `bytes`")
			return Value.new(payload, encoding)

	@staticmethod
	def _upgrade_(inner: _Value) -> 'Value':
		if isinstance(inner, Value):
			return inner
		return _Value.__new__(Value, inner)
	
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

IntoSample = Union[_Sample, Tuple[IntoKeyExpr, IntoValue, SampleKind], Tuple[KeyExpr, IntoValue]]
class Sample(_Sample):
	def __new__(cls, key: IntoKeyExpr, value: IntoValue, kind: SampleKind = None):
		if kind is None:
			return Sample._upgrade_(super().new(KeyExpr(key), Value(value), _SampleKind.PUT))
		else:
			return Sample._upgrade_(super().new(KeyExpr(key), Value(value), kind))
	@staticmethod
	def _upgrade_(inner: _Sample) -> 'Sample':
		return _Sample.__new__(Sample, inner)
	@property
	def key_expr(self) -> KeyExpr:
		return KeyExpr(super().key_expr)
	@property
	def value(self) -> Value:
		return Value(super().value)
	@property
	def payload(self) -> bytes:
		return super().payload
	@property
	def kind(self) -> SampleKind:
		return SampleKind(super().kind)

class Reply(_Reply):
	def __new__(cls, inner: _Reply):
		return super().__new__(cls, inner)
	@property
	def replier_id(self) -> str:
		return super().replier_id
	@property
	def ok(self) -> Sample:
		return Sample(super().ok)
	@property
	def err(self) -> Value:
		return Value(super().err)