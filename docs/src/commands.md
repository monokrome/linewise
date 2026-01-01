# Commands

Linewise provides several subcommands for different analysis tasks.

## analyze

Analyze byte patterns across records to understand data structure.

```bash
lw analyze data.bin [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `-f, --format` | Input format: `lines`, `length16` (default) |
| `-n, --max-positions` | Maximum byte positions to analyze (default: 64) |
| `--bits` | Show bit-level analysis |

### Output

Shows frequency distribution for each byte position, highlighting:
- Fixed values (same across all records)
- Varying values with their frequencies
- Total records analyzed

## ngrams

Find common byte sequences (n-grams) in records.

```bash
lw ngrams data.bin [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `-n, --size` | N-gram size in bytes (default: 4) |
| `-m, --min-count` | Minimum occurrences to report (default: 10) |

## entropy

Calculate Shannon entropy per byte position to identify structured vs random data.

```bash
lw entropy data.bin [OPTIONS]
```

Low entropy positions typically contain structured data (IDs, types), while high entropy positions contain variable data (hashes, random values).

## diff

Compare two sets of records to find positions that differ.

```bash
lw diff file_a.bin file_b.bin
```

Useful for understanding what changed between two versions of data.

## gloss

Apply transforms to decode or translate records.

```bash
lw gloss INPUT [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--preset NAME` | Use a named preset's gloss config |
| `-t, --transform` | Built-in transform: `base85`, `base64`, `hex` |
| `-c, --command` | External command for transformation |

### Examples

```bash
# Decode Base85 data
lw gloss serials.txt -t base85

# Use external command
lw gloss data.txt -c "my-decoder --format json"

# Use preset
lw gloss serials.txt --preset bl4-items
```

## presets

List all available presets.

```bash
lw presets
```

Shows presets from:
- `~/.config/linewise/presets/`
- `/etc/linewise/presets/`
- `/usr/share/linewise/presets/`

## split

Split records into groups by header bytes.

```bash
lw split data.bin -n 4 -o groups/
```

Groups records by their first N bytes and writes each group to a separate file.

## interactive

Open interactive TUI mode (same as `-i` flag).

```bash
lw interactive data.bin
```
