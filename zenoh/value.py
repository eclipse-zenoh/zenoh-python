import abc
from .enums import Encoding
from .zenoh import _Value, _Encoding

class AValue:
	@property
	@abc.abstractmethod
	def payload(self) -> bytes:
		...
	
	@property
	@abc.abstractmethod
	def encoding(self) -> Encoding:
		...
	
	def _encoding(self) -> _Encoding:
		return self.encoding.inner

class Value(AValue):
	def __init__(self, inner: _Value):
		self.inner = inner
	@property
	def payload(self) -> bytes:
		return self.inner.payload()

	@property
	def encoding(self) -> Encoding:
		return Encoding(self.inner.encoding())