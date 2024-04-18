#
# Copyright (c) 2024 ZettaScale Technology
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
from enum import Enum, auto
from typing import Self

class SetIntersectionLevel(Enum):
    DISJOINT = auto()
    INTERSECTS = auto()
    INCLUDES = auto()
    EQUALS = auto()

class KeyExpr:
    """A possibly-owned version of keyexpr that may carry optimisations for use with a Session that may have declared it.
    Check keyexpr's documentation for detailed explainations of the Key Expression Language
    """

    def __new__(cls, key_expr: IntoKeyExpr) -> Self: ...
    def autocanonize(self, key_expr: str) -> Self:
        """Canonizes the passed value before returning it as a KeyExpr.
        Will return Err if the passed value isn't a valid key expression despite canonization.
        """

    def intersects(self, key_expr: IntoKeyExpr) -> bool:
        """Returns true if the keyexprs intersect, i.e. there exists at least one key which is contained in both of the sets defined by self and other."""

    def includes(self, key_expr: IntoKeyExpr) -> bool:
        """Returns true if self includes other, i.e. the set defined by self contains every key belonging to the set defined by other."""

    def relation_to(self, key_expr: IntoKeyExpr) -> SetIntersectionLevel:
        """Returns the relation between self and other from self's point of view (SetIntersectionLevel::Includes signifies that self includes other).
        Note that this is slower than keyexpr::intersects and keyexpr::includes, so you should favor these methods for most applications.
        """

    def join(self, key_expr: IntoKeyExpr) -> KeyExpr:
        """Joins both sides, inserting a / in between them.
        This should be your prefered method when concatenating path segments."""

    def concat(self, key_expr: IntoKeyExpr) -> KeyExpr:
        """Performs string concatenation and returns the result as a KeyExpr if possible.
        You should probably prefer KeyExpr::join as Zenoh may then take advantage of the hierachical separation it ins  erts.
        """

IntoKeyExpr = KeyExpr | str
