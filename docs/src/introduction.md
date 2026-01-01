# Linewise

**Linewise** (`lw`) is a pattern analysis and transformation tool for record-oriented data. It helps you explore, decode, and understand binary and text data through an interactive TUI and command-line interface.

## Features

- **Interactive TUI**: Browse records with hex view, ASCII preview, and frequency analysis
- **Pattern Analysis**: Find byte patterns, n-grams, and entropy across records
- **Preset System**: Define reusable configurations for specific data formats
- **Gloss Transforms**: Decode data using built-in transforms or external commands
- **Record Formats**: Handle line-delimited, length-prefixed, and custom record boundaries

## Use Cases

- Exploring unknown binary formats
- Reverse engineering serialized data
- Analyzing save files, network captures, or game data
- Decoding custom encodings (Base85, Base64, etc.)
- Comparing record sets to find differences

## Quick Example

```bash
# Interactive mode - explore a binary file
lw -i data.bin

# Analyze byte patterns
lw analyze data.bin

# Decode Base85 encoded data
lw gloss -t base85 serials.txt

# Use a preset for specific format
lw gloss --preset bl4-items serials.txt
```
