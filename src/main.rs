use clap::{Args, Parser, Subcommand};
use clap_num::maybe_hex;
use cli_table::{print_stdout, Table};
use qmk_via_api::api::{KeyboardApi, KeyboardValue, Layer, MatrixInfo};
use qmk_via_api::scan;

#[derive(Debug, Parser)]
#[clap(name = "via-cli", version = env!("CARGO_PKG_VERSION"))]
pub struct App {
    #[clap(subcommand)]
    command: Command,

    /// Vendor ID
    #[clap(long, value_parser=maybe_hex::<u16>)]
    vid: Option<u16>,

    /// Product ID
    #[clap(long, value_parser=maybe_hex::<u16>)]
    pid: Option<u16>,

    /// Usage page (defaults to 0xFF60)
    #[clap(long, value_parser=maybe_hex::<u16>, default_value_t = 0xff60)]
    usage_page: u16,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Send raw byte data (up to 32 bytes)
    SendRawData(SendDataArgs),

    /// Receive raw byte data (32 bytes)
    ReceiveRawData,

    /// Get the protocol version
    GetProtocolVersion,

    /// Get the layer count
    GetLayerCount,

    /// Get a key from a specific layer, row, and column
    GetKey(GetKeyArgs),

    /// Set a key at a specific layer, row, and column
    SetKey(SetKeyArgs),

    /// Read the raw matrix for a specific layer
    ReadRawMatrix(ReadRawMatrixArgs),

    /// Write the raw matrix
    WriteRawMatrix(WriteRawMatrixArgs),

    /// Get a keyboard value
    GetKeyboardValue(GetKeyboardValueArgs),

    /// Set a keyboard value
    SetKeyboardValue(SetKeyboardValueArgs),

    /// Get an encoder value
    GetEncoderValue(GetEncoderValueArgs),

    /// Set an encoder value
    SetEncoderValue(SetEncoderValueArgs),

    /// Get a custom menu value
    GetCustomMenuValue(GetCustomMenuValueArgs),

    /// Set a custom menu value
    SetCustomMenuValue(SetCustomMenuValueArgs),

    /// Save custom menu values for a specific channel
    SaveCustomMenu(SaveCustomMenuArgs),

    /// Get the backlight brightness
    GetBacklightBrightness,

    /// Set the backlight brightness
    SetBacklightBrightness(SetBacklightBrightnessArgs),

    /// Get the backlight effect
    GetBacklightEffect,

    /// Set the backlight effect
    SetBacklightEffect(SetBacklightEffectArgs),

    /// Get the RGB light brightness
    GetRgblightBrightness,

    /// Set the RGB light brightness
    SetRgblightBrightness(SetRgblightBrightnessArgs),

    /// Get the RGB light effect
    GetRgblightEffect,

    /// Set the RGB light effect
    SetRgblightEffect(SetRgblightEffectArgs),

    /// Get the RGB light effect speed
    GetRgblightEffectSpeed,

    /// Set the RGB light effect speed
    SetRgblightEffectSpeed(SetRgblightEffectSpeedArgs),

    /// Get the RGB light color
    GetRgblightColor,

    /// Set the RGB light color
    SetRgblightColor(SetRgblightColorArgs),

    /// Get the RGB matrix brightness
    GetRgbMatrixBrightness,

    /// Set the RGB matrix brightness
    SetRgbMatrixBrightness(SetRgbMatrixBrightnessArgs),

    /// Get the RGB matrix effect
    GetRgbMatrixEffect,

    /// Set the RGB matrix effect
    SetRgbMatrixEffect(SetRgbMatrixEffectArgs),

    /// Get the RGB matrix effect speed
    GetRgbMatrixEffectSpeed,

    /// Set the RGB matrix effect speed
    SetRgbMatrixEffectSpeed(SetRgbMatrixEffectSpeedArgs),

    /// Get the RGB matrix color
    GetRgbMatrixColor,

    /// Set the RGB matrix color
    SetRgbMatrixColor(SetRgbMatrixColorArgs),

    /// Get the LED matrix brightness
    GetLedMatrixBrightness,

    /// Set the LED matrix brightness
    SetLedMatrixBrightness(SetLedMatrixBrightnessArgs),

    /// Get the LED matrix effect
    GetLedMatrixEffect,

    /// Set the LED matrix effect
    SetLedMatrixEffect(SetLedMatrixEffectArgs),

    /// Get the LED matrix effect speed
    GetLedMatrixEffectSpeed,

    /// Set the LED matrix effect speed
    SetLedMatrixEffectSpeed(SetLedMatrixEffectSpeedArgs),

    /// Save lighting configuration
    SaveLighting,

    /// Reset EEPROM
    ResetEeprom,

    /// Jump to bootloader
    JumpToBootloader,

    /// Get the macro count
    GetMacroCount,

    /// Get the macro bytes
    GetMacroBytes,

    /// Set the macro bytes
    SetMacroBytes(SetMacroBytesArgs),

    /// Reset macros
    ResetMacros,

    /// Scan and list connected VIA keyboards
    Scan,
}

#[derive(Debug, Args)]
struct SendDataArgs {
    #[clap(value_delimiter = ',', value_parser=maybe_hex::<u8>)]
    data: Vec<u8>,
}

#[derive(Debug, Args)]
struct GetKeyArgs {
    layer: Layer,
    row: u8,
    column: u8,
}

#[derive(Debug, Args)]
struct SetKeyArgs {
    layer: Layer,
    row: u8,
    column: u8,
    #[clap(value_parser=maybe_hex::<u16>)]
    value: u16,
}

#[derive(Debug, Args)]
struct ReadRawMatrixArgs {
    layer: Layer,
    rows: u8,
    cols: u8,
}

#[derive(Debug, Args)]
struct WriteRawMatrixArgs {
    rows: u8,
    cols: u8,
    #[clap(value_delimiter = ',', value_parser=maybe_hex::<u16>)]
    keymap: Vec<u16>,
}

#[derive(Debug, Args)]
struct GetKeyboardValueArgs {
    command: KeyboardValue,
    result_length: usize,
    #[clap(value_delimiter = ',', value_parser=maybe_hex::<u8>)]
    parameters: Vec<u8>,
}

#[derive(Debug, Args)]
struct SetKeyboardValueArgs {
    command: KeyboardValue,
    #[clap(value_delimiter = ',', value_parser=maybe_hex::<u8>)]
    parameters: Vec<u8>,
}

#[derive(Debug, Args)]
struct GetEncoderValueArgs {
    layer: Layer,
    #[clap(value_parser=maybe_hex::<u8>)]
    id: u8,
    is_clockwise: bool,
}

#[derive(Debug, Args)]
struct SetEncoderValueArgs {
    layer: Layer,
    #[clap(value_parser=maybe_hex::<u8>)]
    id: u8,
    is_clockwise: bool,
    #[clap(value_parser=maybe_hex::<u16>)]
    keycode: u16,
}

#[derive(Debug, Args)]
struct GetCustomMenuValueArgs {
    #[clap(value_delimiter = ',', value_parser=maybe_hex::<u8>)]
    command_bytes: Vec<u8>,
}

#[derive(Debug, Args)]
struct SetCustomMenuValueArgs {
    #[clap(value_delimiter = ',', value_parser=maybe_hex::<u8>)]
    args: Vec<u8>,
}

#[derive(Debug, Args)]
struct SetBacklightBrightnessArgs {
    brightness: u8,
}

#[derive(Debug, Args)]
struct SetBacklightEffectArgs {
    effect: u8,
}

#[derive(Debug, Args)]
struct SetRgblightBrightnessArgs {
    brightness: u8,
}

#[derive(Debug, Args)]
struct SetRgblightEffectArgs {
    effect: u8,
}

#[derive(Debug, Args)]
struct SetRgblightEffectSpeedArgs {
    speed: u8,
}

#[derive(Debug, Args)]
struct SetRgblightColorArgs {
    hue: u8,
    sat: u8,
}

#[derive(Debug, Args)]
struct SetRgbMatrixBrightnessArgs {
    brightness: u8,
}

#[derive(Debug, Args)]
struct SetRgbMatrixEffectArgs {
    effect: u8,
}

#[derive(Debug, Args)]
struct SetRgbMatrixEffectSpeedArgs {
    speed: u8,
}

#[derive(Debug, Args)]
struct SetRgbMatrixColorArgs {
    hue: u8,
    sat: u8,
}

#[derive(Debug, Args)]
struct SetLedMatrixBrightnessArgs {
    brightness: u8,
}

#[derive(Debug, Args)]
struct SetLedMatrixEffectArgs {
    effect: u8,
}

#[derive(Debug, Args)]
struct SetLedMatrixEffectSpeedArgs {
    speed: u8,
}

#[derive(Debug, Args)]
struct SaveCustomMenuArgs {
    channel: u8,
}

#[derive(Debug, Args)]
struct SetMacroBytesArgs {
    #[clap(value_delimiter = ',', value_parser=maybe_hex::<u8>)]
    data: Vec<u8>,
}

fn print_result<T: std::fmt::Debug>(result: Option<T>) {
    match result {
        Some(value) => {
            if std::any::type_name::<T>() != "()" {
                println!("{:?}", value);
            }
        }
        None => eprintln!("Command rejected by device."),
    }
}

fn get_api(vid: Option<u16>, pid: Option<u16>, usage_page: u16) -> KeyboardApi {
    let (v, p) = match (vid, pid) {
        (Some(v), Some(p)) => (v, p),
        _ => {
            eprintln!("--vid and --pid are required for this command. Use 'via-cli scan' to list connected devices.");
            std::process::exit(2);
        }
    };
    match KeyboardApi::new(v, p, usage_page) {
        Ok(api) => api,
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    }
}

fn main() {
    let app = App::parse();

    match app.command {
        Command::SendRawData(args) => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.hid_send(args.data));
        }
        Command::ReceiveRawData => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.hid_read());
        }
        Command::GetProtocolVersion => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.get_protocol_version());
        }
        Command::GetLayerCount => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.get_layer_count());
        }
        Command::GetKey(args) => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.get_key(args.layer, args.row, args.column));
        }
        Command::SetKey(args) => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.set_key(args.layer, args.row, args.column, args.value));
        }
        Command::ReadRawMatrix(args) => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            let matrix_info = MatrixInfo {
                rows: args.rows,
                cols: args.cols,
            };
            print_result(api.read_raw_matrix(matrix_info, args.layer));
        }
        Command::WriteRawMatrix(args) => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            let keymap: Vec<Vec<u16>> =
                args.keymap.chunks(16).map(|chunk| chunk.to_vec()).collect();
            print_result(api.write_raw_matrix(
                MatrixInfo {
                    rows: args.rows,
                    cols: args.cols,
                },
                keymap,
            ));
        }
        Command::GetKeyboardValue(args) => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.get_keyboard_value(args.command, args.parameters, args.result_length));
        }
        Command::SetKeyboardValue(args) => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.set_keyboard_value(args.command, args.parameters));
        }
        Command::GetEncoderValue(args) => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.get_encoder_value(args.layer, args.id, args.is_clockwise));
        }
        Command::SetEncoderValue(args) => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.set_encoder_value(
                args.layer,
                args.id,
                args.is_clockwise,
                args.keycode,
            ));
        }
        Command::GetCustomMenuValue(args) => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.get_custom_menu_value(args.command_bytes));
        }
        Command::SetCustomMenuValue(args) => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.set_custom_menu_value(args.args));
        }
        Command::SaveCustomMenu(args) => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.save_custom_menu(args.channel));
        }
        Command::GetBacklightBrightness => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.get_backlight_brightness());
        }
        Command::SetBacklightBrightness(args) => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.set_backlight_brightness(args.brightness));
        }
        Command::GetBacklightEffect => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.get_backlight_effect());
        }
        Command::SetBacklightEffect(args) => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.set_backlight_effect(args.effect));
        }
        Command::GetRgblightBrightness => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.get_rgblight_brightness());
        }
        Command::SetRgblightBrightness(args) => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.set_rgblight_brightness(args.brightness));
        }
        Command::GetRgblightEffect => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.get_rgblight_effect());
        }
        Command::SetRgblightEffect(args) => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.set_rgblight_effect(args.effect));
        }
        Command::GetRgblightEffectSpeed => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.get_rgblight_effect_speed());
        }
        Command::SetRgblightEffectSpeed(args) => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.set_rgblight_effect_speed(args.speed));
        }
        Command::GetRgblightColor => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.get_rgblight_color());
        }
        Command::SetRgblightColor(args) => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.set_rgblight_color(args.hue, args.sat));
        }
        Command::GetRgbMatrixBrightness => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.get_rgb_matrix_brightness());
        }
        Command::SetRgbMatrixBrightness(args) => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.set_rgb_matrix_brightness(args.brightness));
        }
        Command::GetRgbMatrixEffect => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.get_rgb_matrix_effect());
        }
        Command::SetRgbMatrixEffect(args) => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.set_rgb_matrix_effect(args.effect));
        }
        Command::GetRgbMatrixEffectSpeed => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.get_rgb_matrix_effect_speed());
        }
        Command::SetRgbMatrixEffectSpeed(args) => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.set_rgb_matrix_effect_speed(args.speed));
        }
        Command::GetRgbMatrixColor => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.get_rgb_matrix_color());
        }
        Command::SetRgbMatrixColor(args) => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.set_rgb_matrix_color(args.hue, args.sat));
        }
        Command::GetLedMatrixBrightness => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.get_led_matrix_brightness());
        }
        Command::SetLedMatrixBrightness(args) => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.set_led_matrix_brightness(args.brightness));
        }
        Command::GetLedMatrixEffect => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.get_led_matrix_effect());
        }
        Command::SetLedMatrixEffect(args) => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.set_led_matrix_effect(args.effect));
        }
        Command::GetLedMatrixEffectSpeed => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.get_led_matrix_effect_speed());
        }
        Command::SetLedMatrixEffectSpeed(args) => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.set_led_matrix_effect_speed(args.speed));
        }
        Command::SaveLighting => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.save_lighting());
        }
        Command::ResetEeprom => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.reset_eeprom());
        }
        Command::JumpToBootloader => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.jump_to_bootloader());
        }
        Command::GetMacroCount => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.get_macro_count());
        }
        Command::GetMacroBytes => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.get_macro_bytes());
        }
        Command::SetMacroBytes(args) => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.set_macro_bytes(args.data));
        }
        Command::ResetMacros => {
            let api = get_api(app.vid, app.pid, app.usage_page);
            print_result(api.reset_macros());
        }
        Command::Scan => {
            let devices = scan::scan_keyboards();
            if devices.is_empty() {
                println!("No VIA devices found.");
            } else {
                let rows: Vec<Vec<String>> = devices
                    .iter()
                    .map(|device| {
                        vec![
                            format!("{:04x}", device.vendor_id),
                            format!("{:04x}", device.product_id),
                            format!("{:04x}", device.usage_page),
                            device.manufacturer.clone().unwrap_or_default(),
                            device.product.clone().unwrap_or_default(),
                            device.serial_number.clone().unwrap_or_default(),
                        ]
                    })
                    .collect();
                let table = rows.table().title(vec![
                    "VID",
                    "PID",
                    "Usage",
                    "Manufacturer",
                    "Product",
                    "Serial",
                ]);
                print_stdout(table).unwrap();
            }
        }
    }
}
