use clap::Parser;

mod args;
mod encode;

use args::{CmdArgs, ToolMode};
use encode::handle_encode;
fn main() {
    let args = CmdArgs::parse();

    match args.mode {
        ToolMode::Encode(encode_args) => handle_encode(encode_args.to_tuple()),
        ToolMode::Decode(decode_args) => todo!("Make a decode"),
    }
}