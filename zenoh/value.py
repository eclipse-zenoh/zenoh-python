import abc
from .enums import Encoding

class AValue:
	@property
	@abc.abstractmethod
	def data(self) -> memoryview:
		...
	
	@property
	@abc.abstractmethod
	def encoding(self) -> Encoding:
		...
	