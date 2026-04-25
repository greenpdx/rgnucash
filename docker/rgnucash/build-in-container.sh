#!/usr/bin/env bash
#
# Runs INSIDE the rgnucash-builder container. Don't run this on the
# host — it expects the container's filesystem layout (bind-mounts at
# /src + /lgc-deb + /out, /cache for cargo's registry + target).
#
# Inputs (env, set by the host wrapper):
#   RGNUCASH_VERSION   e.g. 0.1.7 — version baked into artifact name.
#   ARCH               e.g. arm64 — dpkg architecture.
# Inputs (bind-mounts):
#   /src       ro    rgnucash source checkout (host-side git work-tree)
#   /lgc-deb   ro    directory containing libgnucash_*.deb
#   /cache     rw    cargo registry + target (named volume or tmpfs)
#   /out       rw    where artifacts land (host-visible)
#
set -euo pipefail

: "${RGNUCASH_VERSION:=0.0.0}"
: "${ARCH:=$(dpkg --print-architecture)}"

echo "--- install libgnucash from /lgc-deb ---"
deb=$(ls /lgc-deb/libgnucash_*.deb 2>/dev/null | head -1 || true)
if [ -z "$deb" ]; then
    echo "no libgnucash deb under /lgc-deb" >&2
    exit 1
fi
# `apt -f install` repairs any broken-deps state if the deb's runtime
# dependencies aren't yet on the container's apt graph (they usually
# are — the deb's depends were computed from the same trixie image).
apt-get update -qq
dpkg -i "$deb" || apt-get -y -f install
/sbin/ldconfig

echo "--- stage source (cp from ro /src to rw /work) ---"
cp -a /src/. /work/
# Wipe any stray target dir that may have been copied; we use /cache.
rm -rf /work/target
mkdir -p /cache/target /cache/cargo /out

echo "--- cargo build --release (against /opt/libgnucash) ---"
cargo build --release

echo "--- cargo package (registry-shaped tarball) ---"
# `cargo package --no-verify` skips the smoke build of the packaged
# tarball (we already did the real release build above). `--allow-dirty`
# tolerates the bind-mounted source not being a clean checkout.
cargo package --no-verify --allow-dirty \
    || echo "!! cargo package failed; release artifacts still emitted"

echo "--- export artifacts ---"
out="/out/rgnucash-${RGNUCASH_VERSION}-${ARCH}"
rm -rf "$out"
mkdir -p "$out/release" "$out/crate"
cp -a "$CARGO_TARGET_DIR/release/." "$out/release/" 2>/dev/null || true
cp -a "$CARGO_TARGET_DIR"/package/*.crate "$out/crate/" 2>/dev/null || true
cp /work/Cargo.lock "$out/Cargo.lock"

echo "--- compress ---"
tarball="rgnucash-${RGNUCASH_VERSION}-${ARCH}.tar.zst"
( cd /out && tar --zstd -cf "$tarball" "rgnucash-${RGNUCASH_VERSION}-${ARCH}" )
rm -rf "$out"

echo "done: /out/$tarball"
ls -lh "/out/$tarball"
