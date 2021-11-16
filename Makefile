.DEFAULT: test

lib_source    := $(wildcard src/*.rs) $(wildcard src/*/*.rs)
alias_source  := $(wildcard src/bin/avahi-alias/*.rs)
daemon_source := $(wildcard src/bin/avahi-alias-daemon/*.rs)

########################################
# DEBUG
########################################

.PHONY: debug debug-test debug-bin
.PHONY: test-results/debug-test-results.txt target/debug/avahi-alias

test: test-results/debug-test-results.txt

debug: test bin

bin: target/debug/avahi-alias target/debug/avahi-alias-daemon

test-results/debug-test-results.txt: $(lib_source)
	rm -f $@
	@mkdir -p test-results
	cargo test --no-fail-fast --lib | tee $@
	cargo clippy -- -A clippy::all

target/debug/avahi-alias target/debug/avahi-alias-daemon: $(lib_source) $(alias_source) $(daemon_source)
	cargo build --bin $(notdir $@)

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

fmt:
	cargo +nightly fmt -v --check

dofmt:
	cargo +nightly fmt -v

dump:
	@echo "lib_source    = $(lib_source)"
	@echo "alias_source  = $(alias_source)"
	@echo "daemon_source = $(daemon_source)"
