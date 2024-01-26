# macOS Documents and GUI Applications

Likely for historical reasons, macOS does not pass documents sent to application bundles as `argv`. This repository is a simple demo of the lack of this feature[^1]. 

## What this repository does 

Simply, this repository:
1. creates an macOS Application Bundle called `Toy.app`[^2]
1. writes anything passed to `argv` to `/tmp/argv.txt`

Using [butonageo/cargo-bundle](https://github.com/burtonageo/cargo-bundle)

## What this repository demonstrates

macOS does not pass documents in `argv` to app bundles when invoked; moreover `cargo-bundle` does not have this functionality, either.

## What we'd like to accomplish:

1. enable [`cargo-bundle`](https://github.com/neovide/neovide/pull/2191) to automatically enable macOS bundles to receive documents using `argv`
1. petition Apple to offer a flag in `Info.plist` file to enable application bundles to receive documents, much as they do on traditional linux and other *nix systems

**Why?** today it is easier than ever to build cross-platform application bundles for Windows, Linux, and macOS. That said, macOS' current implementation places a large burden on developers. For example, there are [47 comments on this GitHub PR](https://github.com/neovide/neovide/pull/2191) for the a GUI-harness for the `neovim` text editor, which consitute dozens of developer hours spent on attempting to rectify this issue.
 
**Related** [this issue](https://github.com/burtonageo/cargo-bundle/issues/174) in the `cargo-bundle` codebase.

## Building

```bash
git clone git@github.com:abhillman/macos-argv.git
cargo build
```

[^1]: Apple instead requires applications to use a bespoke message-passing implementation; see [Wikipedia: Apple Events](https://en.wikipedia.org/wiki/Apple_event) and [Apple Developer Documentation: Apple Events](https://developer.apple.com/documentation/coreservices/apple_events) 
[^2]: Accomplished via simply adding relevant metadata to `Cargo.toml` and adding the [`cargo-bundle`](https://github.com/burtonageo/cargo-bundle) crate

