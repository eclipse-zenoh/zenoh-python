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
from collections.abc import Iterable
from typing import TYPE_CHECKING, Self, TypeVar, final, overload

if TYPE_CHECKING:
    from . import ZBytes

_T = TypeVar("_T")

def _unstable(item: _T) -> _T:
    """marker for unstable functionality"""

@_unstable
@final
class AllocAlignment:
    """alignment in powers of 2: 0 == 1-byte alignment, 1 == 2byte, 2 == 4byte, 3 == 8byte etc"""

    ALIGN_1_BYTE: Self
    ALIGN_2_BYTE: Self
    ALIGN_4_BYTE: Self
    ALIGN_8_BYTE: Self

    def __new__(cls, pow: int) -> Self: ...
    def get_alignment_value(self) -> int:
        """Get alignment in normal units (bytes)"""

    def align_size(self, size: int) -> int:
        """Align size according to inner alignment.
        This call may extend the size"""

@_unstable
@final
class JustAlloc:
    """Just try to allocate"""

@_unstable
@final
class BlockOn:
    def __new__(self, inner_policy: _AllocPolicy = JustAlloc()) -> Self: ...

@_unstable
@final
class Deallocate:
    """Deallocating policy.
    Forcibly deallocate up to N buffers until allocation succeeds."""

    def __new__(
        cls,
        inner_policy: _AllocPolicy = JustAlloc(),
        alt_policy: _AllocPolicy = JustAlloc(),
    ) -> Self: ...

@_unstable
@final
class Defragment:
    def __new__(
        self,
        inner_policy: _AllocPolicy = JustAlloc(),
        alt_policy: _AllocPolicy = JustAlloc(),
    ) -> Self: ...

@_unstable
@final
class GarbageCollect:
    def __new__(
        self,
        inner_policy: _AllocPolicy = JustAlloc(),
        alt_policy: _AllocPolicy = JustAlloc(),
        *,
        safe: bool = True,
    ) -> Self: ...

_AllocPolicy = JustAlloc | BlockOn | Defragment | GarbageCollect

@_unstable
@final
class MemoryLayout:
    """Memory layout representation: alignment and size aligned for this alignment"""

    def __new__(cls, size: int, alignment: AllocAlignment) -> Self: ...
    @property
    def size(self) -> int: ...
    @property
    def alignment(self) -> AllocAlignment: ...

@_unstable
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

@_unstable
@final
class ZShmPool:
    """Explicit shared-memory pool for creating writable SHM payload buffers.

    When ``cuda_pinned`` is true, allocations are additionally registered with
    the CUDA Driver API so CUDA-aware writers can treat the host memory as
    pinned. Overlapping allocations share page registrations within the pool.
    CUDA libraries are not required when ``cuda_pinned`` is false.
    """

    def __new__(
        cls,
        pool_size: int = 268435456,
        *,
        cuda_pinned: bool = False,
        cuda_device: int = 0,
        alignment: AllocAlignment | None = None,
    ) -> Self: ...
    def alloc(self, size: int, alignment: AllocAlignment | None = None) -> ZShmPoolBuf:
        """Allocate a pool-owned mutable SHM buffer."""

    def seal_to_zbytes(self, buffers: Iterable[ZShmPoolBuf]) -> "ZBytes":
        """Consume pool-owned buffers and return a true SHM-backed ZBytes.

        Fails if any buffer still has active Python buffer exports.
        """

    def seal_to_zbytes_unchecked(self, buffers: Iterable[ZShmPoolBuf]) -> "ZBytes":
        """Consume pool-owned buffers even when active buffer exports exist.

        Danger: the caller must guarantee all CPU/GPU writers have completed
        and no existing memoryview, torch tensor, capnp view, or other alias
        will write after this call. Violating that guarantee can race with
        Zenoh reads/sends and produce torn payload contents. Keep the returned
        ZBytes alive until any pre-existing aliases are released.
        """

    @property
    def cuda_pinned(self) -> bool: ...

@_unstable
@final
class ZShmPoolBuf:
    """A mutable buffer allocated by :class:`ZShmPool`.

    It implements the writable Python buffer protocol until sealed.
    """

    @property
    def ptr(self) -> int: ...
    @property
    def is_sealed(self) -> bool: ...
    def is_valid(self) -> bool: ...
    def __len__(self) -> int: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    @overload
    def __setitem__(self, item: int, value: int): ...
    @overload
    def __setitem__(self, item: slice, value: bytes | bytearray): ...

@_unstable
@final
class ZShm:
    """An immutable SHM buffer.

    Implements the Python buffer protocol for read-only memoryviews.
    """

    def is_valid(self) -> bool: ...
    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...

@_unstable
@final
class ZShmMut:
    """A mutable SHM buffer"""

    def __bytes__(self) -> bytes: ...
    def __str__(self) -> str: ...
    @overload
    def __setitem__(self, item: int, value: int): ...
    @overload
    def __setitem__(self, item: slice, value: bytes | bytearray): ...
