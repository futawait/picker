# picker

[![Crates.io](https://img.shields.io/crates/v/picker.svg)](https://crates.io/crates/picker)
[![License](https://img.shields.io/github/license/futawait/picker.svg)](https://github.com/futawait/picker/blob/master/LICENSE)

A cross-platform picker for access files and folders

## Installation

Add `picker` as a dependency in your `Cargo.toml`:

```toml
[dependencies]
picker = "0.1.1"
```

## Example

Pick a single file:

```rust
use picker::FileOpenPicker;

fn main() {
    if let Ok(option) = FileOpenPicker::new()
        .file_type_filter(&["zip", "dmg"])
        .file_type_filter(&["png", "jpg"])
        .pick_single_file()
    {
        if let Some(path) = option {
            println!("Path is {}.", path);
        }
    }
}
```

## Supported Platform

- [x] macOS
- [ ] Windows
- [ ] Linux
