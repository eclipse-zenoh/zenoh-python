#
# Copyright (c) 2022 ZettaScale Technology
#
# This program and the accompanying materials are made available under the
# terms of the Eclipse Public License 2.0 which is available at
# http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
# which is available at https://www.apache.org/licenses/LICENSE-2.0.
#
# SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
#
# Contributors:
#   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
#
from typing import Union, Dict
from .zenoh import _KeyExpr, _Selector

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

    The `KeyExpr.autocanonize` constructor exists to correct eventual infrigements of the canonization rules.

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
            return _KeyExpr.__new__(cls, expr)
        else:
            return _KeyExpr.__new__(cls, _KeyExpr.new(expr))
    
    def _upgrade_(this: _KeyExpr) -> 'KeyExpr':
        return _KeyExpr.__new__(KeyExpr, expr)

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
        return super().intersects(other)
    
    def includes(self, other: 'KeyExpr') -> bool:
        """
        This method returns `True` if all of the keys defined by `other` also belong to the set
        defined by `self`.
        """
        return super().includes(other)
    
    def undeclare(self, session: 'Session'):
        """
        Undeclares a key expression previously declared on the session.
        """
        super().undeclare(session)
    
    def __eq__(self, other: 'KeyExpr') -> bool:
        """
        Corresponds to set equality.
        """
        return super().__eq__(other)
    
    def __truediv__(self, other: IntoKeyExpr) -> 'KeyExpr':
        """
        Joins two key expressions with a `/`.

        Raises a zenoh.ZError exception if `other` is not a valid key expression.
        """
        return KeyExpr.autocanonize(f"{self}/{other}")
    
    def __str__(self):
        return super().__str__()
    
    def __hash__(self):
        return super().__hash__()

IntoSelector = Union['Selector', _Selector, IntoKeyExpr]
class Selector(_Selector):
    """
    A selector is the combination of a [Key Expression](crate::prelude::KeyExpr), which defines the
    set of keys that are relevant to an operation, and a `parameters`, a set of key-value pairs
    with a few uses:
    * specifying arguments to a queryable, allowing the passing of Remote Procedure Call parameters
    * filtering by value,
    * filtering by metadata, such as the timestamp of a value,

    When in string form, selectors look a lot like a URI, with similar semantics:
    * the `key_expr` before the first `?` must be a valid key expression.
    * the `parameters` after the first `?` should be encoded like the query section of a URL:
      * key-value pairs are separated by `&`,
      * the key and value are separated by the first `=`,
      * in the absence of `=`, the value is considered to be the empty string,
      * both key and value should use percent-encoding to escape characters,
      * defining a value for the same key twice is considered undefined behavior.

    Zenoh intends to standardize the usage of a set of keys. To avoid conflicting with RPC parameters,
    the Zenoh team has settled on reserving the set of keys that start with non-alphanumeric characters.

    This document will summarize the standardized keys for which Zenoh provides helpers to facilitate
    coherent behavior for some operations.

    Queryable implementers are encouraged to prefer these standardized keys when implementing their
    associated features, and to prefix their own keys to avoid having conflicting keys with other
    queryables.

    Here are the currently standardized keys for Zenoh:
    * `_time`: used to express interest in only values dated within a certain time range, values for
      this key must be readable by the [Zenoh Time DSL](zenoh_util::time_range::TimeRange) for the value to be considered valid.
    * `_filter`: *TBD* Zenoh intends to provide helper tools to allow the value associated with
      this key to be treated as a predicate that the value should fulfill before being returned.
      A DSL will be designed by the Zenoh team to express these predicates.
    """
    def __new__(cls, selector: IntoSelector):
        if isinstance(selector, Selector):
            return selector
        if isinstance(selector, _Selector):
            return Selector._upgrade_(selector)
        return Selector._upgrade_(super().new(str(selector)))
    @staticmethod
    def _upgrade_(this: _Selector) -> 'Selector':
        return _Selector.__new__(Selector, this)
    @property
    def key_expr(self) -> KeyExpr:
        "The key expression part of the selector."
        return KeyExpr(super().key_expr)
    @property
    def parameters(self):
        "The value selector part of the selector."
        return super().parameters
    @parameters.setter
    def set_parameters(self, parameters: str):
        super().parameters = parameters
    def decode_parameters(self) -> Dict[str, str]:
        """
        Decodes the value selector part of the selector.

        Raises a ZError if some keys were duplicated: duplicated keys are considered undefined behaviour,
        but we encourage you to refuse to process incoming messages with duplicated keys, as they might be
        attempting to use HTTP Parameter Pollution like exploits.
        """
        return super().decode_parameters()
    def __str__(self):
        return super().__str__()