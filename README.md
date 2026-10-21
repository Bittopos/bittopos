# Bittopos Desktop

Bittopos is the cross-platform desktop GUI for Windows, Linux and macOS. This repository owns the desktop shell; the hosted SPA is maintained in [bittopos-web](https://github.com/jasperalsain/bittopos-web) and included here as a Git submodule. The shared Rust domain logic lives in [bittopos-core](https://github.com/jasperalsain/bittopos-core).

## Current status

This is the initial Tauri 2 desktop shell, not yet a production desktop release. It reuses the web UI and expects a Bittopos API at http://127.0.0.1:3000/api/v1. Starting and supervising a local server, secure account/session integration, platform icons, and signed installers remain follow-up work. The shell is not yet a standalone offline app.

## Clone and run

Clone the repository with submodules:

    git clone --recurse-submodules https://github.com/jasperalsain/bittopos.git
    cd bittopos
    npm install
    npm run tauri dev

If the repository was cloned without submodules, run git submodule update --init --recursive.

Install the Tauri platform prerequisites for your OS first. Linux builds require the WebKitGTK and GTK development packages documented in the Tauri prerequisites guide: https://v2.tauri.app/start/prerequisites/.

## Build

    npm run check
    npm run build
    npm run tauri build

Packaging is temporarily disabled in src-tauri/tauri.conf.json until the application icon set and cross-platform release workflow are added.

## Repository boundaries

- bittopos: Tauri desktop GUI and platform packaging.
- bittopos-web: hosted SPA and shared web UI.
- bittopos-server: HTTP API and persistence.
- bittopos-cli: bitto terminal interface.
- bittopos-core: canonical shared Rust crate.

The frontend is included as a submodule rather than copied, so changes to the shared UI are reviewed in bittopos-web. The desktop shell pins bittopos-core to an immutable Git commit; update that revision through a reviewed PR.

## Licensing

The desktop shell and web UI use MPL-2.0. The separately distributed bittopos-core crate uses Apache-2.0.
