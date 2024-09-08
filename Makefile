#------------------------------------------------------------------------------
# Setup
#------------------------------------------------------------------------------


#------------------------------------------------------------------------------
# Development
#------------------------------------------------------------------------------

.PHONY: check-deps
check-deps:
	cargo machete

.PHONY: lint
lint:
	cargo fmt --all
	cargo clippy --all-targets --all-features

.PHONY: fix
fix:
	cargo fix --allow-staged

.PHONY: build
build:
	cargo build

.PHONY: run
run:
	RUST_LOG=debug RUST_BACKTRACE=1 cargo run

.PHONY: clean
clean:
	cargo clean

#------------------------------------------------------------------------------
# Test
#------------------------------------------------------------------------------

.PHONY: test
test:
	cargo test


#------------------------------------------------------------------------------
# Release
#------------------------------------------------------------------------------

.PHONY: release-build
release-build:
	cargo build --release

.PHONY: install-bin
install-bin:
	sudo cp target/release/git-tui-rust /usr/local/bin/git-tui
