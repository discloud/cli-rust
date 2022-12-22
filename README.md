# Discloud CLI
[![Build & Upload](https://github.com/discloud/cli-rust/actions/workflows/release.yml/badge.svg)](https://github.com/discloud/cli-rust/actions/workflows/release.yml) [![Build and Check (w/ Clippy)](https://github.com/discloud/cli-rust/actions/workflows/check.yml/badge.svg)](https://github.com/discloud/cli-rust/actions/workflows/check.yml) [![discloud cli crate badge](https://shields.io/crates/v/discloud)](https://crates.io/crates/discloud) ![license badge](https://img.shields.io/github/license/discloud/cli-rust) ![issues](https://img.shields.io/github/issues/discloud/cli-rust) ![Pull Requests](https://img.shields.io/github/issues-pr/discloud/cli-rust?color=blue)

Blazingly fast Discloud CLI to host your apps.

![CLI Demo](https://user-images.githubusercontent.com/92828847/189148462-b0cf3a1a-fb76-4bb4-9d9b-02dc225d5a6c.gif)

## How to download?

Checkout the [latest release](https://github.com/discloud/cli-rust/releases/latest) page to download the latest version of the CLI for windows, mac and linux (x86); 
Or use `cargo install discloud` to compile from scratch if there's not a pre-compiled binary for your platform (e.g. arm, BSD).

## Using the installer

### Windows: 
1. Run this on powershell:
```ps1
. {iwr -useb "https://discloud.github.io/cli-rust/installer/windows.ps1"} | iex;
```
2. Restart your terminals

### Linux:
1. Run `curl -L https://discloud.github.io/cli-rust/installer/linux | bash`
2. Restart all your terminals or run `exec bash` to make the `discloud` command to be available on the current terminal
### MacOS

__*Coming soon...*__