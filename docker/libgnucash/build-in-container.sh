#!/usr/bin/env bash
#
# Runs INSIDE the libgnucash-builder container. Don't run this on the
# host — it expects the container's filesystem layout (bind-mounts at
# /src + /out, deps installed via apt, etc.).
#
# Inputs (env, set by the host wrapper):
#   GNUCASH_VERSION   e.g. 5.10  — used as the deb's Version: field.
#   ARCH              e.g. arm64 — dpkg architecture for the deb name.
# Inputs (bind-mounts):
#   /src              ro  gnucash source checkout
#   /out              rw  where the .deb lands (host-visible)
#
set -euo pipefail

: "${GNUCASH_VERSION:=0.0.0}"
: "${ARCH:=$(dpkg --print-architecture)}"

bundle=/usr/local/share/libgnucash

echo "--- stage source (cp from ro /src to rw /work) ---"
cp -a /src/. /work/
mkdir -p /work/_build
cd /work/_build

echo "--- configure ---"
cmake -G Ninja /work \
    -DCMAKE_INSTALL_PREFIX=/opt/libgnucash \
    -DCMAKE_BUILD_TYPE=Release \
    -DWITH_GNUCASH=OFF \
    -DWITH_OFX=OFF \
    -DWITH_AQBANKING=OFF \
    -DWITH_PYTHON=OFF

echo "--- build ---"
ninja -j"$(nproc)"

echo "--- test (engine-only) ---"
# Many tests in the suite depend on binaries that are only built with
# WITH_GNUCASH=ON (GUI). They report "Not Run". A non-zero ctest exit
# is not fatal here — install still happens.
ctest --output-on-failure -j"$(nproc)" \
    || echo "!! ctest exited non-zero (expected with WITH_GNUCASH=OFF). Continuing."

echo "--- install ---"
ninja install

echo "--- assemble .deb ---"
pkg=/work/_pkg
rm -rf "$pkg"
mkdir -p "$pkg/opt/libgnucash" "$pkg/DEBIAN" "$pkg/etc/ld.so.conf.d"
cp -a /opt/libgnucash/. "$pkg/opt/libgnucash/"
cp "$bundle/ld.so.conf" "$pkg/etc/ld.so.conf.d/libgnucash.conf"
install -m 0755 "$bundle/postinst" "$pkg/DEBIAN/postinst"
install -m 0644 "$bundle/triggers" "$pkg/DEBIAN/triggers"

echo "--- compute runtime depends from .so NEEDED entries ---"
# Walk every .so in our package, list its NEEDED libraries, drop any
# we ship ourselves, then resolve each remaining soname to the dpkg
# package that owns the matching file on the build system.
needed=$(find "$pkg/opt/libgnucash" -type f \( -name '*.so' -o -name '*.so.*' \) \
    -exec sh -c 'objdump -p "$1" 2>/dev/null | awk "/NEEDED/ {print \$2}"' _ {} \; \
    | sort -u)

deps=
for soname in $needed; do
    case "$soname" in
        libgnc*|libgncmod*) continue ;;  # shipped inside this package
    esac
    # Find the file on disk that ldconfig would resolve this soname to.
    libpath=$(/sbin/ldconfig -p | awk -v s="$soname" '$1 == s {print $NF; exit}')
    [ -n "$libpath" ] || continue
    # Resolve symlinks so dpkg-query -S finds the real owning file.
    realpath_lib=$(readlink -f "$libpath" 2>/dev/null || echo "$libpath")
    pkg_owner=$(dpkg-query -S "$realpath_lib" 2>/dev/null | head -1 | cut -d: -f1)
    [ -n "$pkg_owner" ] && deps="${deps:+$deps, }$pkg_owner"
done

# Dedupe (split on ", ", uniq, rejoin).
if [ -n "$deps" ]; then
    deps=$(printf '%s\n' "$deps" | tr ',' '\n' | sed 's/^ //' | sort -u | paste -sd, - | sed 's/,/, /g')
else
    deps=libc6
fi
echo "    runtime deps: $deps"

sed -e "s/@VERSION@/$GNUCASH_VERSION/" \
    -e "s/@ARCH@/$ARCH/" \
    -e "s|@DEPENDS@|$deps|" \
    "$bundle/control.in" > "$pkg/DEBIAN/control"

echo "--- control file ---"
cat "$pkg/DEBIAN/control"

echo "--- dpkg-deb --build ---"
deb_name="libgnucash_${GNUCASH_VERSION}_${ARCH}.deb"
dpkg-deb --root-owner-group --build "$pkg" "/out/$deb_name"

if command -v lintian >/dev/null; then
    echo "--- lintian (best-effort) ---"
    lintian "/out/$deb_name" || true
fi

echo "done: /out/$deb_name"
ls -lh "/out/$deb_name"
