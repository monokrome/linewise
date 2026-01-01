# Record Formats

Linewise supports multiple ways to split input data into individual records.

## Built-in Formats

### lines

Newline-delimited text records. Each line is a separate record.

```toml
[records]
format = "lines"
```

Use with text files, logs, or line-separated data.

### length16

Binary format with u16 (2-byte) little-endian length prefix before each record.

```toml
[records]
format = "length16"
```

Common in many binary protocols and save file formats.

### length32

Binary format with u32 (4-byte) little-endian length prefix.

```toml
[records]
format = "length32"
```

For formats with records larger than 65KB.

### custom

Custom record boundaries using regex patterns.

```toml
[records]
format = "custom"
pattern = "\\x00\\x00"  # Split on double-null
```

Use when records have specific delimiters or markers.

## CLI Format Specification

Most commands accept a `-f` or `--format` flag:

```bash
# Line-delimited text
lw analyze -f lines data.txt

# Length-prefixed binary (default)
lw analyze -f length16 data.bin
lw analyze data.bin  # same as above

# Interactive mode
lw -i data.bin -f length16
```

## Detecting Format

To figure out what format a file uses:

1. Check if it's text (UTF-8 readable) → likely `lines`
2. Look at first bytes:
   - If small value (< 1000) followed by that many bytes → `length16`
   - If larger value with same pattern → `length32`
3. Look for repeating delimiters → `custom`

Use `lw analyze` to see byte patterns that can help identify the format.
