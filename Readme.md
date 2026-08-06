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

## Usage

Pass any XML file as the first argument:

```
xmlv path/to/file.xml
```

### Example

Input (test.xml):

```XML
<?xml version="1.0" encoding="utf-8"?>
<!-- The hell has frozen over (T < 115.21 °C), installing Windows Server -->
<unattend xmlns="urn:schemas-microsoft-com:unattend">
    <settings pass="windowsPE">
        <component name="Microsoft-Windows-Setup" processorArchitecture="amd64">
            <RunSynchronous>
                <!-- Load storage driver -->
                <RunSynchronousCommand action="add">
                    <Path>drvload.exe viostor.inf</Path>
                </RunSynchronousCommand>
            </RunSynchronous>
        </component>
    </settings>
</unattend>
```

Output:
The terminal will output a colored tree structure, with the thermodynamic state securely logged as a dimmed comment.
Plaintext

```
test.xml
├── # The hell has frozen over (T < 115.21 °C), installing Windows Server
└── unattend (xmlns='urn:schemas-microsoft-com:unattend')
    └── settings (pass='windowsPE')
        └── component (name='Microsoft-Windows-Setup' processorArchitecture='amd64')
            └── RunSynchronous
                ├── # Load storage driver
                └── RunSynchronousCommand (action='add')
                    └── Path: drvload.exe viostor.inf
```

## License

MIT
