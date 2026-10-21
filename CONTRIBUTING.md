# Contributing

Bittopos desktop changes should be made on feature branches and submitted as pull requests. Do not commit directly to main.

## Repository boundaries

- Desktop window, native integration and packaging: this repository.
- Shared web UI: bittopos-web (included as a Git submodule).
- Server API and persistence: bittopos-server.
- Terminal interface: bittopos-cli.
- Shared domain logic: bittopos-core.

When updating the core dependency, pin an immutable commit SHA and include the reason for the update in the PR.

## Checks

    git submodule update --init --recursive
    npm install
    npm run check
    npm run build
    cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check
    cargo check --manifest-path src-tauri/Cargo.toml
    cargo test --manifest-path src-tauri/Cargo.toml
