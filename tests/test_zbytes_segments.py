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

from zenoh import ZBytes


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


def test_from_segments_can_copy_non_contiguous_segment_explicitly():
    segment = memoryview(bytearray(b"hello"))[::2]

    payload = ZBytes.from_segments(
        [segment],
        copy=True,
        require_contiguous=False,
    )

    assert bytes(payload) == b"hlo"


def test_segments_return_readonly_memoryviews_with_independent_lifetimes():
    payload = ZBytes.from_segments([b"hello", b"world"], copy=True)
    segments = payload.segments()

    del payload
    gc.collect()

    assert isinstance(segments, tuple)
    assert all(isinstance(segment, memoryview) for segment in segments)
    assert all(segment.readonly for segment in segments)
    assert b"".join(map(bytes, segments)) == b"helloworld"
    with pytest.raises(TypeError):
        segments[0][0] = 0


def test_memoryviews_is_an_alias_for_segments():
    payload = ZBytes.from_segments([b"hello", b"world"], copy=True)

    assert tuple(map(bytes, payload.memoryviews())) == tuple(
        map(bytes, payload.segments())
    )


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
