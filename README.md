# linewise (lw)

Pattern analysis and transformation tool for record-oriented data.

## Installation

```bash
cargo install linewise
```

Or build from source:
```bash
git clone https://github.com/monokrome/linewise
cd linewise
cargo build --release
```

## Usage

```bash
# Interactive TUI mode
lw -i data.bin

# Analyze byte patterns
lw analyze data.bin

# Apply gloss transform (decode/translate)
lw gloss serials.txt --transform base85

# List available presets
lw presets
```

## Commands

| Command | Description |
|---------|-------------|
| `analyze` | Analyze byte patterns across records |
| `ngrams` | Find common byte sequences |
| `entropy` | Show entropy per position |
| `diff` | Compare two sets of records |
| `group` | Group records by byte value at position |
| `filter` | Filter records by byte value |
| `compare` | Compare groups side-by-side |
| `split` | Split records into files by header |
| `frequency` | Analyze position/value frequency |
| `boundaries` | Detect field boundaries |
| `interactive` | Interactive TUI for exploration |
| `gloss` | Apply transform to show decoded values |
| `presets` | List available presets |

## Presets

Presets define how to parse, transform, and display data. They're stored as TOML files in:

- `~/.config/linewise/presets/`
- `/etc/linewise/presets/`
- `/usr/share/linewise/presets/`

### Example Preset

```toml
[preset]
name = "bl4-items"
description = "Borderlands 4 item serials"

[records]
format = "lines"

[[detect]]
type = "starts_with"
value = "@Ug"

[gloss]
command = ["bl4", "serial", "decode", "--json"]

[[color]]
match = "Legendary"
style = "yellow bold"
```

### Preset Features

- **Record formats**: lines, length16, length32, custom regex
- **Detection rules**: auto-detect preset based on content
- **Gloss transforms**: base85, base64, hex, or external commands
- **Coloring**: regex-based syntax highlighting
- **Field extraction**: structured data from patterns

## Input Formats

| Format | Description |
|--------|-------------|
| `lines` | Newline-delimited text (hex encoded) |
| `length16` | Binary with u16 length prefix per record |
| `length32` | Binary with u32 length prefix per record |

## License

BSD-2-Clause
