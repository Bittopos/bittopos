CARGO ?= cargo
ARGS ?=

.PHONY: fmt fmt-check lint test build check run clean

# Apply Rust formatting.
fmt:
	$(CARGO) fmt --all

# Check formatting without modifying files (not part of the legacy CI gate yet).
fmt-check:
	$(CARGO) fmt --all -- --check

# Run Clippy across the workspace; warnings are reported but not promoted to errors.
lint:
	$(CARGO) clippy --workspace --all-targets

# Run every test in the workspace, including the CLI crate.
test:
	$(CARGO) test --workspace --verbose

# Build every workspace member.
build:
	$(CARGO) build --workspace --verbose

# Keep the CI gate equivalent to the previously passing build-and-test checks.
check: build test

# Run the terminal interface; pass arguments with ARGS="...".
run:
	$(CARGO) run -p bitto-cli -- $(ARGS)

# Remove generated Cargo artifacts.
clean:
	$(CARGO) clean
