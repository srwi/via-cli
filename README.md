> [!WARNING]
> `via-cli` is in early development and partly untested. Use at your own risk!

# via-cli

A command-line interface (CLI) for the [VIA API](https://github.com/srwi/qmk-via-api), designed for use with keyboards running the [QMK](https://github.com/qmk/qmk_firmware) (Quantum Mechanical Keyboard) firmware.

## Usage

```
Usage: via-cli [OPTIONS] --vid <VID> --pid <PID> <COMMAND>

Commands:
  get-protocol-version    Get the protocol version
  get-layer-count         Get the layer count
  get-key                 Get a key from a specific layer, row, and column
  set-key                 Set a key at a specific layer, row, and column
  read-raw-matrix         Read the raw matrix for a specific layer
  write-raw-matrix        Write the raw matrix
  get-keyboard-value      Get a keyboard value
  set-keyboard-value      Set a keyboard value
  get-encoder-value       Get an encoder value
  set-encoder-value       Set an encoder value
  get-custom-menu-value   Get a custom menu value
  set-custom-menu-value   Set a custom menu value
  get-per-key-rgb-matrix  Get the per-key RGB matrix
  set-per-key-rgb-matrix  Set a per-key RGB matrix value
  get-rgb-mode            Get the current RGB mode
  get-brightness          Get the current brightness
  get-color               Get a color
  set-color               Set a color
  get-custom-color        Get a custom color
  set-custom-color        Set a custom color
  set-rgb-mode            Set the RGB mode
  commit-custom-menu      Commit custom menu changes
  save-lighting           Save lighting configuration
  reset-eeprom            Reset EEPROM
  jump-to-bootloader      Jump to bootloader
  get-macro-count         Get the macro count
  get-macro-buffer-size   Get the macro buffer size
  get-macro-bytes         Get the macro bytes
  set-macro-bytes         Set the macro bytes
  reset-macros            Reset macros
  help                    Print this message or the help of the given subcommand(s)

Options:
      --vid <VID>                Vendor ID
      --pid <PID>                Product ID
      --usage-page <USAGE_PAGE>  Usage page [default: 65376]
  -h, --help                     Print help
  -V, --version                  Print version
```

## Examples

Get decimal keycode of row 2, column 7 on layer 0:

```sh
❯ via-cli get-key -h
Get a key from a specific layer, row, and column

Usage: via-cli --vid <VID> --pid <PID> get-key <LAYER> <ROW> <COLUMN>

❯ via-cli --vid 0x594D --pid 0x604D get-key 0 2 7
13
```

Write keymap for a keyboard with 4 rows and 12 columns:

```sh
❯ via-cli write-raw-matrix -h
Write the raw matrix

Usage: via-cli --vid <VID> --pid <PID> write-raw-matrix <ROWS> <COLS> [KEYMAP]...

❯ via-cli --vid 0x594D --pid 0x604D write-raw-matrix 4 12 41,30,31,...,231,228,0
```