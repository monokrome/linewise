# Color Rules

Color rules define how to highlight matching patterns in output. They're used in both interactive mode and CLI output (when not using `--plain`).

## Defining Color Rules

Add `[[color]]` sections to your preset:

```toml
[[color]]
match = "Error"
style = "red bold"

[[color]]
match = "Warning"
style = "yellow"

[[color]]
match = "Success"
style = "green"
```

## Match Patterns

The `match` field accepts regex patterns:

```toml
# Literal match
[[color]]
match = "Legendary"
style = "yellow bold"

# Regex pattern
[[color]]
match = "\\d{4}-\\d{2}-\\d{2}"  # Dates
style = "cyan"

# Case-insensitive (use (?i))
[[color]]
match = "(?i)error"
style = "red"
```

## Style Specification

Styles combine colors and modifiers:

### Colors
- `black`, `red`, `green`, `yellow`, `blue`, `magenta`, `cyan`, `white`
- Bright variants: `bright_red`, `bright_green`, etc.

### Modifiers
- `bold`
- `dim`
- `italic`
- `underline`

### Combining

Separate with spaces:

```toml
style = "red bold"
style = "cyan underline"
style = "bright_yellow bold italic"
```

## Disabling Colors

Use the `-p` / `--plain` flag to disable all colorized output:

```bash
lw gloss --preset my-format data.txt --plain
```

This is useful for:
- Piping output to other tools
- Environments without color support
- Logging to files

## Example Preset

```toml
[preset]
name = "log-analyzer"
description = "Color-coded log analysis"

[records]
format = "lines"

[[color]]
match = "\\[ERROR\\]"
style = "red bold"

[[color]]
match = "\\[WARN\\]"
style = "yellow"

[[color]]
match = "\\[INFO\\]"
style = "green"

[[color]]
match = "\\[DEBUG\\]"
style = "dim"

[[color]]
match = "\\d{4}-\\d{2}-\\d{2}T\\d{2}:\\d{2}:\\d{2}"
style = "cyan"
```
