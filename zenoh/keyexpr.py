from typing import Union
from .zenoh import _KeyExpr

IntoKeyExpr = Union['KeyExpr', _KeyExpr, str]

class KeyExpr(_KeyExpr):
	"""
	Zenoh's address space is designed around keys which serve as the names of ressources.

	Keys are slash-separated lists of non-empty UTF8 strings. They may not contain the following characters: `$*#?`.
	
	Zenoh's operations are executed on key expressions, a small language that allows the definition
	of sets of keys via the use of wildcards:
	- `*` is the single-chunk wildcard, and will match any chunk: "a/*/c" will match "a/b/c", "a/hello/c", etc...
	- `**` is the 0 or more chunks wildcard: "a/**/c" matches "a/c", "a/b/c", "a/b/hello/c"...
	- `$*` is the subchunk wildcard, it will match any amount of non-/ characters: "a/b$*" matches "a/b", "a/because", "a/blue"... but not "a/c" nor "a/blue/c"
	
	To allow for better performance and gain the property that two key expressions define the same
	set if and only if they are the same string, the rules of canon form are mandatory for a key
	expression to be propagated by a Zenoh network:
	- `**/**` may not exist, as it could always be replaced by the shorter `**`,
	- `**/*` may not exist, and must be written as its equivalent `*/**` instead,
	- `$*` may not exist alone in a chunk, as it must be written `*` instead.
	`KeyExpr.autocanonize` exists to make respecting these rules easier.

	A KeyExpr is a string that has been validated to be a valid Key Expression.
	"""
	def __new__(cls, expr: IntoKeyExpr):
		"""
		The default constructor for KeyExpr will ensure that the passed expression is valid.
		It won't however try to correct expressions that aren't canon.

		You may use `KeyExpr.autocanonize(expr)` instead if you are unsure if the expression
		you will use for construction will be canon.

		Raises a zenoh.ZError exception if `expr` is not a valid key expression.
		"""
		if isinstance(expr, KeyExpr):
			return expr
		elif isinstance(expr, _KeyExpr):
			return super().__new__(cls, expr.as_str())
		else:
			return super().__new__(cls, expr)

	@staticmethod
	def autocanonize(expr: str) -> 'KeyExpr':
		"""
		This alternative constructor for key expressions will attempt to canonize the passed
		expression before checking if it is valid.

		Raises a zenoh.ZError exception if `expr` is not a valid key expression.
		"""
		if isinstance(expr, KeyExpr):
			return expr
		else:
			e = _KeyExpr.autocanonize(expr)
			return KeyExpr(e.as_str())
	
	def intersects(self, other: 'KeyExpr') -> bool:
		"""
		This method returns `True` if there exists at least one key that belongs to both sets
		defined by `self` and `other`. 
		"""
		return super().intersects(other.inner)
	
	def includes(self, other: 'KeyExpr') -> bool:
		"""
		This method returns `True` if all of the keys defined by `other` also belong to the set
		defined by `self`
		"""
		return super().includes(other.inner)
	
	def __eq__(self, other: 'KeyExpr') -> bool:
		"""
		Corresponds to set equality
		"""
		return super().equals(other.inner)
	
	def __truediv__(self, other: IntoKeyExpr) -> 'KeyExpr':
		"""
		Joins two key expressions with a `/`.

		Raises a zenoh.ZError exception if `other` is not a valid key expression.
		"""
		return KeyExpr.autocanonize(f"{self}/{other}")
	
	def __str__(self):
		return super().as_str()