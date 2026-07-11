## Install

When you want to build the tool from scratch, using cargo is recommended. Refer to the [official rust website](https://rust-lang.org/tools/install/) for installation advice.

Once cargo is installed, you can clone this repo and install it
```
git clone git@github.com:EricMiddelhove/forge-config-edit.git
cd forge-config-edit
cargo install --path .
```

this set of commands installs the binary to your system.

## Usage

For usage information, refer to the help page of the command

```
Usage: forge-config-edit [COMMAND] [--help] [--version]
    --help      Print help information
    --version   Print version information

Commands:
    get         Get a value/type from the forge config file
    set         Set a value in the forge config file


forge-config-edit get --file=<path> [--type] <key>
    --file=<path>    Path to the forge config file to be used
    --type           Get the type of the value instead of the value itself
    key              Key to get the value/type for. Use dot notation for nested keys (e.g., "modpack.version")

forge-config-edit set --file=<path> <key> [<value>]
    --file=<path>    Path to the forge config file to be used
    key              Key to set the value for. Use dot notation for nested keys (e.g., "modpack.version")
    value            Value to set. If omitted, the value will be removed.

```

