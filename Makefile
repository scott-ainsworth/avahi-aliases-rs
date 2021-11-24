.DEFAULT: test

lib_source    := $(wildcard src/*.rs) $(wildcard src/*/*.rs)
alias_source  := $(wildcard src/bin/avahi-alias/*.rs)
daemon_source := $(wildcard src/bin/avahi-alias-daemon/*.rs)

########################################
# DEBUG
########################################

.PHONY: debug coverage

debug: lib bin

lib: target/debug/avahi-aliases.rlib

target/debug/avahi-aliases.rlib: $(lib_source)
	rm -f *.profraw target/debug/deps/avahi_alias*.gcd[ao]
	RUSTFLAGS="-Z instrument-coverage" \
	  LLVM_PROFILE_FILE="avahi-aliases-%m-test.profraw" \
	  cargo +nightly test --lib --tests --no-fail-fast

cov: lib
	rm -f target/debug/deps/avahi_aliases-*.gcda
	CARGO_INCREMENTAL=0 \
	  RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code \
	    -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort" \
	  RUSTDOCFLAGS="-Cpanic=abort" \
	  cargo +nightly test --lib --tests --no-fail-fast
	grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing \
	  -o ./target/debug/coverage/

bin: target/debug/avahi-alias target/debug/avahi-alias-daemon

target/debug/avahi-alias target/debug/avahi-alias-daemon: lib $(alias_source) $(daemon_source)
	cargo +nightly build --bin

clippy: debug
	cargo +nightly clippy -- -A clippy::all

########################################
# RELEASE
########################################

.PHONY: release release-test release-bin
.PHONY: test-results/release-test-results.txt target/release/avahi-alias

release-test: test-results/release-test-results.txt

release: release-test release-bin

release-bin: target/release/avahi-alias target/release/avahi-alias-daemon

test-results/release-test-results.txt: $(lib_source)
	@mkdir -p test-results
	RUSTFLAGS="-Dwarnings" cargo test --release --lib | tee $@
	RUSTFLAGS="-Dwarnings" cargo clippy --release -- -A clippy::all

target/release/avahi-alias target/release/avahi-alias-daemon: $(lib_source) $(alias_source) $(daemon_source)
	RUSTFLAGS="-Dwarnings" cargo build --release --bin $(notdir $@)
	strip $@

########################################
# DOCUMENTATION
########################################

.PHONY: doc

doc:
	cargo test --no-fail-fast --doc
	cargo doc --no-deps --document-private-items

########################################
# UTILITY
########################################

clean:
	cargo clean
	rm -fr test-results
	rm -f *.profraw *.profdata

fmt:
	cargo +nightly fmt -v --check

dofmt:
	cargo +nightly fmt -v

dump:
	@echo "lib_source    = $(lib_source)"
	@echo "alias_source  = $(alias_source)"
	@echo "daemon_source = $(daemon_source)"

setup-rust:
	@echo "Install the nightly toolchain"
	@echo "Install coverage tools"
	cargo install rustfilt
	rustup component add llvm-tools-preview
	cargo install cargo-binutils
