#!/usr/bin/env bash
#
# DEPRECATED — superseded by the Debian package emitted by build.sh.
#
# Old workflow (loose-dir install):
#     ./build.sh && ./install.sh
# New workflow (proper .deb):
#     ./build.sh
#     sudo dpkg -i ../../dist/libgnucash_*.deb
#
# This script is kept as a compatibility shim — if the legacy
# dist/libgnucash/ tree exists, install it; otherwise point at the
# new flow.
#
set -euo pipefail

here=$(cd "$(dirname "$0")" && pwd)
repo=$(cd "$here/../.." && pwd)

DIST=${DIST:-$repo/dist/libgnucash}
PREFIX=${PREFIX:-/opt/libgnucash}

# New-flow check: prefer the .deb if it's there.
deb=$(ls -t "$repo"/dist/libgnucash_*.deb 2>/dev/null | head -1 || true)
if [ -n "$deb" ]; then
    cat <<EOF
This script is deprecated. A built .deb is already present:
    $deb

Install it with:
    sudo dpkg -i "$deb"
EOF
    exit 0
fi

# Legacy fallback.
if [ ! -d "$DIST" ] || [ -z "$(ls -A "$DIST" 2>/dev/null)" ]; then
    cat >&2 <<EOF
No build artifacts at $DIST and no $repo/dist/libgnucash_*.deb.

The current build flow emits a Debian package — run:
    ./build.sh
then:
    sudo dpkg -i $repo/dist/libgnucash_*.deb
EOF
    exit 1
fi

echo "Installing legacy loose-dir tree $DIST -> $PREFIX (sudo)"
sudo mkdir -p "$PREFIX"
sudo cp -a "$DIST"/. "$PREFIX"/

cat <<EOF

Done. To build rgnucash against this install:
    export GNUCASH_LIB_PATH=$PREFIX/lib
    export GNUCASH_INCLUDE_PATH=$PREFIX/include/gnucash
    cargo check -p crfactura-bridge
EOF
