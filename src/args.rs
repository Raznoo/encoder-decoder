use clap:: {
    Args,
    Parser,
    Subcommand,
    ValueEnum
};

type Filepath = String;

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum OutputFormat {
    Csharp,
    Vba,
    HexVBA,
}

#[derive(Debug, Parser)]
pub struct CmdArgs {
    ///Whether to use the tool for encoding or decoding
    #[clap(subcommand)]
    pub mode: ToolMode,
}

#[derive(Debug, Subcommand)]
pub enum ToolMode {
    ///Encode a file containing a msfvenom payload.
    Encode(EncodeCommand),
    ///Decode the file
    Decode(DecodeCommand)
}

#[derive(Debug, Args)]
pub struct EncodeCommand {
    ///File to ouput encoded payload to
    #[clap(long, short)]
    pub output_file: Filepath,

    ///File containing unencoded payload
    #[clap(long, short)]
    pub input_file: Filepath,

    ///File containing key to use in encryption
    #[clap(long, short)]
    pub keyfile : Option<Filepath>,

    //Format of output
    #[clap(long, short)]
    pub format: OutputFormat,
}

impl EncodeCommand {
    pub fn to_tuple(&self) -> (&String,&String,&Option<String>, &OutputFormat) {
        (&self.input_file, &self.output_file, &self.keyfile, &self.format)
    }
}

#[derive(Debug, Args)]
pub struct DecodeCommand {
    ///File to ouput decoded payload to
    #[clap(long, short)]
    pub output_file: Filepath,

    ///File containing encoded payload
    #[clap(long, short)]
    pub input_file: Filepath,

    ///File containing key to use in decryption
    #[clap(long, short)]
    pub keyfile : Filepath,
}