# Gloss Transforms

Gloss transforms decode or translate record data into human-readable formats. They can use built-in decoders or external commands.

## Built-in Transforms

### base85

Decode Base85 (ASCII85) encoded data to hex.

```toml
[gloss]
transform = "base85"
```

#### Custom Charsets

Different Base85 implementations use different character sets:

```toml
[gloss]
transform = "base85"
base85_charset = "bl4"  # Borderlands 4 custom charset
```

Available charsets:
- `ascii85` / `standard` - Standard ASCII85 (Adobe)
- `z85` / `zeromq` - ZeroMQ Z85
- `bl4` / `borderlands` - Borderlands 4 custom

### base64

Decode Base64 encoded data to hex.

```toml
[gloss]
transform = "base64"
```

### hex

Pass through hex data, removing whitespace.

```toml
[gloss]
transform = "hex"
```

### none

No transformation (pass through as-is).

```toml
[gloss]
transform = "none"
```

## External Commands

Use any command-line tool for transformation:

```toml
[gloss]
command = ["my-decoder", "--format", "json"]
```

The record is passed as an argument to the command. stdout is captured as the result.

Example with a hypothetical decoder:

```toml
[gloss]
command = ["bl4", "serial", "decode"]
```

## Caching

Results can be cached to improve performance:

```toml
[gloss]
command = ["slow-decoder"]
cache = true  # default
```

Set `cache = false` if the decoder has side effects or time-dependent output.

## CLI Usage

```bash
# Built-in transform
lw gloss -t base85 input.txt

# External command
lw gloss -c "my-decoder --json" input.txt

# From preset
lw gloss --preset my-format input.txt
```
