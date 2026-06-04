#!/usr/bin/env bash
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
# Usage: wheel-to-deb.sh <wheel_file> <package_name> <version> <debian_arch>
# Example:
#   wheel-to-deb.sh eclipse_zenoh-1.0.0-cp39-abi3-manylinux_2_17_x86_64.whl \
#     python3-eclipse-zenoh 1.0.0 amd64

set -euo pipefail

WHEEL=$1
PKG=$2
VER=$3
ARCH=$4

WORKDIR=$(mktemp -d)
trap 'rm -rf "$WORKDIR"' EXIT

unzip -q "$WHEEL" -d "$WORKDIR/contents"

DIST_PKG="$WORKDIR/deb/usr/lib/python3/dist-packages"
mkdir -p "$DIST_PKG"

cp -r "$WORKDIR/contents/zenoh" "$DIST_PKG/"

mkdir -p "$WORKDIR/deb/DEBIAN"
cat > "$WORKDIR/deb/DEBIAN/control" <<CTRL
Package: $PKG
Version: $VER
Architecture: $ARCH
Maintainer: ZettaScale Zenoh Team <zenoh@zettascale.tech>
Depends: python3 (>= 3.9), libc6
Section: python
Priority: optional
Homepage: https://zenoh.io
Description: Eclipse Zenoh Python bindings
 Eclipse Zenoh: Zero Overhead Pub/sub, Store/Query and Compute.
 .
 This package provides the Python bindings for Eclipse Zenoh, enabling
 pub/sub, queryable and geo-distributed storage in Python.
 .
 Built from manylinux wheels; requires Ubuntu 22.04+, Debian 12+,
 or any distribution with glibc >= 2.17.
CTRL

dpkg-deb --build --root-owner-group "$WORKDIR/deb" "${PKG}_${VER}_${ARCH}.deb"
echo "Built: ${PKG}_${VER}_${ARCH}.deb"
