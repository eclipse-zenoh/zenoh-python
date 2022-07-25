import abc
from typing import Generic, Callable, Union
from multiprocessing import Queue

class IClosure(Generic[In, Out]):
	@property
	@abc.abstractmethod
	def call(self) -> Callable[[In], Out]:
		...
	@property
	@abc.abstractmethod
	def drop(self) -> Callable[[], None]:
		...

class IHandler(Generic[In, Out, Receiver]):
	@abc.abstractmethod
	def closure(self) -> IClosure[In, Out]:
		...
	@abc.abstractmethod
	def receiver(self) -> Receiver:
		...

IntoClosure = Union[IHandler, IClosure, Queue, Tuple[Callable, Callable], Callable]

class Closure(IClosure, Generic[In, Out]):
	def __init__(self, closure: IClosure):
		self._call_ = None
		self._drop_ = lambda: None
		if isinstance(closure, IHandler):
			closure = closure.closure()
		if isinstance(closure, IClosure):
			self._call_ = closure.call()
			self._drop_ = closure.drop()
			return
		if isinstance(closure, Queue):
			self._call_ = lambda x: closure.put(x)
			return
		if type(closure) is tuple:
			self._call_, self._drop_ = closure
		if callable(closure):
			self._call_ = closure
		raise TypeError("Unexpected type as input for zenoh.Closure")

	def call(self) -> Callable[In, Out]:
		return self._call_

	def drop(self) -> Callable[[], None]:
		return self._drop_