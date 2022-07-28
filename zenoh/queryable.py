from .zenoh import _Query, _Queryable

class Queryable:
	def __init__(self, inner: _Queryable, receiver):
		self._inner_ = inner
		self.receiver = receiver
	
	def undeclare(self):
		self._inner_ = None

class Query(_Query):
	def __new__(cls, inner: _Query):
		return super().__new__(cls, inner)