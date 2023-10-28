use clap::Parser;

mod args;
mod encode;
mod output_formatter;

use args::{CmdArgs, ToolMode};
use encode::handle_encode;
fn main() {
    let args = CmdArgs::parse();

    match args.mode {
        ToolMode::Encode(encode_args) => handle_encode(encode_args.to_tuple()).unwrap(),
        ToolMode::Decode(_) => todo!("Make a decode"),
    }
}