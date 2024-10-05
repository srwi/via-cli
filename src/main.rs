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

    /// Get the per-key RGB matrix
    GetPerKeyRgbMatrix(GetPerKeyRgbMatrixArgs),

    /// Set a per-key RGB matrix value
    SetPerKeyRgbMatrix(SetPerKeyRgbMatrixArgs),

    /// Get the current RGB mode
    GetRgbMode,

    /// Get the current brightness
    GetBrightness,

    /// Get a color
    GetColor(GetColorArgs),

    /// Set a color
    SetColor(SetColorArgs),

    /// Get a custom color
    GetCustomColor(GetCustomColorArgs),

    /// Set a custom color
    SetCustomColor(SetCustomColorArgs),

    /// Set the RGB mode
    SetRgbMode(SetRgbModeArgs),

    /// Commit custom menu changes
    CommitCustomMenu(CommitCustomMenuArgs),

    /// Save lighting configuration
    SaveLighting,

    /// Reset EEPROM
    ResetEeprom,

    /// Jump to bootloader
    JumpToBootloader,

    /// Get the macro count
    GetMacroCount,

    /// Get the macro buffer size
    GetMacroBufferSize,

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
struct GetPerKeyRgbMatrixArgs {
    #[clap(value_delimiter = ',', value_parser=maybe_hex::<u8>)]
    led_index_mapping: Vec<u8>,
}

#[derive(Debug, Args)]
struct SetPerKeyRgbMatrixArgs {
    index: u8,
    hue: u8,
    sat: u8,
}

#[derive(Debug, Args)]
struct GetColorArgs {
    color_number: u8,
}

#[derive(Debug, Args)]
struct SetColorArgs {
    color_number: u8,
    hue: u8,
    sat: u8,
}

#[derive(Debug, Args)]
struct GetCustomColorArgs {
    color_number: u8,
}

#[derive(Debug, Args)]
struct SetCustomColorArgs {
    color_number: u8,
    hue: u8,
    sat: u8,
}

#[derive(Debug, Args)]
struct SetRgbModeArgs {
    effect: u8,
}

#[derive(Debug, Args)]
struct CommitCustomMenuArgs {
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
        Command::GetPerKeyRgbMatrix(args) => {
            print_result(api.get_per_key_rgb_matrix(args.led_index_mapping));
        }
        Command::SetPerKeyRgbMatrix(args) => {
            print_result(api.set_per_key_rgb_matrix(args.index, args.hue, args.sat));
        }
        Command::GetRgbMode => {
            print_result(api.get_rgb_mode());
        }
        Command::GetBrightness => {
            print_result(api.get_brightness());
        }
        Command::GetColor(args) => {
            print_result(api.get_color(args.color_number));
        }
        Command::SetColor(args) => {
            print_result(api.set_color(args.color_number, args.hue, args.sat));
        }
        Command::GetCustomColor(args) => {
            print_result(api.get_custom_color(args.color_number));
        }
        Command::SetCustomColor(args) => {
            print_result(api.set_custom_color(args.color_number, args.hue, args.sat));
        }
        Command::SetRgbMode(args) => {
            print_result(api.set_rgb_mode(args.effect));
        }
        Command::CommitCustomMenu(args) => {
            print_result(api.commit_custom_menu(args.channel));
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
        Command::GetMacroBufferSize => {
            print_result(api.get_macro_buffer_size());
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
