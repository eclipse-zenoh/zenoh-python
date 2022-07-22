from .zenoh import _Encoding, _SampleKind, _CongestionControl, _Priority

class Encoding:
	def __init__(self, inner: _Encoding):
		self.inner = inner
	@staticmethod
	def from_str(s: str) -> 'Encoding':
		return Encoding(_Encoding.from_str(s))
	def append(self, s: str):
		self.inner.append(s)
	def __eq__(self, other: 'Encoding'):
		self.inner.equals(other.inner)
	EMPTY = Encoding(_Encoding.EMPTY)
	APP_OCTET_STREAM = Encoding(_Encoding.APP_OCTET_STREAM)
	APP_CUSTOM = Encoding(_Encoding.APP_CUSTOM)
	TEXT_PLAIN = Encoding(_Encoding.TEXT_PLAIN)
	APP_PROPERTIES = Encoding(_Encoding.APP_PROPERTIES)
	APP_JSON = Encoding(_Encoding.APP_JSON)
	APP_SQL = Encoding(_Encoding.APP_SQL)
	APP_INTEGER = Encoding(_Encoding.APP_INTEGER)
	APP_FLOAT = Encoding(_Encoding.APP_FLOAT)
	APP_XML = Encoding(_Encoding.APP_XML)
	APP_XHTML_XML = Encoding(_Encoding.APP_XHTML_XML)
	APP_X_WWW_FORM_URLENCODED = Encoding(_Encoding.APP_X_WWW_FORM_URLENCODED)
	TEXT_JSON = Encoding(_Encoding.TEXT_JSON)
	TEXT_HTML = Encoding(_Encoding.TEXT_HTML)
	TEXT_XML = Encoding(_Encoding.TEXT_XML)
	TEXT_CSS = Encoding(_Encoding.TEXT_CSS)
	TEXT_CSV = Encoding(_Encoding.TEXT_CSV)
	TEXT_JAVASCRIPT = Encoding(_Encoding.TEXT_JAVASCRIPT)
	IMAGE_JPEG = Encoding(_Encoding.IMAGE_JPEG)
	IMAGE_PNG = Encoding(_Encoding.IMAGE_PNG)
	IMAGE_GIF = Encoding(_Encoding.IMAGE_GIF)

class Priority:
	REAL_TIME = _Priority.REAL_TIME
	REAL_TIME = _Priority.REAL_TIME
	INTERACTIVE_HIGH = _Priority.INTERACTIVE_HIGH
	INTERACTIVE_LOW = _Priority.INTERACTIVE_LOW
	DATA_HIGH = _Priority.DATA_HIGH
	DATA = _Priority.DATA
	DATA_LOW = _Priority.DATA_LOW
	BACKGROUND = _Priority.BACKGROUND

class SampleKind:
	PUT = _SampleKind.PUT
	DELETE = _SampleKind.DELETE

class CongestionControl:
	BLOCK = _CongestionControl.BLOCK
	DROP = _CongestionControl.DROP