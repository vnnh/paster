# paster

A Rust utility that allows you to "paste" clipboard contents into fields that do not allow pasting by simulating inputs.

## Hotkeys

| Hotkey         | Behavior                                                                 |
|----------------|--------------------------------------------------------------------------|
| Ctrl+B         | Direct paste (types clipboard contents as-is)                            |
| Ctrl+L         | Paste, ignore new lines, and manually insert 2 Enter key presses         |
| Ctrl+A+L       | Prefix every line with `/all ` before typing                             |

## Development

### Dependencies

- [arboard](https://crates.io/crates/arboard) – Clipboard access
- [enigo](https://crates.io/crates/enigo) – Keyboard simulation
- [device_query](https://crates.io/crates/device_query) – Global key detection
- [rand](https://crates.io/crates/rand) – Randomized delays
