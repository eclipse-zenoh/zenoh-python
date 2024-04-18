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
"""Zenoh /zeno/ is a stack that unifies data in motion, data at rest and computations. It elegantly blends traditional pub/sub with geo distributed storage, queries and computations, while retaining a level of time and space efficiency that is well beyond any of the mainstream stacks.

Before delving into the examples, we need to introduce few Zenoh concepts. First off, in Zenoh you will deal with Resources, where a resource is made up of a key and a value. The other concept you’ll have to familiarize yourself with are key expressions, such as robot/sensor/temp, robot/sensor/*, robot/**, etc. As you can gather, the above key expression denotes set of keys, while the * and ** are wildcards representing respectively (1) an arbitrary string of characters, with the exclusion of the / separator, and (2) an arbitrary sequence of characters including separators."""

from typing import Any, Callable, TypeVar, overload

from zenoh.config import Config, IntoWhatAmIMatcher
from zenoh.handlers import DefaultHandler, Handler, PythonHandler, RustHandler
from zenoh.scouting import Hello, Scout
from zenoh.session import Session

_H = TypeVar("_H")

class ZError(Exception): ...

def init_logger(): ...
def open(config: Config) -> Session:
    """Open a zenoh Session."""

@overload
def scout(
    what: IntoWhatAmIMatcher,
    config: Config,
    *,
    handler: RustHandler[Hello] | None = None,
) -> Scout[Handler[Hello]]:
    """Scout for routers and/or peers.

    scout spawns a task that periodically sends scout messages and waits for Hello replies.
    Drop the returned Scout to stop the scouting task."""

@overload
def scout(
    what: IntoWhatAmIMatcher,
    config: Config,
    *,
    handler: PythonHandler[Hello, _H],
) -> Scout[_H]: ...
@overload
def scout(
    what: IntoWhatAmIMatcher,
    config: Config,
    *,
    handler: Callable[[Hello], Any],
) -> Scout[None]: ...
