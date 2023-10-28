extern crate rand;

use rand::Rng;
use std::fs::File;
use std::io::{Read, Write};
use std::result::Result;

use crate::args::OutputFormat;
use crate::output_formatter;

pub fn handle_encode(
    (input_filepath, output_file, keyfile, output_format): (
        &String,
        &String,
        &Option<String>,
        &OutputFormat,
    ),
) -> Result<(), std::io::Error> {
    if let Some(_) = keyfile {
        panic!("[ERROR!!!] Inputted keys haven't been implemented yet.")
    }

    let mut input_file = File::open(input_filepath).unwrap();

    let mut unencoded_bytes = Vec::new();
    input_file.read_to_end(&mut unencoded_bytes)?;
    let key = generate_key(unencoded_bytes.len());
    let mut encoded_bytes = Vec::new();
    for (i, unencoded_byte) in unencoded_bytes.into_iter().enumerate() {
        let encoded_byte = unencoded_byte ^ key[i];
        encoded_bytes.push(encoded_byte);
    }

    write_to_file(output_file, "moose", encoded_bytes, output_format)?;
    write_to_file("key", "key", key, output_format)?;
    Ok(())
}

fn write_to_file(
    filename: &str,
    variable_name: &str,
    data: Vec<u8>,
    output_format: &OutputFormat,
) -> Result<(), std::io::Error> {
    let mut file = File::create(filename)?;

    let results: String = match output_format {
        OutputFormat::Csharp => output_formatter::handle_csharp_output(data, variable_name),
        OutputFormat::Vba => output_formatter::handle_vba_output(data, variable_name),
        OutputFormat::HexVBA => output_formatter::handle_hex_vba_output(data, variable_name),
    };
    
    write!(file, "{}", results)?;
    Ok(())
}

fn generate_key(len: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..len).map(|_| rng.gen::<u8>()).collect()
}
