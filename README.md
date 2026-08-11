# SCHEDULER Desktop — Tauri automatic-build edition

The finished SCHEDULER app has **no command-line setup for users**.

## What users receive

- **Windows:** `SCHEDULER_..._x64-setup.exe`
- **macOS:** a universal `SCHEDULER_..._universal.dmg` for Apple Silicon and Intel Macs

A user simply opens the installer/DMG and runs SCHEDULER. They do not install Rust, Cargo, Node, npm, Electron, or Tauri.

## Automatic builds

This repository includes `.github/workflows/build-installers.yml`.

Whenever `main` is updated, GitHub automatically builds Windows and macOS versions and publishes/replaces the installers in the **Auto Build** release.

## App usage

1. Install/open SCHEDULER.
2. Click **import file**.
3. Select the `.scheduler.json` exported from the SCHEDULER Chrome extension.
4. The desktop widget opens and remembers the imported schedule.

## Windows runtime

The project uses Tauri's offline WebView2 installer mode so the Windows installer can install the required WebView runtime as part of setup rather than making the user separately configure development dependencies.

## macOS signing

The workflow can create the DMG without an Apple Developer account. For public distribution without Gatekeeper warnings, Apple code signing and notarization credentials can be added later.
