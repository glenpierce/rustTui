# Vaporwave Terminal

A small Rust terminal UI that displays a stylized "vaporwave" TUI, reads shell history to show top commands, and applies simple screen effects.

- Language: Rust
- UI: `ratatui` + `crossterm`
- Effects: `tachyonfx`
- Platform: macOS (development tested)

## Features

- Neon / vaporwave palette and animated fades.
- Reads shell history files (`~/.zsh_history`, `~/.bash_history`) and shows the top 20 most-used commands at startup.
- Simple input box for sending messages into the messages pane.
- Keybindings for quick interaction.

## Keybindings

- `Esc` — exit the application
- Type characters — add to input
- `Backspace` — delete last character from input
- `Enter` — submit the input as a message (adds message and triggers a short visual effect)

## How history is parsed

The history reader function returns two values: `(top_commands, debug_messages)`.

- Attempts to read `~/.zsh_history` first, then `~/.bash_history`.
- Uses lossy UTF-8 conversion so non-UTF-8 bytes don't crash parsing.
- Handles zsh-style history lines like `: 1600000000:0;git status` by taking the substring after the last `;`.
- When a command starts with `sudo`, the parser treats the next token as the command name (so `sudo apt update` counts as `apt`).
- Counts unique command names and returns the top 20 by frequency.
- Extra debug messages are collected and can be appended to the UI for troubleshooting (see source).

## Build & Run

From the project root:

- Install Rust toolchain (recommended via `rustup`).
- Build in release mode:

```bash
cargo build --release
```

Run:
```bash
cargo run --release
```

Note: running in a terminal that supports true color and proper cursor control is recommended.

## Development

- The project can be opened in RustRover (or other Rust-capable IDE).
- Use the IDE's run/debug tooling or cargo run from the terminal.
- No automated tests are included currently; add tests in `tests/`.

## Files of interest
- `src/main.rs` — main application, UI layout, event loop, and history parsing.
- `Cargo.toml` — project dependencies.

## Troubleshooting
- Ensure the `HOME` environment variable is set; the app uses it to locate history files.
- If no commands are parsed, the UI will note that no history commands were found.
- To enable extra history debugging output, uncomment the section of main that appends history_debug into the messages vector.

## Dependencies

Key crates used:
- `ratatui` — terminal UI rendering
- `crossterm` — terminal event handling
- `tachyonfx` — simple effects manager

See Cargo.toml for exact versions.