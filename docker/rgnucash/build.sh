#!/usr/bin/env bash
#
# Builds rgnucash inside a throw-away podman container, against the
# libgnucash deb produced by ../libgnucash/build.sh. Emits
#   dist/rgnucash-<ver>-<arch>.tar.zst
# containing target/release/ + Cargo.lock + the .crate file from
# `cargo package`.
#
# Source stays on the host as an ordinary git work-tree — the container
# only sees it bind-mounted read-only. To push a change upstream:
#   cd $SRC && git commit && git push   # done from the host
#
# Usage:
#   ./docker/rgnucash/build.sh
#
# Environment overrides:
#   SRC              rgnucash source checkout. Default: the rgnucash
#                    repo this script lives in (../..).
#   LGC_DEB          Path to a libgnucash_*.deb. Default:
#                    auto-pick the newest one in <rgnucash>/dist/.
#   OUT              Where artifacts land. Default <rgnucash>/dist/.
#   RGNUCASH_VERSION Version string baked into the artifact name.
#                    Default: parsed from $SRC/Cargo.toml.
#   IMAGE            Container image tag. Default rgnucash-builder:trixie.
#   CACHE_VOLUME     Named podman volume for cargo cache. Default
#                    rgnucash-cargo-cache. Set to '' to disable cache.
#
set -euo pipefail

here=$(cd "$(dirname "$0")" && pwd)
repo=$(cd "$here/../.." && pwd)

SRC=${SRC:-$repo}
OUT=${OUT:-$repo/dist}
IMAGE=${IMAGE:-rgnucash-builder:trixie}
CACHE_VOLUME=${CACHE_VOLUME:-rgnucash-cargo-cache}
ARCH=$(dpkg --print-architecture 2>/dev/null || uname -m)

if [ ! -f "$SRC/Cargo.toml" ]; then
    echo "SRC=$SRC does not look like an rgnucash checkout (no Cargo.toml)" >&2
    exit 1
fi

# Pull version from Cargo.toml if not set.
if [ -z "${RGNUCASH_VERSION:-}" ]; then
    RGNUCASH_VERSION=$(awk -F'"' '/^version = "/ {print $2; exit}' "$SRC/Cargo.toml")
    [ -n "$RGNUCASH_VERSION" ] || { echo "could not parse version from $SRC/Cargo.toml" >&2; exit 1; }
fi

# Find the libgnucash deb to install inside the container.
if [ -z "${LGC_DEB:-}" ]; then
    LGC_DEB=$(ls -t "$repo"/dist/libgnucash_*.deb 2>/dev/null | head -1 || true)
fi
if [ -z "$LGC_DEB" ] || [ ! -f "$LGC_DEB" ]; then
    cat >&2 <<EOF
No libgnucash_*.deb found.

  Run ./docker/libgnucash/build.sh first, or pass:
      LGC_DEB=/path/to/libgnucash_*.deb $0
EOF
    exit 1
fi
lgc_dir=$(dirname "$LGC_DEB")

mkdir -p "$OUT"

echo "=== building image $IMAGE ==="
podman build -f "$here/Containerfile" -t "$IMAGE" "$here"

# Cache volume: long-lived named volume keeps cargo's registry + target
# warm across runs. Worth ~2-5x speedup on incremental builds. Set
# CACHE_VOLUME='' for a hermetic build (slower but deterministic).
cache_args=()
if [ -n "$CACHE_VOLUME" ]; then
    podman volume inspect "$CACHE_VOLUME" >/dev/null 2>&1 || \
        podman volume create "$CACHE_VOLUME" >/dev/null
    cache_args+=(-v "$CACHE_VOLUME:/cache:Z")
else
    # Anonymous tmpfs volume — discarded when the container exits.
    cache_args+=(--tmpfs /cache:rw,size=4g)
fi

echo "=== running rgnucash build container ($RGNUCASH_VERSION, $ARCH) ==="
echo "  src:    $SRC (ro)"
echo "  lgc:    $LGC_DEB"
echo "  out:    $OUT"
[ -n "$CACHE_VOLUME" ] && echo "  cache:  $CACHE_VOLUME (named volume)"

podman run --rm \
    -e RGNUCASH_VERSION="$RGNUCASH_VERSION" \
    -e ARCH="$ARCH" \
    -v "$SRC:/src:ro" \
    -v "$lgc_dir:/lgc-deb:ro" \
    "${cache_args[@]}" \
    -v "$OUT:/out:Z" \
    "$IMAGE"

artifact="$OUT/rgnucash-${RGNUCASH_VERSION}-${ARCH}.tar.zst"
if [ ! -f "$artifact" ]; then
    echo "build did not produce $artifact" >&2
    exit 1
fi

echo
echo "=== artifact ready ==="
echo "  $artifact"
du -h "$artifact"
echo
echo "Inspect contents:"
echo "    tar --zstd -tf \"$artifact\" | head"
