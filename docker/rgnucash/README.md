# rgnucash builder

Builds the `rgnucash` Cargo crate (FFI bindings to libgnucash) inside
a throw-away podman container, against the `libgnucash` deb
produced by `../libgnucash/build.sh`. Source stays on the host as an
ordinary git work-tree; the container only sees it bind-mounted
read-only.

## Why containerize

Two reasons:

1. **Reproducibility.** The container has a fixed Debian trixie
   userspace + libclang + cargo. Whatever's on the host's
   `/opt/libgnucash/` is irrelevant — the container installs the
   versioned `libgnucash_*.deb` you point it at, builds
   against that, and then exits. Same input → same output.
2. **Host hygiene.** No `libclang-dev`, no `libglib2.0-dev`, no
   `libxml2-dev` on the build host. They live in the container image.

## Usage

```sh
# Step 1: build libgnucash first.
../libgnucash/build.sh

# Step 2: build rgnucash against it.
./build.sh
```

The artifact lands at `<rgnucash>/dist/rgnucash-<ver>-<arch>.tar.zst`. It
contains:

- `release/` — `target/release/` (`.rlib`, examples, build cache that
  another consumer can copy into its own `target/` to skip a rebuild).
- `crate/` — the `.crate` file from `cargo package` (registry-shaped,
  consumable as `cargo install --path` or via `cargo publish`).
- `Cargo.lock` — the exact dependency resolution the artifact was
  built with.

## Inputs

- `SRC` — rgnucash source checkout. Default: the rgnucash repo this
  script lives in (`../..`).
- `LGC_DEB` — explicit path to a `libgnucash_*.deb`. Default:
  newest match under `<rgnucash>/dist/`.
- `OUT` — artifact destination. Default `<rgnucash>/dist/`.
- `RGNUCASH_VERSION` — overrides the version in the artifact filename.
  Default: parsed from `$SRC/Cargo.toml`.
- `IMAGE` — container image tag. Default `rgnucash-builder:trixie`.
- `CACHE_VOLUME` — named podman volume for the cargo registry + target
  cache. Default `rgnucash-cargo-cache`. Set to empty string for a
  hermetic (slower, deterministic) build:

  ```sh
  CACHE_VOLUME='' ./build.sh
  ```

## GitHub-push workflow

The container is a build sandbox, not a source sandbox. To upstream a
change to rgnucash:

```sh
cd $SRC                          # the rgnucash repo (default: this one)
git checkout -b some-fix
$EDITOR src/...
git add -p && git commit
./docker/rgnucash/build.sh       # rebuilds artifact in container
git push origin some-fix
gh pr create
```

The container has no SSH keys, no `.gitconfig`, no GitHub credentials.
Source is bind-mounted **read-only**; the only writable surface is the
`/out` and `/cache` mounts.

## Footprint

- Container image: ~700 MB (cargo + rustc + libclang + glib/xml dev
  headers).
- Cache volume `rgnucash-cargo-cache`: ~1–2 GB after a few builds.
  Wipe with `podman volume rm rgnucash-cargo-cache`.
- Host: only the `libgnucash_*.deb` you pass in. The build
  host doesn't need libgnucash installed system-wide — the container
  installs it into its own ephemeral filesystem at run time.

Remove the build env entirely:

```sh
podman volume rm rgnucash-cargo-cache
podman image rm rgnucash-builder:trixie
```
