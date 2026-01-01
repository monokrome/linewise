# Quick Start

## Interactive Mode

The fastest way to explore data is interactive mode:

```bash
# Open a binary file with u16 length-prefixed records
lw -i data.bin

# Open with line-delimited format
lw -i data.txt -f lines
```

### Key Bindings

| Key | Action |
|-----|--------|
| `j/k` or `↓/↑` | Navigate records |
| `h/l` or `←/→` | Scroll hex view horizontally |
| `g/G` | Go to first/last record |
| `Tab` | Cycle views (Normal → Decode → Diff) |
| `/` | Search records |
| `n/N` | Next/previous search match |
| `q` | Quit |

## Command-Line Analysis

```bash
# Analyze byte patterns
lw analyze data.bin

# Find common n-grams
lw ngrams data.bin -n 4

# Show entropy per position
lw entropy data.bin

# Compare two files
lw diff file_a.bin file_b.bin
```

## Using Presets

Presets define how to parse and display specific data formats:

```bash
# List available presets
lw presets

# Use a preset for gloss transforms
lw gloss --preset my-preset data.txt
```
