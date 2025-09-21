# via-cli

A command-line interface (CLI) for the [VIA API](https://github.com/srwi/qmk-via-api), designed for use with keyboards running the [QMK](https://github.com/qmk/qmk_firmware) (Quantum Mechanical Keyboard) firmware.

## Usage

> [!NOTE]
> To run `via-cli` on Linux, you need to have `libudev` installed.

Keycodes are displayed in decimal format by default. They can be provided in either decimal or hexadecimal format (prefixed with `0x`).

```
Usage: via-cli [OPTIONS] --vid <VID> --pid <PID> <COMMAND>

Commands:
  send-raw-data                Send raw byte data (up to 32 bytes)
  receive-raw-data             Receive raw byte data (32 bytes)
  get-protocol-version         Get the protocol version
  get-layer-count              Get the layer count
  get-key                      Get a key from a specific layer, row, and column
  set-key                      Set a key at a specific layer, row, and column
  read-raw-matrix              Read the raw matrix for a specific layer
  write-raw-matrix             Write the raw matrix
  get-keyboard-value           Get a keyboard value
  set-keyboard-value           Set a keyboard value
  get-encoder-value            Get an encoder value
  set-encoder-value            Set an encoder value
  get-custom-menu-value        Get a custom menu value
  set-custom-menu-value        Set a custom menu value
  save-custom-menu             Save custom menu values for a specific channel
  get-backlight-brightness     Get the backlight brightness
  set-backlight-brightness     Set the backlight brightness
  get-backlight-effect         Get the backlight effect
  set-backlight-effect         Set the backlight effect
  get-rgblight-brightness      Get the RGB light brightness
  set-rgblight-brightness      Set the RGB light brightness
  get-rgblight-effect          Get the RGB light effect
  set-rgblight-effect          Set the RGB light effect
  get-rgblight-effect-speed    Get the RGB light effect speed
  set-rgblight-effect-speed    Set the RGB light effect speed
  get-rgblight-color           Get the RGB light color
  set-rgblight-color           Set the RGB light color
  get-rgb-matrix-brightness    Get the RGB matrix brightness
  set-rgb-matrix-brightness    Set the RGB matrix brightness
  get-rgb-matrix-effect        Get the RGB matrix effect
  set-rgb-matrix-effect        Set the RGB matrix effect
  get-rgb-matrix-effect-speed  Get the RGB matrix effect speed
  set-rgb-matrix-effect-speed  Set the RGB matrix effect speed
  get-rgb-matrix-color         Get the RGB matrix color
  set-rgb-matrix-color         Set the RGB matrix color
  get-led-matrix-brightness    Get the LED matrix brightness
  set-led-matrix-brightness    Set the LED matrix brightness
  get-led-matrix-effect        Get the LED matrix effect
  set-led-matrix-effect        Set the LED matrix effect
  get-led-matrix-effect-speed  Get the LED matrix effect speed
  set-led-matrix-effect-speed  Set the LED matrix effect speed
  save-lighting                Save lighting configuration
  reset-eeprom                 Reset EEPROM
  jump-to-bootloader           Jump to bootloader
  get-macro-count              Get the macro count
  get-macro-bytes              Get the macro bytes
  set-macro-bytes              Set the macro bytes
  reset-macros                 Reset macros
  scan                         Scan and list connected VIA keyboards
  help                         Print this message or the help of the given subcommand(s)

Options:
      --vid <VID>                Vendor ID
      --pid <PID>                Product ID
      --usage-page <USAGE_PAGE>  Usage page [default: 65376]
  -h, --help                     Print help
  -V, --version                  Print version
```

## Examples

Scan for connected VIA devices:

```console
$ via-cli scan
+------+------+-------+--------------+---------+--------+
| VID  | PID  | Usage | Manufacturer | Product | Serial |
+------+------+-------+--------------+---------+--------+
| 7372 | 0003 | ff60  | srwi         | Lily58  |        |
+------+------+-------+--------------+---------+--------+
```

Get decimal keycode of row 2 and column 7 on layer 0:

```console
$ via-cli get-key -h
Get a key from a specific layer, row, and column

Usage: via-cli --vid <VID> --pid <PID> get-key <LAYER> <ROW> <COLUMN>

$ via-cli --vid 0x594D --pid 0x604D get-key 0 2 7
13
```

Write keymap for a keyboard with 4 rows and 12 columns:

```console
$ via-cli write-raw-matrix -h
Write the raw matrix

Usage: via-cli --vid <VID> --pid <PID> write-raw-matrix <ROWS> <COLS> [KEYMAP]...

$ via-cli --vid 0x594D --pid 0x604D write-raw-matrix 4 12 41,30,31,...,231,228,0
```