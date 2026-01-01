# Interactive Mode

Interactive mode provides a TUI (Terminal User Interface) for exploring records visually.

## Starting Interactive Mode

```bash
# Using -i flag
lw -i data.bin

# Using subcommand
lw interactive data.bin
```

## Interface Layout

```
┌─ Record List ─────────────────────────────────────────────────────┐
│ [1/1000] Record 0 (256 bytes)                                     │
├─ Hex View ────────────────────────────────────────────────────────┤
│ 00: 01 02 03 04 05 06 07 08  09 0A 0B 0C 0D 0E 0F 10  │................│
│ 10: 11 12 13 14 15 16 17 18  19 1A 1B 1C 1D 1E 1F 20  │............... │
├─ Info Panel ──────────────────────────────────────────────────────┤
│ Position: 0x00  Value: 0x01  Frequency: 45%                       │
└───────────────────────────────────────────────────────────────────┘
```

## View Modes

Press `Tab` to cycle through view modes:

### Normal Mode
- Standard hex view with ASCII panel
- Color-coded by byte frequency

### Decode Mode
- Shows decoded/glossed values
- Uses configured preset transforms

### Diff Mode
- Highlights bytes that differ from other records
- Useful for finding variable fields

## Key Bindings

### Navigation

| Key | Action |
|-----|--------|
| `j` / `↓` | Next record |
| `k` / `↑` | Previous record |
| `h` / `←` | Scroll left |
| `l` / `→` | Scroll right |
| `g` | Go to first record |
| `G` | Go to last record |
| `Ctrl+d` | Page down |
| `Ctrl+u` | Page up |

### Search

| Key | Action |
|-----|--------|
| `/` | Start search |
| `n` | Next match |
| `N` | Previous match |
| `Esc` | Cancel search |

### Display

| Key | Action |
|-----|--------|
| `Tab` | Cycle view mode |
| `b` | Toggle bit view |
| `+` / `-` | Adjust bytes per row |

### Other

| Key | Action |
|-----|--------|
| `q` | Quit |
| `?` | Show help |

## Color Coding

Bytes are colored based on their frequency across all records:

- **Red**: Very common (80-100%)
- **Yellow**: Common (60-79%)
- **Green**: Moderate (40-59%)
- **Cyan**: Uncommon (20-39%)
- **Blue**: Rare (0-19%)
- **Gray**: Zero/empty bytes
