# CLI Reference

Complete command-line interface reference for `lw`.

## Global Options

| Option | Description |
|--------|-------------|
| `-i, --interactive <FILE>` | Open file in interactive mode |
| `-f, --format <FORMAT>` | Input format for -i mode (default: length16) |
| `-p, --plain` | Disable colorized output |
| `-V, --version` | Print version |
| `-h, --help` | Print help |

## Commands

### analyze

Analyze byte patterns across records.

```
lw analyze [OPTIONS] <INPUT>
```

| Option | Description |
|--------|-------------|
| `-f, --format <FORMAT>` | Input format (default: length16) |
| `-n, --max-positions <N>` | Max positions to analyze (default: 64) |
| `--bits` | Show bit-level analysis |

### ngrams

Find common byte sequences.

```
lw ngrams [OPTIONS] <INPUT>
```

| Option | Description |
|--------|-------------|
| `-f, --format <FORMAT>` | Input format (default: length16) |
| `-n, --size <N>` | N-gram size in bytes (default: 4) |
| `-m, --min-count <N>` | Minimum occurrences (default: 10) |

### entropy

Show entropy per byte position.

```
lw entropy [OPTIONS] <INPUT>
```

| Option | Description |
|--------|-------------|
| `-f, --format <FORMAT>` | Input format (default: length16) |
| `-n, --max-positions <N>` | Max positions (default: 64) |

### diff

Compare two record sets.

```
lw diff [OPTIONS] <FILE_A> <FILE_B>
```

| Option | Description |
|--------|-------------|
| `-f, --format <FORMAT>` | Input format (default: length16) |

### gloss

Apply transforms to decode records.

```
lw gloss [OPTIONS] <INPUT>
```

| Option | Description |
|--------|-------------|
| `--preset <NAME>` | Use named preset's gloss config |
| `-t, --transform <TRANSFORM>` | Built-in: base85, base64, hex |
| `-c, --command <CMD>` | External command |

Use `-` as INPUT to read from stdin.

### split

Split records by header bytes.

```
lw split [OPTIONS] <INPUT>
```

| Option | Description |
|--------|-------------|
| `-f, --format <FORMAT>` | Input format (default: length16) |
| `-n, --header-length <N>` | Header bytes to group by (default: 4) |
| `-o, --output <DIR>` | Output directory (default: ./groups) |

### boundaries

Detect record boundaries in binary data.

```
lw boundaries [OPTIONS] <INPUT>
```

| Option | Description |
|--------|-------------|
| `-f, --format <FORMAT>` | Input format (default: length16) |
| `-n, --max-positions <N>` | Max positions (default: 32) |

### presets

List available presets.

```
lw presets
```

### interactive

Open interactive TUI mode.

```
lw interactive [OPTIONS] <INPUT>
```

| Option | Description |
|--------|-------------|
| `-f, --format <FORMAT>` | Input format (default: length16) |

## Examples

```bash
# Quick interactive exploration
lw -i data.bin

# Analyze patterns
lw analyze data.bin -n 128 --bits

# Decode Base85 data
echo "@UgABC123..." | lw gloss -t base85 -

# Use preset
lw gloss --preset bl4-items serials.txt

# Split by 4-byte header
lw split data.bin -n 4 -o ./by-header/
```
