use clap::{Args, Parser, Subcommand};
use clap_num::maybe_hex;
use qmk_via_api::api::{KeyboardApi, KeyboardValue, Layer, MatrixInfo};

#[derive(Debug, Parser)]
#[clap(name = "via-cli", version = env!("CARGO_PKG_VERSION"))]
pub struct App {
    #[clap(subcommand)]
    command: Command,

    /// Vendor ID
    #[clap(long, value_parser=maybe_hex::<u16>)]
    vid: u16,

    /// Product ID
    #[clap(long, value_parser=maybe_hex::<u16>)]
    pid: u16,

    /// Usage page
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

fn main() {
    let app = App::parse();
    let api = match KeyboardApi::new(app.vid, app.pid, app.usage_page) {
        Ok(api) => api,
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    };

    match app.command {
        Command::SendRawData(args) => {
            print_result(api.hid_send(args.data));
        }
        Command::ReceiveRawData => {
            print_result(api.hid_read());
        }
        Command::GetProtocolVersion => {
            print_result(api.get_protocol_version());
        }
        Command::GetLayerCount => {
            print_result(api.get_layer_count());
        }
        Command::GetKey(args) => {
            print_result(api.get_key(args.layer, args.row, args.column));
        }
        Command::SetKey(args) => {
            print_result(api.set_key(args.layer, args.row, args.column, args.value));
        }
        Command::ReadRawMatrix(args) => {
            let matrix_info = MatrixInfo {
                rows: args.rows,
                cols: args.cols,
            };
            print_result(api.read_raw_matrix(matrix_info, args.layer));
        }
        Command::WriteRawMatrix(args) => {
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
            print_result(api.get_keyboard_value(args.command, args.parameters, args.result_length));
        }
        Command::SetKeyboardValue(args) => {
            print_result(api.set_keyboard_value(args.command, args.parameters));
        }
        Command::GetEncoderValue(args) => {
            print_result(api.get_encoder_value(args.layer, args.id, args.is_clockwise));
        }
        Command::SetEncoderValue(args) => {
            print_result(api.set_encoder_value(
                args.layer,
                args.id,
                args.is_clockwise,
                args.keycode,
            ));
        }
        Command::GetCustomMenuValue(args) => {
            print_result(api.get_custom_menu_value(args.command_bytes));
        }
        Command::SetCustomMenuValue(args) => {
            print_result(api.set_custom_menu_value(args.args));
        }
        Command::SaveCustomMenu(args) => {
            print_result(api.save_custom_menu(args.channel));
        }
        Command::GetBacklightBrightness => {
            print_result(api.get_backlight_brightness());
        }
        Command::SetBacklightBrightness(args) => {
            print_result(api.set_backlight_brightness(args.brightness));
        }
        Command::GetBacklightEffect => {
            print_result(api.get_backlight_effect());
        }
        Command::SetBacklightEffect(args) => {
            print_result(api.set_backlight_effect(args.effect));
        }
        Command::GetRgblightBrightness => {
            print_result(api.get_rgblight_brightness());
        }
        Command::SetRgblightBrightness(args) => {
            print_result(api.set_rgblight_brightness(args.brightness));
        }
        Command::GetRgblightEffect => {
            print_result(api.get_rgblight_effect());
        }
        Command::SetRgblightEffect(args) => {
            print_result(api.set_rgblight_effect(args.effect));
        }
        Command::GetRgblightEffectSpeed => {
            print_result(api.get_rgblight_effect_speed());
        }
        Command::SetRgblightEffectSpeed(args) => {
            print_result(api.set_rgblight_effect_speed(args.speed));
        }
        Command::GetRgblightColor => {
            print_result(api.get_rgblight_color());
        }
        Command::SetRgblightColor(args) => {
            print_result(api.set_rgblight_color(args.hue, args.sat));
        }
        Command::GetRgbMatrixBrightness => {
            print_result(api.get_rgb_matrix_brightness());
        }
        Command::SetRgbMatrixBrightness(args) => {
            print_result(api.set_rgb_matrix_brightness(args.brightness));
        }
        Command::GetRgbMatrixEffect => {
            print_result(api.get_rgb_matrix_effect());
        }
        Command::SetRgbMatrixEffect(args) => {
            print_result(api.set_rgb_matrix_effect(args.effect));
        }
        Command::GetRgbMatrixEffectSpeed => {
            print_result(api.get_rgb_matrix_effect_speed());
        }
        Command::SetRgbMatrixEffectSpeed(args) => {
            print_result(api.set_rgb_matrix_effect_speed(args.speed));
        }
        Command::GetRgbMatrixColor => {
            print_result(api.get_rgb_matrix_color());
        }
        Command::SetRgbMatrixColor(args) => {
            print_result(api.set_rgb_matrix_color(args.hue, args.sat));
        }
        Command::GetLedMatrixBrightness => {
            print_result(api.get_led_matrix_brightness());
        }
        Command::SetLedMatrixBrightness(args) => {
            print_result(api.set_led_matrix_brightness(args.brightness));
        }
        Command::GetLedMatrixEffect => {
            print_result(api.get_led_matrix_effect());
        }
        Command::SetLedMatrixEffect(args) => {
            print_result(api.set_led_matrix_effect(args.effect));
        }
        Command::GetLedMatrixEffectSpeed => {
            print_result(api.get_led_matrix_effect_speed());
        }
        Command::SetLedMatrixEffectSpeed(args) => {
            print_result(api.set_led_matrix_effect_speed(args.speed));
        }
        Command::SaveLighting => {
            print_result(api.save_lighting());
        }
        Command::ResetEeprom => {
            print_result(api.reset_eeprom());
        }
        Command::JumpToBootloader => {
            print_result(api.jump_to_bootloader());
        }
        Command::GetMacroCount => {
            print_result(api.get_macro_count());
        }
        Command::GetMacroBytes => {
            print_result(api.get_macro_bytes());
        }
        Command::SetMacroBytes(args) => {
            print_result(api.set_macro_bytes(args.data));
        }
        Command::ResetMacros => {
            print_result(api.reset_macros());
        }
    }
}
