from .zenoh import init_logger, scout as _scout
from .keyexpr import *
from .config import *
from .session import *
from .enums import *
from .value import *
from .closures import *
from .queryable import *

def open(*args, **kwargs):
	return Session(*args, **kwargs)

class Scout:
	def __init__(self, inner, receiver):
		self._inner_ = inner
		self.receiver = receiver
	
	def stop(self):
		self._inner_ = None

from threading import Timer
def scout(handler: IntoHandler[Hello, Any, Any] = None, what: str = None, config: Config = None, timeout=None):
	if handler is None:
		handler = ListCollector()
	handler = Handler(handler, lambda x: Hello._upgrade_)
	scout = _scout(handler.closure, config, what)
	scout = Scout(scout, handler.receiver)
	if timeout:
		Timer(timeout, lambda: scout.stop()).start()
	return scout