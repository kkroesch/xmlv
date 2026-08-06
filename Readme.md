# xmlv

A minimalistic, fast CLI tool written in Rust to visualize XML files as a colored tree structure in the terminal. 

Standard CLI tools often discard XML comments or require piping through multiple utilities. `xmlv` parses the XML and renders a clean hierarchy: it strips the `< >` angle brackets, highlights tags and attributes, and gently dims comments to keep them readable without cluttering the view.

Perfect for immutable environments (like Fedora Silverblue/Sway Atomic) where you want a statically linked, fast binary with zero runtime dependencies.

## Features

* **No visual clutter:** Angle brackets are removed in favor of a clean tree layout.
* **Smart comments:** XML comments are preserved but rendered dimmed and italicized.
* **Syntax highlighting:** Tags (blue), attributes (cyan/green), and text nodes (yellow) are color-coded for quick scanning.
* **Lightweight:** Compiled as a stripped, standalone Rust binary.

## Prerequisites

* [Rust toolchain](https://rustup.rs/)

