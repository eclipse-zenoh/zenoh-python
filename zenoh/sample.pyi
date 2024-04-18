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
from typing import Self, Type, TypeVar

from zenoh.key_expr import KeyExpr
from zenoh.prelude import Encoding
from zenoh.publication import CongestionControl, Priority
from zenoh.time import Timestamp

_T = TypeVar("_T")

class QoS:
    priority: Priority
    congestion_control: CongestionControl
    express: bool

    def __new__(
        cls,
        priority: Priority | None = None,
        congestion_control: CongestionControl | None = None,
        express: bool | None = None,
    ) -> Self: ...

class SampleKind(Enum):
    PUT = auto()
    DELETE = auto()

class Sample:
    key_expr: KeyExpr
    payload: bytes
    kind: SampleKind
    encoding: Encoding
    timestamp: Timestamp
    qos: QoS

    def payload_as(self, tp: Type[_T]) -> _T: ...
