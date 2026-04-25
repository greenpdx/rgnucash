#!/usr/bin/env bash
#
# Builds an engine-only libgnucash inside a throw-away podman container
# and emits libgnucash_<ver>_<arch>.deb to $OUT on the host.
# Build-time dev dependencies stay inside the container image — the
# host receives only the .deb.
#
# Usage:
#   ./docker/libgnucash/build.sh                # default: clone GNUCASH_TAG
#   SRC=/path/to/gnucash ./docker/libgnucash/build.sh
#
# Environment overrides:
#   SRC             Path to a gnucash source checkout. Bind-mounted ro
#                   into the container. If unset, this script clones
#                   github.com/Gnucash/gnucash at GNUCASH_TAG into a
#                   temp dir and uses that.
#   GNUCASH_TAG     git tag/branch to clone if SRC is unset. Default 5.10.
#   GNUCASH_VERSION Version string for the .deb. Default = GNUCASH_TAG
#                   with a leading 'v' stripped.
#   OUT             Where the .deb is written on the host.
#                   Default: <rgnucash>/dist/.
#   IMAGE           Container image tag. Default libgnucash-builder:trixie.
#
set -euo pipefail

here=$(cd "$(dirname "$0")" && pwd)
repo=$(cd "$here/../.." && pwd)

GNUCASH_TAG=${GNUCASH_TAG:-5.10}
GNUCASH_VERSION=${GNUCASH_VERSION:-${GNUCASH_TAG#v}}
SRC=${SRC:-}
OUT=${OUT:-$repo/dist}
IMAGE=${IMAGE:-libgnucash-builder:trixie}
ARCH=$(dpkg --print-architecture 2>/dev/null || uname -m)

mkdir -p "$OUT"

# If SRC is unset we clone the requested tag into a temp dir bound ro
# into the container. The clone happens on the *host* — by design the
# container has no network access at run time, so source acquisition
# always lives outside the build boundary.
cleanup_src=0
if [ -z "$SRC" ]; then
    SRC=$(mktemp -d -t gnucash-src.XXXXXX)
    cleanup_src=1
    echo "=== cloning gnucash@$GNUCASH_TAG -> $SRC (one-shot, depth=1) ==="
    git clone --depth 1 --branch "$GNUCASH_TAG" \
        https://github.com/Gnucash/gnucash.git "$SRC"
fi
cleanup() {
    if [ "$cleanup_src" -eq 1 ] && [ -d "$SRC" ]; then
        rm -rf "$SRC"
    fi
}
trap cleanup EXIT

if [ ! -f "$SRC/CMakeLists.txt" ]; then
    echo "SRC=$SRC does not look like a gnucash source checkout" >&2
    exit 1
fi

echo "=== building image $IMAGE ==="
podman build -f "$here/Containerfile" -t "$IMAGE" "$here"

echo "=== running build container (engine-only, $GNUCASH_VERSION, $ARCH) ==="
# `:Z` relabels the volume for SELinux-enforcing hosts; no-op on Debian.
# Rootless podman: UID 0 inside maps back to the invoking user, so files
# in $OUT end up owned by us without needing --userns=keep-id.
podman run --rm \
    -e GNUCASH_VERSION="$GNUCASH_VERSION" \
    -e ARCH="$ARCH" \
    -v "$SRC:/src:ro" \
    -v "$OUT:/out:Z" \
    "$IMAGE"

deb="$OUT/libgnucash_${GNUCASH_VERSION}_${ARCH}.deb"
if [ ! -f "$deb" ]; then
    echo "build did not produce $deb" >&2
    exit 1
fi

echo
echo "=== artifact ready ==="
echo "  $deb"
du -h "$deb"
echo
echo "Install on this host with:"
echo "    sudo dpkg -i \"$deb\""
echo
echo "After install, /opt/libgnucash/ holds the engine; rgnucash can be"
echo "built against it via:"
echo "    GNUCASH_LIB_PATH=/opt/libgnucash/lib \\"
echo "    GNUCASH_INCLUDE_PATH=/opt/libgnucash/include/gnucash \\"
echo "    cargo build"
