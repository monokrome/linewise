# Presets

Presets are TOML configuration files that define how to parse, transform, and display specific data formats. They enable automatic detection and consistent handling of different record types.

## Preset Locations

Linewise searches for presets in these directories:

1. `~/.config/linewise/presets/` (user presets)
2. `$XDG_CONFIG_HOME/linewise/presets/` (XDG config)
3. `/etc/linewise/presets/` (system presets)
4. `/usr/share/linewise/presets/` (package presets)

## Creating a Preset

Create a `.toml` file in one of the preset directories:

```toml
[preset]
name = "my-format"
description = "My custom data format"

[records]
format = "lines"

[[detect]]
type = "starts_with"
value = "@"

[gloss]
transform = "base85"

[[color]]
match = "Error"
style = "red bold"
```

## Using Presets

```bash
# List available presets
lw presets

# Use with gloss command
lw gloss --preset my-format data.txt

# Presets auto-detect in interactive mode
lw -i data.bin
```

## Example: BL4 Items Preset

A complete example for Borderlands 4 item serials:

```toml
[preset]
name = "bl4-items"
description = "Borderlands 4 item serials (Base85 encoded)"

[records]
format = "lines"

[[detect]]
type = "starts_with"
value = "@Ug"

[[detect]]
type = "min_length"
value = 20

[gloss]
transform = "base85"
base85_charset = "bl4"

[[color]]
match = "Legendary"
style = "yellow bold"

[[color]]
match = "Epic"
style = "magenta"

[[fields]]
name = "serial"
pattern = "^(@[A-Za-z0-9+/=~!@#$%^&*]+)"

[[fields]]
name = "rarity"
from_gloss = true
pattern = "Rarity: (\\w+)"
```

See [Preset Format](./preset-format.md) for complete reference.
