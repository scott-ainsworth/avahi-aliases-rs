LIB_SOURCE    := $(wildcard src/*.rs) $(wildcard src/avahi-client/*.rs src/avahi-client/*/*.rs)
ALIAS_SOURCE  := $(wildcard src/bin/avahi-alias/*.rs)
DAEMON_SOURCE := $(wildcard src/bin/avahi-alias-daemon/*.rs)

#QUIET         := --quiet

########################################
# CONVENIENCE TARGETS
########################################

help:
	@echo "Available Targets:"
	@echo ""
	@echo "debug        Build, test, and document"
	@echo "test         Test libavahi_aliases.rlib (debug)"
	@echo "cov          Create the test coverage report (debug)"
	@echo "clippy       Run clippy (debug)"
	@echo "lib          Build libavahi_aliases.rlib (debug)"
	@echo "doc          Build the libavahi_aliases.rlib documentation"
	@echo "bin          Build avahi-alias and avahi-alias-deamon"
	@echo "release      Build release versions of everything"
	@echo "clean        Remove all build artifacts"

########################################
# DEBUG
########################################

DEBUG_ENV := \
	CARGO_INCREMENTAL=0 \
	RUSTFLAGS="\
	  -Clink-dead-code -Coverflow-checks=off -Cpanic=abort \
	  -Zprofile -Zpanic_abort_tests" \
	RUSTDOCFLAGS="-Cpanic=abort"

.PHONY: debug test cov clippy lib doc bin

debug: lib test bin cov clippy doc

lib: target/debug/libavahi_aliases.rlib

target/debug/libavahi_aliases.rlib: $(LIB_SOURCE)
	rm -f *.profraw target/debug/deps/avahi_alias*.gcd[ao]
	rm -f target/debug/deps/avahi_aliases-*.gcda
	$(DEBUG_ENV) cargo +nightly build $(QUIET) --lib

test:
	$(DEBUG_ENV) cargo +nightly test $(QUIET) --no-fail-fast

bin: target/debug/avahi-alias target/debug/avahi-alias-daemon

target/debug/avahi-alias: $(ALIAS_SOURCE) lib
	$(DEBUG_ENV) cargo +nightly build $(QUIET) --bin $(@F)

target/debug/avahi-alias-daemon: $(DAEMON_SOURCE) lib
	$(DEBUG_ENV) cargo +nightly build $(QUIET) --bin $(@F)

cov: target/debug/coverage/index.html

target/debug/coverage/index.html: test
	grcov . --source-dir . --binary-path ./target/debug/ --branch \
	  --output-type html --output-path target/debug/coverage/ \
	  --ignore 'src/bin/*.rs' \
	  --ignore 'src/avahi_dbus/*.rs' \
	  --excl-start '^#\[cfg\(test\)\]|^// coverage\(off\)' \
	  --excl-br-start '^#\[cfg\(test\)\]|^// coverage\(off\)' \
	  --excl-stop '^// coverage\(on\)' \
	  --excl-br-stop '^// coverage\(on\)' \
	  --excl-line '\#\[derive\(|// cov\(skip\)' \
	  --excl-br-line '\#\[derive\(|// cov\(skip\)'

clippy:
	$(DEBUG_ENV) cargo +nightly clippy $(QUIET) -- -A clippy::all

doc: target/doc/avahi_aliases/index.html

target/doc/avahi_aliases/index.html: lib
	rm -fr $(@D)
	$(DEBUG_ENV) cargo +nightly doc $(QUIET) --no-deps --document-private-items

########################################
# RELEASE
########################################

RELEASE_ENV := RUSTFLAGS="-Dwarnings"

.PHONY: release release-test release-clippy release-lib release-bin

release: release-test release-bin

release-lib: target/release/libavahi_aliases.rlib

release-bin: target/release/avahi-alias target/release/avahi-alias-daemon

release-test:
	$(RELEASE_ENV) cargo +stable test --release --lib --no-fail-fast
	$(RELEASE_ENV) cargo +stable test --release --doc --no-fail-fast

release-clippy:
	$(RELEASE_ENV) cargo +nightly clippy -- -A clippy::all

target/release/libavahi_aliases.rlib: $(LIB_SOURCE) release-clippy
	$(RELEASE_ENV) cargo build --release --lib

target/release/avahi-alias: $(ALIAS_SOURCE) release-lib
	$(RELEASE_ENV) cargo build --release --bin $(@F)
	strip $@

target/release/avahi-alias-daemon: $(DAEMON_SOURCE) release-lib
	$(RELEASE_ENV) cargo build --release --bin $(@F)
	strip $@

########################################
# AVAHI DBUS CLIENT GENERATED CODE
########################################

AVAHI_SRC := github/avahi/avahi-daemon
GEN_DEST := src/avahi_client/avahi_dbus
CODEGEN := ~/.cargo/bin/dbus-codegen-rust

generated-dbus-code: $(GEN_DEST)/server.rs $(GEN_DEST)/entry_group.rs

$(GEN_DEST)/server.rs: $(AVAHI_SRC)/org.freedesktop.Avahi.Server.xml $(CODEGEN)
	mkdir -p $(GEN_DEST)
	bin/generate-dbus-code < $< > $@

$(GEN_DEST)/entry_group.rs: $(AVAHI_SRC)/org.freedesktop.Avahi.EntryGroup.xml $(CODEGEN)
	mkdir -p $(GEN_DEST)
	bin/generate-dbus-code < $< > $@

$(AVAHI_SRC)/org.freedesktop.Avahi.Server.xml \
$(AVAHI_SRC)/org.freedesktop.Avahi.EntryGroup.xml:
	git clone https://github.com/lathiat/avahi.git github/avahi

$(CODEGEN):
	cargo install dbus-codegen

########################################
# UTILITY
########################################

.PHONY: clean fmt dofmt dump setup-rust

clean:
	rm -fr target test-results github
	rm -f *.profraw *.profdata

fmt:
	cargo +nightly fmt -v --check

dofmt:
	cargo +nightly fmt -v

dump:
	@echo "LIB_SOURCE    = $(LIB_SOURCE)"
	@echo "ALIAS_SOURCE  = $(ALIAS_SOURCE)"
	@echo "DAEMON_SOURCE = $(DAEMON_SOURCE)"

# `rust-setup` This is likely incomplete
setup-rust:
	@echo "Install the nightly toolchain"
	rustup toolchain install nightly \
	  --allow-downgrade \
	  --profile minimal \
	  --component clippy
	@echo "Install coverage tools"
	cargo install grcov
	cargo install rustfilt
	rustup component add llvm-tools-preview
	#cargo install cargo-binutils

# end
