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

from zenoh.config import ZenohId
from zenoh.sample import Sample
from zenoh.value import Value

class QueryTarget(Enum):
    """The kind of consolidation."""

    BEST_MATCHING = auto()
    ALL = auto()
    ALL_COMPLETE = auto()

    DEFAULT = BEST_MATCHING

class ConsolidationMode(Enum):
    """The kind of consolidation."""

    AUTO = auto()
    NONE = auto()
    MONOTONIC = auto()
    LATEST = auto()

    DEFAULT = AUTO

class Reply:
    """Structs returned by a get."""

    result: Sample | Value
    ok: Sample | None
    err: Value | None
    replier_id: ZenohId
