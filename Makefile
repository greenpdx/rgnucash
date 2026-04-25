# rgnucash — convenience build targets.
#
# Two parallel build paths:
#
#   container (default)    Hermetic — uses podman + the recipes under
#                          docker/. Build-time dependencies stay inside
#                          the container image; the host receives only
#                          the artifact in dist/.
#
#   native                 Direct — runs cmake/ninja/cargo on the host.
#                          Requires the build-time -dev packages to be
#                          apt-installed on this host. Faster after the
#                          one-shot apt install, but pollutes the host
#                          with ~600 MB of build deps that linger.
#
# Build host: arm64 (this machine) by default. The container path is
# trivially repeatable on x86_64 with the same commands. The native
# path is one host = one arch.

.PHONY: all help \
        libgnucash libgnucash-native libgnucash-deps \
        rgnucash rgnucash-native rgnucash-deps \
        clean dist-clean

all: libgnucash rgnucash

help:
	@awk 'BEGIN {FS=":.*##"; printf "\nTargets:\n"} /^[a-zA-Z_-]+:.*?##/ {printf "  \033[36m%-22s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

# ---- libgnucash --------------------------------------------------------

libgnucash: ## Build libgnucash_*.deb in podman (default)
	./docker/libgnucash/build.sh

libgnucash-deps: ## apt-install build deps for native libgnucash
	@if [ -f /etc/debian_version ]; then \
		echo "==> installing libgnucash build-deps via apt"; \
		sudo apt-get install -y --no-install-recommends \
			build-essential cmake ninja-build pkg-config swig gettext \
			libglib2.0-dev libxml2-dev libxslt1-dev xsltproc libicu-dev zlib1g-dev \
			libboost-dev libboost-date-time-dev libboost-filesystem-dev \
			libboost-locale-dev libboost-program-options-dev libboost-regex-dev \
			guile-3.0-dev libdbi-dev libdbd-sqlite3 libdbd-pgsql \
			libsecret-1-dev libgtest-dev libgmock-dev \
			dpkg-dev file binutils \
			git ca-certificates; \
	else \
		echo "==> not a Debian host; install build deps via your package manager" >&2; \
		exit 1; \
	fi

# Native libgnucash: clones gnucash@$(GNUCASH_TAG) into dist/build/, runs
# cmake/ninja/install on the host (no container, no apt step here —
# `make libgnucash-deps` handles that). Writes to /opt/libgnucash via sudo.
GNUCASH_TAG ?= 5.10
libgnucash-native: ## Build libgnucash natively → /opt/libgnucash/
	@command -v cmake >/dev/null || { echo "==> cmake missing. Run 'make libgnucash-deps' first." >&2; exit 1; }
	@command -v ninja >/dev/null || { echo "==> ninja missing. Run 'make libgnucash-deps' first." >&2; exit 1; }
	@mkdir -p dist/build
	@if [ ! -d dist/build/gnucash ]; then \
		echo "==> cloning gnucash@$(GNUCASH_TAG)"; \
		git clone --depth 1 --branch $(GNUCASH_TAG) https://github.com/Gnucash/gnucash.git dist/build/gnucash; \
	fi
	@mkdir -p dist/build/gnucash/_build
	@cd dist/build/gnucash/_build && cmake -G Ninja .. \
		-DCMAKE_INSTALL_PREFIX=/opt/libgnucash \
		-DCMAKE_BUILD_TYPE=Release \
		-DWITH_GNUCASH=OFF -DWITH_OFX=OFF \
		-DWITH_AQBANKING=OFF -DWITH_PYTHON=OFF
	@cd dist/build/gnucash/_build && ninja
	@cd dist/build/gnucash/_build && sudo ninja install
	@sudo ldconfig
	@echo
	@echo "==> /opt/libgnucash/ populated. To verify:"
	@echo "    ls /opt/libgnucash/include/gnucash/ | head"

# ---- rgnucash ----------------------------------------------------------

rgnucash: ## Build rgnucash artifact tarball in podman (default)
	./docker/rgnucash/build.sh

rgnucash-deps: ## apt-install build deps for native rgnucash
	@if [ -f /etc/debian_version ]; then \
		echo "==> installing rgnucash build-deps via apt"; \
		sudo apt-get install -y --no-install-recommends \
			cargo rustc \
			build-essential pkg-config \
			libclang-dev clang \
			libglib2.0-dev libxml2-dev; \
	else \
		echo "==> not a Debian host; install build deps via your package manager" >&2; \
		exit 1; \
	fi

# Native rgnucash: assumes /opt/libgnucash/ exists (from libgnucash-native
# or `dpkg -i dist/libgnucash_*.deb`). Cargo writes to ./target as usual.
rgnucash-native: ## Build rgnucash natively against /opt/libgnucash/
	@[ -d /opt/libgnucash/include/gnucash ] || { \
		echo "==> /opt/libgnucash/ missing. Run 'make libgnucash-native' first," >&2; \
		echo "    or install dist/libgnucash_*.deb via 'sudo dpkg -i'." >&2; \
		exit 1; \
	}
	@command -v cargo >/dev/null || { echo "==> cargo missing. Run 'make rgnucash-deps' first." >&2; exit 1; }
	GNUCASH_LIB_PATH=/opt/libgnucash/lib \
	GNUCASH_INCLUDE_PATH=/opt/libgnucash/include/gnucash \
	cargo build --release

# ---- cleanup ----------------------------------------------------------

clean: ## Remove cargo build artifacts
	cargo clean

# `clean` deliberately preserves dist/. The deb + tarball there are
# expensive to rebuild; nuke explicitly with dist-clean.
dist-clean: clean ## Also remove dist/ (deb, tarball, gnucash source clone)
	rm -rf dist/
