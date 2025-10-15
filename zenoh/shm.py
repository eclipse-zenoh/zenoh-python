from typing import Self, TypeVar, final, overload

_T = TypeVar("_T")


def _unstable(item):
    warning = ".. warning:: This API has been marked as unstable: it works as advertised, but it may be changed in a future release."
    if item.__doc__:
        item.__doc__ += "\n" + warning
    else:
        item.__doc__ = warning
    return item


@_unstable
@final
class AllocAlignment:
    """alignment in powers of 2: 0 == 1-byte alignment, 1 == 2byte, 2 == 4byte, 3 == 8byte etc"""

    ALIGN_1_BYTE: Self
    ALIGN_2_BYTE: Self
    ALIGN_4_BYTE: Self
    ALIGN_8_BYTE: Self

    def __new__(cls, pow: "int") -> "Self": ...

    def get_alignment_value(self) -> "int":
        """Get alignment in normal units (bytes)"""

    def align_size(self, size: "int") -> "int":
        """Align size according to inner alignment.
        This call may extend the size"""


@_unstable
@final
class JustAlloc:
    """Just try to allocate"""


@_unstable
@final
class BlockOn:

    def __new__(self, inner_policy: "_AllocPolicy" = JustAlloc()) -> "Self": ...


@_unstable
@final
class Deallocate:
    """Deallocating policy.
    Forcibly deallocate up to N buffers until allocation succeeds."""

    def __new__(
        cls,
        inner_policy: "_AllocPolicy" = JustAlloc(),
        alt_policy: "_AllocPolicy" = JustAlloc(),
    ) -> "Self": ...


@_unstable
@final
class Defragment:

    def __new__(
        self,
        inner_policy: "_AllocPolicy" = JustAlloc(),
        alt_policy: "_AllocPolicy" = JustAlloc(),
    ) -> "Self": ...


@_unstable
@final
class GarbageCollect:

    def __new__(
        self,
        inner_policy: "_AllocPolicy" = JustAlloc(),
        alt_policy: "_AllocPolicy" = JustAlloc(),
        *,
        safe: "bool" = True,
    ) -> "Self": ...


_AllocPolicy = JustAlloc | BlockOn | Defragment | GarbageCollect


@_unstable
@final
class MemoryLayout:
    """Memory layout representation: alignment and size aligned for this alignment"""

    def __new__(cls, size: "int", alignment: "AllocAlignment") -> "Self": ...

    @property
    def size(self) -> "int": ...

    @property
    def alignment(self) -> "AllocAlignment": ...


@_unstable
@final
class ShmProvider:
    """A generalized interface for shared memory data sources"""

    @classmethod
    def default_backend(cls, layout: "_IntoMemoryLayout") -> "Self":
        """Set the default backend"""

    def alloc(
        self, layout: "_IntoMemoryLayout", policy: "_AllocPolicy" = JustAlloc()
    ) -> "ZShmMut":
        """Rich interface for making allocations"""

    def defragment(self):
        """Defragment memory"""

    def garbage_collect(self) -> "int":
        """Try to collect free chunks.
        Returns the size of largest collected chunk"""

    def garbage_collect_unsafe(self) -> "int":
        """Try to collect free chunks.
        Returns the size of largest collected chunk"""

    @property
    def available(self) -> "int":
        """Bytes available for use"""


_IntoMemoryLayout = MemoryLayout | tuple[int, AllocAlignment] | int


@_unstable
@final
class ZShmMut:
    """A mutable SHM buffer"""

    def __bytes__(self) -> "bytes": ...

    def __str__(self) -> "str": ...

    def __setitem__(self, item: "int", value: "int"): ...
