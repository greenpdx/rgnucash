# libgnucash builder

Builds an engine-only `libgnucash` (no GUI, no OFX, no AqBanking, no
Python) in a throw-away podman container and emits the result as a
proper Debian package: `libgnucash_<gc-ver>_<arch>.deb`.

This builder lives in the **rgnucash** repo because rgnucash is the
direct consumer of libgnucash's C API at compile time. Downstream
projects (e.g. crfactura) consume the resulting `.deb` as an apt
runtime/build dependency without needing the source toolchain.

## Why containerize

`libgnucash-dev` is not packaged in any Debian release — GnuCash
upstream doesn't promise a stable C API, so no distribution ships the
engine headers. The only way to get headers + `.so`s is to build from
source, and that pulls ~20 `-dev` packages (cmake, boost, guile, glib,
icu, libxml2, libxslt, libdbi, libsecret …). Doing the build in a
container keeps every one of those build-time deps off the host.

## Usage

```sh
# Build the .deb in a container, drop it in <rgnucash>/dist/.
./build.sh

# Install on this host:
sudo dpkg -i ../../dist/libgnucash_*.deb
```

The deb installs `/opt/libgnucash/` and registers it with `ldconfig`
via a config drop-in under `/etc/ld.so.conf.d/`. After install, both
the rgnucash builder (see `../rgnucash/`) and any downstream project
(e.g. crfactura's `make deb`) find the engine via `/opt/libgnucash/`.

## Inputs

Flags honoured via env:
- `SRC` — path to a gnucash source checkout (bind-mounted read-only).
  If unset, the script clones `github.com/Gnucash/gnucash` at
  `$GNUCASH_TAG` into a temp dir on the host.
- `GNUCASH_TAG` — git tag/branch to clone. Default `5.10`.
- `GNUCASH_VERSION` — version baked into the deb's `Version:` field.
  Default `${GNUCASH_TAG}` with a leading `v` stripped.
- `OUT` — directory the .deb lands in. Default `<rgnucash>/dist/`.
- `IMAGE` — container image tag. Default `libgnucash-builder:trixie`.

## Build flags (CMake)

- `WITH_GNUCASH=OFF` — skip the GTK desktop app (no `libgtk-3-dev`, no
  `libwebkit2gtk-4.1-dev`, no GUI transitive deps).
- `WITH_OFX=OFF` — skip OFX/QFX import (no `libofx-dev`).
- `WITH_AQBANKING=OFF` — skip online banking (no aqbanking, no gwen).
- `WITH_PYTHON=OFF` — skip Python bindings.

`ctest` runs inside the container. Tests that depend on the GUI build
fail to locate their binaries (they aren't built with
`WITH_GNUCASH=OFF`) and report "Not Run" — the script logs this and
continues to the install step. Only engine tests are a real signal.

## Coexistence with the desktop GnuCash

If the host also has the apt `gnucash` desktop package installed, the
two engines coexist: that package's engine ships in
`/usr/lib/<triplet>/`; ours ships in `/opt/libgnucash/lib/`. Different
paths, different RPATH for any rgnucash-linked binary. The on-disk
book file is the only shared state; concurrent access is mediated by
GnuCash's `<book>.LCK` sibling-file convention.

## Runtime depends in the .deb

The container computes `Depends:` automatically by walking each `.so`'s
NEEDED entries and resolving them via `dpkg-query -S`. This produces
exact package names for the build host's distro (Debian trixie). If
you build on one distro and install on another, regenerate the deb
inside a container that matches the target.

## Footprint

After build + install:
- `/opt/libgnucash/` — ~50–100 MB of `.so` + headers + pkg-config.
- Container image (on demand, can be removed): ~600 MB.
- No apt changes to the host beyond the `libgnucash` package
  itself, which `dpkg -r` removes cleanly.

Remove everything with:

```sh
sudo dpkg -r libgnucash
podman image rm libgnucash-builder:trixie
rm -f <rgnucash>/dist/libgnucash_*.deb
```

## Layout under `/opt/libgnucash`

```
/opt/libgnucash/
    include/gnucash/                     # engine headers (Account.h, …)
    lib/
        libgnc-engine.so.0               # main engine
        libgnc-*.so.0                    # backend / module libs
        gnucash/
            libgncmod-backend-dbi.so     # dlopen()ed at runtime by the engine
            libgncmod-backend-xml.so
        pkgconfig/gnucash.pc             # pkg-config metadata
    share/                               # locale catalogues, schema files
```
