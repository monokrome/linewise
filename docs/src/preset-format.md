# Preset Format Reference

Complete reference for the TOML preset file format.

## File Structure

```toml
[preset]
# Preset metadata

[records]
# Record format configuration

[[detect]]
# Auto-detection rules (multiple allowed)

[gloss]
# Transform configuration

[[color]]
# Color rules (multiple allowed)

[[fields]]
# Field extraction patterns (multiple allowed)
```

## [preset] Section

```toml
[preset]
name = "my-preset"           # Required: unique identifier
description = "Description"  # Optional: human-readable description
```

## [records] Section

Defines how input is split into records.

### format = "lines"

Newline-delimited text records.

```toml
[records]
format = "lines"
```

### format = "length16"

Binary with u16 little-endian length prefix.

```toml
[records]
format = "length16"
```

### format = "length32"

Binary with u32 little-endian length prefix.

```toml
[records]
format = "length32"
```

### format = "custom"

Custom delimiter pattern.

```toml
[records]
format = "custom"
pattern = "\\x00\\x00"  # Regex pattern
```

## [[detect]] Sections

Rules for auto-detecting which preset to use. Multiple rules can be specified; all must match.

### starts_with

```toml
[[detect]]
type = "starts_with"
value = "@Ug"
```

### ends_with

```toml
[[detect]]
type = "ends_with"
value = "=="
```

### contains

```toml
[[detect]]
type = "contains"
value = "magic"
```

### regex

```toml
[[detect]]
type = "regex"
pattern = "^[A-Z]{3}\\d{4}"
```

### min_length / max_length

```toml
[[detect]]
type = "min_length"
value = 20

[[detect]]
type = "max_length"
value = 1000
```

### byte_equals

Check specific byte position.

```toml
[[detect]]
type = "byte_equals"
position = 0
value = 33  # 0x21
```

## [gloss] Section

Transform configuration for decoding/translating records.

### Built-in Transform

```toml
[gloss]
transform = "base85"  # base85, base64, hex, none
cache = true          # Cache results (default: true)
```

### Base85 with Custom Charset

```toml
[gloss]
transform = "base85"
base85_charset = "bl4"  # ascii85, z85, bl4
```

### External Command

```toml
[gloss]
command = ["decoder", "--json"]
cache = true
```

## [[color]] Sections

Patterns for colorized output.

```toml
[[color]]
match = "Error"        # Regex pattern
style = "red bold"     # Color and modifiers
```

### Available Styles

Colors: `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`
Bright: `bright_red`, `bright_green`, etc.
Modifiers: `bold`, `dim`, `italic`, `underline`

Combine with spaces: `"yellow bold"`, `"red underline"`

## [[fields]] Sections

Extract structured fields from records.

```toml
[[fields]]
name = "serial"           # Field name
pattern = "^(@\\w+)"      # Regex with capture group
from_gloss = false        # Extract from raw (false) or glossed (true)
```

### Example: Multiple Fields

```toml
[[fields]]
name = "id"
pattern = "ID:(\\d+)"

[[fields]]
name = "type"
pattern = "Type:(\\w+)"
from_gloss = true

[[fields]]
name = "value"
pattern = "Value:(\\d+\\.\\d+)"
from_gloss = true
```

## Complete Example

```toml
[preset]
name = "game-items"
description = "Game item data format"

[records]
format = "lines"

[[detect]]
type = "starts_with"
value = "@"

[[detect]]
type = "min_length"
value = 15

[gloss]
transform = "base85"
base85_charset = "bl4"

[[color]]
match = "Legendary"
style = "yellow bold"

[[color]]
match = "Rare"
style = "blue"

[[color]]
match = "Error"
style = "red"

[[fields]]
name = "serial"
pattern = "^(@[A-Za-z0-9!#$%&()*+,-./:;<=>?@^_`{|}~]+)"

[[fields]]
name = "rarity"
from_gloss = true
pattern = "Rarity: (\\w+)"

[[fields]]
name = "level"
from_gloss = true
pattern = "Level: (\\d+)"
```
