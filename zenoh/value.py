import abc
from typing import Union
import json

from .enums import Encoding, SampleKind
from .zenoh import _Value, _Encoding, _Sample, _SampleKind, _Reply
from .keyexpr import KeyExpr

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
	def __new__(cls, inner: _Value):
		"""This constructor is only here for inheritance purposes, use `Value.new` instead."""
		if isinstance(inner, Value):
			return inner
		assert isinstance(inner, _Value)
		return super().__new__(cls, inner)
	
	@staticmethod
	def autoencode(value: IntoValue) -> IValue:
		if isinstance(value, IValue):
			return value
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
		this = Value(_Value.new(payload))
		if encoding is not None:
			this.encoding = encoding
		return this

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

IntoSample = Union[Sample, Tuple[KeyExpr, IntoValue, SampleKind], Tuple[KeyExpr, IntoValue]]
class Sample(_Sample):
	def new(sample: IntoSample) -> 'Sample':
		if isinstance(sample, Sample):
			return sample
		if len(sample) == 3:
			ke, value, kind = sample
			return Sample(super().new(ke, value, kind))
		ke, value = sample
		return Sample(super().new(ke, value, _SampleKind.PUT))
		
	def __new__(cls, inner: _Sample):
		return super().__new__(cls, inner)
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