from typing import Self, final

@final
class AllocAlignment:
    """alignment in powers of 2: 0 == 1-byte alignment, 1 == 2byte, 2 == 4byte, 3 == 8byte etc"""

    ALIGN_1_BYTE: Self
    ALIGN_2_BYTE: Self
    ALIGN_4_BYTE: Self
    ALIGN_8_BYTE: Self

    def __new__(cls, pow: int) -> Self: ...

_AllocPolicy = JustAlloc | BlockOn

@final
class BlockOn:
    def __new__(self, inner_policy: _AllocPolicy | None = JustAlloc()) -> Self: ...

@final
class Deallocate:
    """Deallocating policy.
    Forcibly deallocate up to N buffers until allocation succeeds."""

    def __new__(
        cls,
        inner_policy: _AllocPolicy | None = JustAlloc(),
        alt_policy: _AllocPolicy | None = JustAlloc(),
        deallocate_policy: _DeallocatePolicy | None = DeallocOptimal(),
    ) -> Self: ...

_DeallocatePolicy = DeallocEldest | DeallocOptimal | DeallocYoungest

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
        inner_policy: _AllocPolicy | None = JustAlloc(),
        alt_policy: _AllocPolicy | None = JustAlloc(),
    ) -> Self: ...

@final
class GarbageCollect:
    def __new__(
        self,
        inner_policy: _AllocPolicy | None = JustAlloc(),
        alt_policy: _AllocPolicy | None = JustAlloc(),
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
class ZShmMut:
    """A mutable SHM buffer"""

    def __buffer__(self, flags: int) -> memoryview: ...
    def __release_buffer__(self, view: memoryview) -> None: ...

@final
class ShmProvider:
    """A generalized interface for shared memory data sources"""

    @classmethod
    def default_backend(cls, memory: _IntoMemoryLayout) -> Self:
        """Set the default backend"""

    def alloc(self, size: int, policy=None) -> ZShmMut:
        """Rich interface for making allocations"""

    def defragment(self):
        """Defragment memory"""

    def garbage_collect(self) -> int:
        """Try to collect free chunks.
        Returns the size of largest collected chunk"""

    @property
    def available(self) -> int:
        """Bytes available for use"""

_IntoMemoryLayout = MemoryLayout | tuple[int, AllocAlignment] | int
