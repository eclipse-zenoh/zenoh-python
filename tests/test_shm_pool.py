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
# Contributors:
#   ZettaScale Zenoh Team, <zenoh@zettascale.tech>
#
import gc
import time

import pytest


def require_shm():
    return pytest.importorskip("zenoh.shm")


def make_cuda_pinned_payload(data: bytes, *, unchecked: bool = False):
    torch = pytest.importorskip("torch")
    if not torch.cuda.is_available():
        pytest.skip("CUDA is unavailable")
    torch.cuda.set_device(0)
    torch.empty(1, device="cuda")
    shm = require_shm()
    try:
        pool = shm.ZShmPool(pool_size=4096, cuda_pinned=True, cuda_device=0)
    except RuntimeError as exc:
        pytest.skip(f"CUDA pinned SHM unavailable: {exc}")
    buf = pool.alloc(len(data))
    view = memoryview(buf)
    view[:] = data
    if unchecked:
        payload = pool.seal_to_zbytes_unchecked([buf])
        view.release()
        return torch, payload
    view.release()
    return torch, pool.seal_to_zbytes([buf])


def wait_for_sample(samples):
    deadline = time.monotonic() + 5
    while time.monotonic() < deadline:
        if samples:
            return samples[0]
        time.sleep(0.01)
    pytest.fail("timed out waiting for sample")


def assert_cuda_pinned_zbytes(torch, payload, expected: bytes):
    assert bytes(payload) == expected
    shm_payload = payload.as_shm()
    assert shm_payload is not None
    tensor = torch.frombuffer(memoryview(shm_payload), dtype=torch.uint8)
    assert tensor.is_pinned()


def test_zshm_pool_alloc_exposes_writable_buffer_protocol():
    shm = require_shm()
    pool = shm.ZShmPool(pool_size=4096)

    buf = pool.alloc(5)
    view = memoryview(buf)
    view[:] = b"hello"

    assert not view.readonly
    assert view.format == "B"
    assert bytes(buf) == b"hello"
    assert buf.ptr > 0
    assert len(buf) == 5
    assert buf.is_valid()
    assert not buf.is_sealed


def test_zshm_pool_seal_to_zbytes_returns_shm_payload():
    shm = require_shm()
    pool = shm.ZShmPool(pool_size=4096)
    buf = pool.alloc(5)
    memoryview(buf)[:] = b"hello"

    payload = pool.seal_to_zbytes([buf])

    assert bytes(payload) == b"hello"
    assert payload.as_shm() is not None
    assert buf.is_sealed
    with pytest.raises(Exception, match="sealed|consumed"):
        buf[0] = 0
    with pytest.raises(Exception, match="sealed|consumed"):
        memoryview(buf)


def test_zshm_pool_seal_rejects_live_export_then_succeeds_after_release():
    shm = require_shm()
    pool = shm.ZShmPool(pool_size=4096)
    buf = pool.alloc(5)
    view = memoryview(buf)
    view[:] = b"hello"

    with pytest.raises(BufferError, match="exports exist"):
        pool.seal_to_zbytes([buf])

    view.release()
    payload = pool.seal_to_zbytes([buf])

    assert bytes(payload) == b"hello"
    assert payload.as_shm() is not None


def test_zshm_pool_seal_unchecked_allows_live_export():
    shm = require_shm()
    pool = shm.ZShmPool(pool_size=4096)
    buf = pool.alloc(5)
    view = memoryview(buf)
    view[:] = b"hello"

    with pytest.raises(BufferError, match="exports exist"):
        pool.seal_to_zbytes([buf])

    payload = pool.seal_to_zbytes_unchecked([buf])

    assert bytes(payload) == b"hello"
    assert payload.as_shm() is not None
    assert buf.is_sealed
    with pytest.raises(Exception, match="sealed|consumed"):
        memoryview(buf)

    view.release()
    with pytest.raises(Exception, match="sealed|consumed"):
        buf[0] = 0


def test_zshm_pool_buf_supports_stepped_and_reverse_slice_assignment():
    shm = require_shm()
    pool = shm.ZShmPool(pool_size=4096)
    buf = pool.alloc(6)
    buf[:] = b"abcdef"

    buf[::2] = b"XYZ"
    assert bytes(buf) == b"XbYdZf"

    buf[::-1] = b"fedcba"
    assert bytes(buf) == b"abcdef"


def test_zshm_pool_seal_rejects_empty_and_non_pool_buffers():
    shm = require_shm()
    pool = shm.ZShmPool(pool_size=4096)

    with pytest.raises(ValueError, match="empty"):
        pool.seal_to_zbytes([])
    with pytest.raises(TypeError, match="pool-owned|ZShmPoolBuf"):
        pool.seal_to_zbytes([memoryview(b"hello")])


def test_zshm_pool_seal_rejects_duplicate_buffer_without_consuming():
    shm = require_shm()
    pool = shm.ZShmPool(pool_size=4096)
    buf = pool.alloc(5)
    buf[:] = b"hello"

    with pytest.raises(RuntimeError, match="duplicate|repeats"):
        pool.seal_to_zbytes([buf, buf])

    assert bytes(buf) == b"hello"
    assert not buf.is_sealed


def test_zshm_pool_seal_rejects_buffer_from_another_pool():
    shm = require_shm()
    pool_a = shm.ZShmPool(pool_size=4096)
    pool_b = shm.ZShmPool(pool_size=4096)
    buf = pool_a.alloc(5)
    buf[:] = b"hello"

    with pytest.raises(RuntimeError, match="different pool"):
        pool_b.seal_to_zbytes([buf])

    assert bytes(buf) == b"hello"
    assert not buf.is_sealed


def test_zshm_pool_instances_are_independent():
    shm = require_shm()
    small = shm.ZShmPool(pool_size=4096, cuda_pinned=False)
    large = shm.ZShmPool(pool_size=8192, cuda_pinned=False)

    left = small.alloc(4)
    right = large.alloc(5)
    left[:] = b"left"
    right[:] = b"right"

    assert bytes(small.seal_to_zbytes([left])) == b"left"
    assert bytes(large.seal_to_zbytes([right])) == b"right"


def test_zshm_pool_cpu_buffer_is_not_torch_pinned_when_available():
    torch = pytest.importorskip("torch")
    shm = require_shm()
    pool = shm.ZShmPool(pool_size=4096, cuda_pinned=False)
    buf = pool.alloc(16)

    tensor = torch.frombuffer(memoryview(buf), dtype=torch.uint8)

    assert not tensor.is_pinned()


def test_zshm_pool_cuda_pinned_buffer_is_torch_pinned_when_available():
    torch = pytest.importorskip("torch")
    if not torch.cuda.is_available():
        pytest.skip("CUDA is unavailable")
    torch.cuda.set_device(0)
    torch.empty(1, device="cuda")
    shm = require_shm()
    try:
        pool = shm.ZShmPool(pool_size=4096, cuda_pinned=True, cuda_device=0)
    except RuntimeError as exc:
        pytest.skip(f"CUDA pinned SHM unavailable: {exc}")
    buf = pool.alloc(16)

    view = memoryview(buf)
    tensor = torch.frombuffer(view, dtype=torch.uint8)

    assert tensor.is_pinned()
    del tensor
    del view
    gc.collect()

    payload = pool.seal_to_zbytes([buf])
    assert payload.as_shm() is not None
    assert torch.frombuffer(memoryview(payload.as_shm()), dtype=torch.uint8).is_pinned()


def test_zshm_pool_cuda_pinned_supports_d2h_copy_when_available():
    torch = pytest.importorskip("torch")
    if not torch.cuda.is_available():
        pytest.skip("CUDA is unavailable")
    torch.cuda.set_device(0)
    shm = require_shm()
    try:
        pool = shm.ZShmPool(pool_size=4096, cuda_pinned=True, cuda_device=0)
    except RuntimeError as exc:
        pytest.skip(f"CUDA pinned SHM unavailable: {exc}")
    buf = pool.alloc(16)
    host = torch.frombuffer(memoryview(buf), dtype=torch.uint8)
    device = torch.arange(16, dtype=torch.uint8, device="cuda")

    assert host.is_pinned()
    host.copy_(device, non_blocking=True)
    torch.cuda.synchronize()

    assert bytes(buf) == bytes(range(16))


def test_zshm_pool_cuda_registration_survives_session_put_wrapper_drop():
    zenoh = pytest.importorskip("zenoh")
    torch, payload = make_cuda_pinned_payload(b"session-payload")
    samples = []
    key_expr = "test/zshm_pool/session_put"
    session = zenoh.open(zenoh.Config())
    subscriber = session.declare_subscriber(key_expr, lambda sample: samples.append(sample))
    try:
        session.put(key_expr, payload)
        del payload
        gc.collect()

        sample = wait_for_sample(samples)
        assert_cuda_pinned_zbytes(torch, sample.payload, b"session-payload")
    finally:
        subscriber.undeclare()
        session.close()


def test_zshm_pool_cuda_registration_survives_unchecked_session_put_wrapper_drop():
    zenoh = pytest.importorskip("zenoh")
    torch, payload = make_cuda_pinned_payload(b"unchecked-session-payload", unchecked=True)
    samples = []
    key_expr = "test/zshm_pool/unchecked_session_put"
    session = zenoh.open(zenoh.Config())
    subscriber = session.declare_subscriber(key_expr, lambda sample: samples.append(sample))
    try:
        session.put(key_expr, payload)
        del payload
        gc.collect()

        sample = wait_for_sample(samples)
        assert_cuda_pinned_zbytes(torch, sample.payload, b"unchecked-session-payload")
    finally:
        subscriber.undeclare()
        session.close()


def test_zshm_pool_cuda_registration_survives_publisher_put_wrapper_drop():
    zenoh = pytest.importorskip("zenoh")
    torch, payload = make_cuda_pinned_payload(b"publisher-payload")
    samples = []
    key_expr = "test/zshm_pool/publisher_put"
    session = zenoh.open(zenoh.Config())
    subscriber = session.declare_subscriber(key_expr, lambda sample: samples.append(sample))
    publisher = session.declare_publisher(key_expr)
    try:
        publisher.put(payload)
        del payload
        gc.collect()

        sample = wait_for_sample(samples)
        assert_cuda_pinned_zbytes(torch, sample.payload, b"publisher-payload")
    finally:
        publisher.undeclare()
        subscriber.undeclare()
        session.close()


def test_zshm_pool_cuda_registration_survives_attachment_wrapper_drop():
    zenoh = pytest.importorskip("zenoh")
    torch, attachment = make_cuda_pinned_payload(b"attachment-payload")
    samples = []
    key_expr = "test/zshm_pool/attachment"
    session = zenoh.open(zenoh.Config())
    subscriber = session.declare_subscriber(key_expr, lambda sample: samples.append(sample))
    try:
        session.put(key_expr, b"body", attachment=attachment)
        del attachment
        gc.collect()

        sample = wait_for_sample(samples)
        assert sample.attachment is not None
        assert_cuda_pinned_zbytes(torch, sample.attachment, b"attachment-payload")
    finally:
        subscriber.undeclare()
        session.close()
