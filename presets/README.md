# Linewise Presets

These are default presets for common data formats. To use them, copy to your config directory:

```bash
mkdir -p ~/.config/linewise/presets
cp *.toml ~/.config/linewise/presets/
```

## Available Presets

| Preset | Description |
|--------|-------------|
| `base64` | Decode Base64 encoded data |
| `hex` | Clean and format hex strings |
| `ascii85` | Decode ASCII85/Base85 data |
| `z85` | Decode Z85 (ZeroMQ) Base85 data |
| `jwt` | Decode JWT token segments |

## Usage

```bash
# List installed presets
lw presets

# Use a preset
echo "SGVsbG8gV29ybGQ=" | lw gloss --preset base64 -
```
