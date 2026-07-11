# forge-config-edit

Parse, inspect, and modify Forge/FML Minecraft configuration files with full
round-trip fidelity — comments, blank lines, and formatting are preserved.

## Install

### Pre-built binaries

Download the latest staging build from the
[releases page](https://github.com/EricMiddelhove/forge-config-edit/releases/tag/staging).
Builds are available for Linux amd64, arm64, armv6, and armv7 (musl, statically linked).

### From source

Requires [Rust/Cargo](https://rust-lang.org/tools/install).

```sh
git clone git@github.com:EricMiddelhove/forge-config-edit.git
cd forge-config-edit
cargo install --path .
```

## Usage

```
forge-config-edit [OPTIONS] [FILE]
```

The input file can be passed as a positional argument, via `--input-file-path`,
or piped through stdin. All paths use **dot notation**: `section.subsection.key`.

### Options

| Flag | Description |
|------|-------------|
| `FILE` / `--input-file-path <FILE>` | Input config file. Reads from stdin if omitted. |
| `--set <PATH>=<VALUE>` | Set a value and write the modified config to stdout. May be repeated. |
| `--get=<PATH>` | Print the value at PATH. Prints the full section when PATH is a section. |
| `--type=<PATH>` | Print the type of the key at PATH (`bool`, `string`, `int`, `double`, `long`, `char`, `bool[]`, …, `subtree`). |
| `-h`, `--help` | Show help and exit. |

### Examples

```sh
# Re-export a config unchanged (round-trip check)
forge-config-edit config.cfg

# Read from stdin
cat config.cfg | forge-config-edit

# Modify a value and write the result to a new file
forge-config-edit --set backups.enable_backups=false config.cfg > out.cfg

# Modify multiple values at once
forge-config-edit --set afk.enabled=false --set backups.compression_level=9 config.cfg

# Read a single value
forge-config-edit --get=backups.enable_backups config.cfg
# → true

# Read an entire section
forge-config-edit --get=backups config.cfg

# Check the type of a key
forge-config-edit --type=backups.compression_level config.cfg
# → int
```
