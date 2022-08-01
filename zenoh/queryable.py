from .zenoh import _Query, _Queryable
from .keyexpr import KeyExpr, Selector
from .value import Sample

class Queryable:
	def __init__(self, inner: _Queryable, receiver):
		self._inner_ = inner
		self.receiver = receiver
	
	def undeclare(self):
		self._inner_ = None

class Query(_Query):
	def __new__(cls, inner: _Query):
		return super().__new__(cls, inner)
	@property
	def key_expr(self) -> KeyExpr:
		return KeyExpr(super().key_expr)
	@property
	def value_selector(self) -> str:
		return super().value_selector
	@property
	def selector(self) -> Selector:
		return Selector._upgrade_(super().selector)
	def reply(self, sample: Sample):
		super().reply(sample)