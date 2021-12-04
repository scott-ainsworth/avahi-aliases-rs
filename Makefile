LIB_SOURCE    := $(wildcard src/*.rs) $(wildcard src/avahi-client/*.rs src/avahi-client/*/*.rs)
ALIAS_SOURCE  := $(wildcard src/bin/avahi-alias/*.rs)
DAEMON_SOURCE := $(wildcard src/bin/avahi-alias-daemon/*.rs)
DEBUG_BIN     := target/debug
TEST_DATA     := test-data
RELEASE_BIN   := target/release

#QUIET         := --quiet

########################################
# CONVENIENCE TARGETS
########################################

help:
	@echo "Available Targets:"
	@echo ""
	@echo "debug        Build, test, and document (debug)"
	@echo "  bin        Build avahi-alias and avahi-alias-deamon (debug)"
	@echo "  cov        Create the test coverage report (debug)"
	@echo "  clippy     Run clippy (debug)"
	@echo "  doc        Build the libavahi_aliases.rlib documentation (debug)"
	@echo "  lib        Build libavahi_aliases.rlib (debug)"
	@echo "  test       Test avahi-alias and avahi-alias-daemon (debug)"
	@echo "  unit-test  Test libavahi_aliases.rlib (debug)"
	@echo ""
	@echo "release      Build release versions of everything"
	@echo ""
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

.PHONY: debug lib unit-test bin test cov clippy doc

debug: lib unit-test bin test cov clippy doc

lib: $(DEBUG_BIN)/libavahi_aliases.rlib

$(DEBUG_BIN)/libavahi_aliases.rlib: $(LIB_SOURCE)
	rm -f *.profraw $(DEBUG_BIN)/deps/avahi_alias*.gcd[ao]
	rm -f $(DEBUG_BIN)/deps/avahi_aliases-*.gcda
	$(DEBUG_ENV) cargo +nightly build $(QUIET) --lib

unit-test:
	$(DEBUG_ENV) cargo +nightly test $(QUIET) --no-fail-fast

bin: $(DEBUG_BIN)/avahi-alias $(DEBUG_BIN)/avahi-alias-daemon

$(DEBUG_BIN)/avahi-alias: $(ALIAS_SOURCE) lib
	$(DEBUG_ENV) cargo +nightly build $(QUIET) --bin $(@F)

$(DEBUG_BIN)/avahi-alias-daemon: $(DAEMON_SOURCE) lib
	$(DEBUG_ENV) cargo +nightly build $(QUIET) --bin $(@F)

test: lib bin unit-test
	bin/test-avahi-alias.sh debug

cov: $(DEBUG_BIN)/coverage/index.html

$(DEBUG_BIN)/coverage/index.html: unit-test
	grcov . --source-dir . --binary-path ./$(DEBUG_BIN)/ --branch \
	  --output-type html --output-path $(DEBUG_BIN)/coverage/ \
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

.PHONY: release release-lib release-bin release-unit-test release-test release-clippy

release: release-lib release-bin release-clippy release-unit-test release-test

release-lib: $(RELEASE_BIN)/libavahi_aliases.rlib

$(RELEASE_BIN)/libavahi_aliases.rlib: $(LIB_SOURCE)
	$(RELEASE_ENV) cargo +stable build --release --lib

release-bin: $(RELEASE_BIN)/avahi-alias $(RELEASE_BIN)/avahi-alias-daemon

$(RELEASE_BIN)/avahi-alias: $(ALIAS_SOURCE) release-lib
	$(RELEASE_ENV) cargo +stable build --release --bin $(@F)
	strip $@

$(RELEASE_BIN)/avahi-alias-daemon: $(DAEMON_SOURCE) release-lib
	$(RELEASE_ENV) cargo +stable build --release --bin $(@F)
	strip $@

release-clippy:
	$(RELEASE_ENV) cargo +stable clippy -- -A clippy::all

release-unit-test:
	$(RELEASE_ENV) cargo +stable test --release --lib --no-fail-fast

release-test:
	bin/test-avahi-alias.sh release

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
