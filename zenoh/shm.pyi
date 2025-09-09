#
# Copyright (c) 2025 ZettaScale Technology
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
from typing import Self, final

@final
class AllocAlignment:
    """alignment in powers of 2: 0 == 1-byte alignment, 1 == 2byte, 2 == 4byte, 3 == 8byte etc"""

    ALIGN_1_BYTE: Self
    ALIGN_2_BYTE: Self
    ALIGN_4_BYTE: Self
    ALIGN_8_BYTE: Self

    def __new__(cls, pow: int) -> Self: ...

_AllocPolicy = JustAlloc | BlockOn | Defragment | GarbageCollect

@final
class BlockOn:
    def __new__(self, inner_policy: _AllocPolicy = JustAlloc()) -> Self: ...

@final
class Deallocate:
    """Deallocating policy.
    Forcibly deallocate up to N buffers until allocation succeeds."""

    def __new__(
        cls,
        inner_policy: _AllocPolicy = JustAlloc(),
        alt_policy: _AllocPolicy = JustAlloc(),
        deallocate_policy: _ForceDeallocPolicy = DeallocOptimal(),
    ) -> Self: ...

_ForceDeallocPolicy = DeallocEldest | DeallocOptimal | DeallocYoungest

@final
class DeallocEldest:
    """Try to dealloc eldest chunk"""

@final
class DeallocOptimal:
    """Try to dealloc optimal (currently eldest+1) chunk"""

@final
class DeallocYoungest:
    """Try to dealloc youngest chunk"""

@final
class Defragment:
    def __new__(
        self,
        inner_policy: _AllocPolicy = JustAlloc(),
        alt_policy: _AllocPolicy = JustAlloc(),
    ) -> Self: ...

@final
class GarbageCollect:
    def __new__(
        self,
        inner_policy: _AllocPolicy = JustAlloc(),
        alt_policy: _AllocPolicy = JustAlloc(),
        *,
        safe: bool = True,
    ) -> Self: ...

@final
class JustAlloc:
    """Just try to allocate"""

@final
class MemoryLayout:
    """Memory layout representation: alignment and size aligned for this alignment"""

    def __new__(cls, size: int, alignment: AllocAlignment) -> Self: ...
    @property
    def size(self) -> int: ...
    @property
    def alignment(self) -> AllocAlignment: ...

@final
class ShmProvider:
    """A generalized interface for shared memory data sources"""

    @classmethod
    def default_backend(cls, layout: _IntoMemoryLayout) -> Self:
        """Set the default backend"""

    def alloc(
        self, layout: _IntoMemoryLayout, policy: _AllocPolicy = JustAlloc()
    ) -> ZShmMut:
        """Rich interface for making allocations"""

    def defragment(self):
        """Defragment memory"""

    def garbage_collect(self) -> int:
        """Try to collect free chunks.
        Returns the size of largest collected chunk"""

    def garbage_collect_unsafe(self) -> int:
        """Try to collect free chunks.
        Returns the size of largest collected chunk"""

    @property
    def available(self) -> int:
        """Bytes available for use"""

_IntoMemoryLayout = MemoryLayout | tuple[int, AllocAlignment] | int

@final
class ZShmMut:
    """A mutable SHM buffer"""

    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    def __buffer__(self, flags: int) -> memoryview: ...
    def __release_buffer__(self, view: memoryview) -> None: ...
