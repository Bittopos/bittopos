CARGO ?= cargo
ARGS ?=

.PHONY: fmt fmt-check lint test build check run clean

# Apply the repository's Rust formatting.
fmt:
	$(CARGO) fmt --all

# Verify formatting without changing files.
fmt-check:
	$(CARGO) fmt --all -- --check

# Run Clippy across the workspace and fail on warnings.
lint:
	$(CARGO) clippy --workspace --all-targets -- -D warnings

# Run every test in the workspace, including the CLI crate.
test:
	$(CARGO) test --workspace

# Build every workspace member.
build:
	$(CARGO) build --workspace

# Run the complete local/CI verification suite.
check: fmt-check lint test build

# Run the terminal interface; pass arguments with ARGS="...".
run:
	$(CARGO) run -p bitto-cli -- $(ARGS)

# Remove generated Cargo artifacts.
clean:
	$(CARGO) clean
