#
# Copyright (c) 2026 ZettaScale Technology
#
# This program and the accompanying materials are made available under the
# terms of the Eclipse Public License 2.0 which is available at
# http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
# which is available at https://www.apache.org/licenses/LICENSE-2.0.
#
# SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
#
import gc
import sys
from array import array

import pytest

from zenoh import ZBytes, ZBytesSegment


class RecordingLeaseSink:
    def __init__(self, *, raise_on_release=False):
        self.raise_on_release = raise_on_release
        self.released = []

    def release(self, lease_id):
        self.released.append(lease_id)
        if self.raise_on_release:
            raise RuntimeError("release failed")


class LeaseState:
    def __init__(self, sink, lease_id):
        self.sink = sink
        self.lease_id = lease_id


def make_lease(lease_id="lease-1", *, raise_on_release=False):
    sink = RecordingLeaseSink(raise_on_release=raise_on_release)
    return sink, LeaseState(sink, lease_id)


@pytest.mark.parametrize(
    "segments",
    [
        [b"hello", b"world"],
        [bytearray(b"hello"), bytearray(b"world")],
        [memoryview(b"hello"), memoryview(bytearray(b"world"))],
        [array("B", b"hello"), array("b", b"world")],
        [b"", b"hello", b"", b"world"],
        [bytes([i]) for i in range(256)],
    ],
)
def test_from_segments_copies_byte_compatible_buffers(segments):
    payload = ZBytes.from_segments(segments, copy=True)
    expected = b"helloworld" if len(segments) < 10 else bytes(range(256))

    assert bytes(payload) == expected


def test_from_segments_preserves_owned_segment_layout():
    payload = ZBytes.from_segments([b"hello", b"world"], copy=True)

    assert tuple(map(bytes, payload.segments())) == (b"hello", b"world")


def test_from_segments_constructs_zero_copy_payload_from_immutable_bytes():
    def make_payload():
        return ZBytes.from_segments([b"hello", b"world"])

    payload = make_payload()
    gc.collect()

    assert bytes(payload) == b"helloworld"
    assert tuple(map(bytes, payload.segments())) == (b"hello", b"world")


@pytest.mark.parametrize(
    ("segment", "expected"),
    [
        (memoryview(b"hello"), b"hello"),
        (memoryview(b"hello")[1:], b"ello"),
    ],
)
def test_from_segments_constructs_zero_copy_payload_from_bytes_memoryview(
    segment, expected
):
    payload = ZBytes.from_segments([segment])

    assert bytes(payload) == expected


def test_zero_copy_payload_keeps_python_bytes_owner_alive():
    owner = bytes(bytearray(b"hello"))
    initial_refcount = sys.getrefcount(owner)

    payload = ZBytes.from_segments([owner])

    assert sys.getrefcount(owner) > initial_refcount
    del payload
    gc.collect()
    assert sys.getrefcount(owner) == initial_refcount


def test_zero_copy_payload_keeps_readonly_buffer_export_alive():
    owner = bytearray(b"hello")
    payload = ZBytes.from_segments([memoryview(owner).toreadonly()])

    with pytest.raises(BufferError):
        owner.extend(b"!")

    assert bytes(payload) == b"hello"
    del payload
    gc.collect()
    owner.extend(b"!")
    assert owner == b"hello!"


def test_zero_copy_payload_lease_releases_after_payload_drop():
    sink, lease = make_lease("slot-1")
    payload = ZBytes.from_segments([b"hello"], lease=lease)

    assert sink.released == []

    del payload
    gc.collect()

    assert sink.released == ["slot-1"]


def test_zero_copy_payload_lease_is_shared_by_multiple_segments():
    sink, lease = make_lease("slot-2")
    payload = ZBytes.from_segments([b"hello", memoryview(b"world")], lease=lease)

    assert bytes(payload) == b"helloworld"
    del payload
    gc.collect()

    assert sink.released == ["slot-2"]


@pytest.mark.parametrize(
    "view_factory",
    [
        lambda payload: payload.segments(),
        lambda payload: payload.memoryviews(),
    ],
)
def test_zero_copy_payload_lease_waits_for_derived_views(view_factory):
    sink, lease = make_lease("slot-3")
    payload = ZBytes.from_segments([b"hello", b"world"], lease=lease)
    views = view_factory(payload)

    del payload
    gc.collect()
    assert sink.released == []

    assert b"".join(map(bytes, views)) == b"helloworld"
    del views
    gc.collect()

    assert sink.released == ["slot-3"]


def test_zero_copy_payload_lease_release_error_is_unraisable(monkeypatch):
    captured = []

    def hook(args):
        captured.append(args)

    monkeypatch.setattr(sys, "unraisablehook", hook)
    sink, lease = make_lease("slot-error", raise_on_release=True)
    payload = ZBytes.from_segments([b"hello"], lease=lease)

    del payload
    gc.collect()

    assert sink.released == ["slot-error"]
    assert len(captured) == 1
    assert isinstance(captured[0].exc_value, RuntimeError)
    assert captured[0].object is sink


@pytest.mark.parametrize(
    "segment",
    [
        bytearray(b"hello"),
        memoryview(bytearray(b"hello")),
        memoryview(b"hello")[::2],
        array("I", [1, 2, 3]),
    ],
)
def test_from_segments_rejects_unsupported_zero_copy_buffers(segment):
    with pytest.raises(RuntimeError, match="segment 0.*use copy=True"):
        ZBytes.from_segments([segment])


def test_from_segments_rejects_non_buffer_segment():
    with pytest.raises(TypeError, match="segment 1 does not support"):
        ZBytes.from_segments([b"hello", object()], copy=True)


def test_from_segments_rejects_non_byte_compatible_segment():
    with pytest.raises(TypeError, match="segment 0 has unsupported item format"):
        ZBytes.from_segments([array("I", [1, 2, 3])], copy=True)


def test_from_segments_rejects_non_contiguous_segment_by_default():
    segment = memoryview(bytearray(b"hello"))[::2]

    with pytest.raises(TypeError, match="segment 0 is not C-contiguous"):
        ZBytes.from_segments([segment], copy=True)


def test_from_segments_rejects_lease_with_copy():
    sink, lease = make_lease("slot-copy")

    with pytest.raises(RuntimeError, match="lease can only be used with copy=False"):
        ZBytes.from_segments([b"hello"], copy=True, lease=lease)

    assert sink.released == []


@pytest.mark.parametrize("segments", [[], [b""]])
def test_from_segments_rejects_lease_without_nonempty_raw_borrow(segments):
    sink, lease = make_lease("slot-empty")

    with pytest.raises(RuntimeError, match="lease requires at least one non-empty"):
        ZBytes.from_segments(segments, lease=lease)

    assert sink.released == []


def test_from_segments_rejects_lease_with_zbytes_segment_only():
    source = ZBytes.from_segments([b"hello"], copy=True)
    (segment,) = source.segments()
    sink, lease = make_lease("slot-segment")

    with pytest.raises(RuntimeError, match="lease requires at least one non-empty"):
        ZBytes.from_segments([segment], lease=lease)

    assert sink.released == []


def test_from_segments_rejects_invalid_lease_interface():
    with pytest.raises(TypeError, match="lease must provide a 'sink' attribute"):
        ZBytes.from_segments([b"hello"], lease=object())


def test_from_segments_can_copy_non_contiguous_segment_explicitly():
    segment = memoryview(bytearray(b"hello"))[::2]

    payload = ZBytes.from_segments(
        [segment],
        copy=True,
        require_contiguous=False,
    )

    assert bytes(payload) == b"hlo"


def test_segments_return_zero_copy_segment_views_with_independent_lifetimes():
    payload = ZBytes.from_segments([b"hello", b"world"], copy=True)
    segments = payload.segments()

    del payload
    gc.collect()

    assert isinstance(segments, tuple)
    assert all(isinstance(segment, ZBytesSegment) for segment in segments)
    assert all(memoryview(segment).readonly for segment in segments)
    assert b"".join(map(bytes, segments)) == b"helloworld"
    with pytest.raises(TypeError):
        memoryview(segments[0])[0] = 0


def test_memoryviews_are_zero_copy_views_over_segments():
    payload = ZBytes.from_segments([b"hello", b"world"], copy=True)
    views = payload.memoryviews()

    assert all(isinstance(view, memoryview) for view in views)
    assert all(view.readonly for view in views)
    assert tuple(map(bytes, views)) == tuple(map(bytes, payload.segments()))


def test_memoryviews_keep_segment_owner_alive():
    payload = ZBytes.from_segments([b"hello", b"world"], copy=True)
    views = payload.memoryviews()

    del payload
    gc.collect()

    assert tuple(map(bytes, views)) == (b"hello", b"world")


def test_copied_memoryviews_preserve_old_copy_out_behavior():
    payload = ZBytes.from_segments([b"hello", b"world"], copy=True)
    views = payload.copied_memoryviews()

    del payload
    gc.collect()

    assert all(isinstance(view, memoryview) for view in views)
    assert tuple(map(bytes, views)) == (b"hello", b"world")


def test_from_segments_copies_large_payload_without_joining_inputs():
    segment = bytes(1024 * 1024)
    payload = ZBytes.from_segments([segment, segment, segment, segment], copy=True)

    assert len(payload) == 4 * 1024 * 1024
    assert sum(map(len, payload.segments())) == len(payload)


def test_from_segments_accepts_numpy_uint8_when_available():
    numpy = pytest.importorskip("numpy")
    segments = [numpy.array([1, 2, 3], dtype=numpy.uint8)]

    assert bytes(ZBytes.from_segments(segments, copy=True)) == b"\x01\x02\x03"


def test_from_segments_accepts_readonly_numpy_uint8_zero_copy_when_available():
    numpy = pytest.importorskip("numpy")
    segment = numpy.array([1, 2, 3], dtype=numpy.uint8)
    segment.flags.writeable = False

    assert bytes(ZBytes.from_segments([segment])) == b"\x01\x02\x03"


def test_from_segments_accepts_zbytes_segment_without_copy():
    source = ZBytes.from_segments([b"hello", b"world"], copy=True)
    hello, world = source.segments()

    payload = ZBytes.from_segments([world, hello], copy=False)

    assert bytes(payload) == b"worldhello"


def test_from_segments_copies_zbytes_segment():
    source = ZBytes.from_segments([bytearray(b"hello")], copy=True)
    (segment,) = source.segments()

    payload = ZBytes.from_segments([segment], copy=True)

    assert bytes(payload) == b"hello"


def test_from_segments_accepts_shm_mut_zero_copy_when_available():
    shm = pytest.importorskip("zenoh.shm")
    provider = shm.ShmProvider.default_backend(4096)
    buf = provider.alloc(5)
    buf[:] = b"hello"

    payload = ZBytes.from_segments([buf], copy=False)

    assert bytes(payload) == b"hello"
    assert payload.as_shm() is not None
    with pytest.raises(Exception, match="consumed"):
        bytes(buf)


def test_from_segments_rejects_lease_with_shm_mut_when_available():
    shm = pytest.importorskip("zenoh.shm")
    provider = shm.ShmProvider.default_backend(4096)
    buf = provider.alloc(5)
    buf[:] = b"hello"
    sink, lease = make_lease("slot-shm-mut")

    with pytest.raises(RuntimeError, match="lease cannot be used with shared-memory"):
        ZBytes.from_segments([buf], copy=False, lease=lease)

    assert bytes(buf) == b"hello"
    assert sink.released == []


def test_from_segments_accepts_shm_zero_copy_when_available():
    shm = pytest.importorskip("zenoh.shm")
    provider = shm.ShmProvider.default_backend(4096)
    buf = provider.alloc(5)
    buf[:] = b"hello"
    original = ZBytes(buf).as_shm()

    payload = ZBytes.from_segments([original], copy=False)

    assert bytes(payload) == b"hello"
    assert payload.as_shm() is not None


def test_from_segments_rejects_lease_with_shm_when_available():
    shm = pytest.importorskip("zenoh.shm")
    provider = shm.ShmProvider.default_backend(4096)
    buf = provider.alloc(5)
    buf[:] = b"hello"
    original = ZBytes(buf).as_shm()
    sink, lease = make_lease("slot-shm")

    with pytest.raises(RuntimeError, match="lease cannot be used with shared-memory"):
        ZBytes.from_segments([original], copy=False, lease=lease)

    assert sink.released == []


def test_from_segments_accepts_mixed_shm_segments_when_available():
    shm = pytest.importorskip("zenoh.shm")
    provider = shm.ShmProvider.default_backend(4096)
    buf = provider.alloc(5)
    buf[:] = b"frame"

    payload = ZBytes.from_segments([b"h", buf, b"t"], copy=False)

    assert bytes(payload) == b"hframet"
    assert payload.as_shm() is None
    segments = payload.segments()
    assert tuple(map(bytes, segments)) == (b"h", b"frame", b"t")
    assert segments[1].as_shm() is not None


def test_from_segments_copies_shm_mut_without_consuming_when_available():
    shm = pytest.importorskip("zenoh.shm")
    provider = shm.ShmProvider.default_backend(4096)
    buf = provider.alloc(5)
    buf[:] = b"hello"

    payload = ZBytes.from_segments([buf], copy=True)

    assert bytes(payload) == b"hello"
    assert payload.as_shm() is None
    assert bytes(buf) == b"hello"


def test_from_segments_does_not_partially_consume_shm_mut_on_validation_error():
    shm = pytest.importorskip("zenoh.shm")
    provider = shm.ShmProvider.default_backend(4096)
    buf = provider.alloc(5)
    buf[:] = b"hello"

    with pytest.raises(RuntimeError, match="segment 1.*use copy=True"):
        ZBytes.from_segments([buf, bytearray(b"mutable")], copy=False)

    assert bytes(buf) == b"hello"


def test_from_segments_rejects_repeated_shm_mut_when_available():
    shm = pytest.importorskip("zenoh.shm")
    provider = shm.ShmProvider.default_backend(4096)
    buf = provider.alloc(5)
    buf[:] = b"hello"

    with pytest.raises(RuntimeError, match="repeats the same mutable SHM"):
        ZBytes.from_segments([buf, buf], copy=False)

    assert bytes(buf) == b"hello"


def test_zshm_and_shm_segment_export_readonly_memoryview_when_available():
    shm = pytest.importorskip("zenoh.shm")
    provider = shm.ShmProvider.default_backend(4096)
    buf = provider.alloc(5)
    buf[:] = b"hello"
    payload = ZBytes.from_segments([buf], copy=False)
    zshm = payload.as_shm()

    shm_view = memoryview(zshm)
    segment_view = memoryview(payload.segments()[0])

    assert shm_view.readonly
    assert segment_view.readonly
    assert bytes(shm_view) == b"hello"
    assert bytes(segment_view) == b"hello"
