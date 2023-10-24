extern crate rand;

use rand::Rng;
use std::fs::File;
use std::result::Result;
use std::io::{Read, Write};

const BYTES_PER_LINE: usize = 12;
const BYTES_IN_FIRST_LINE: usize = 6;

pub fn handle_encode(encode_args: (&String, &String, &Option<String>)) -> Result<(), std::io::Error> {
    let (input_filepath, output_file, keyfile) = encode_args;
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

    write_to_file(output_file, "buf", encoded_bytes)?;
    write_to_file("key.cs", "key", key)?;
    Ok(())
}

fn write_to_file(filename: &str, variable_name: &str, data: Vec<u8>) -> Result<(), std::io::Error> {
    let mut file = File::create(filename)?;
    write!(
        file,
        "byte[] {} = new byte[{}] {{",
        variable_name,
        data.len()
    )?;

    // Handle the first line separately
    let mut bytes_written = 0;
    for &byte in data.iter().take(BYTES_IN_FIRST_LINE) {
        write!(file, "0x{:02x},", byte)?;
        bytes_written += 1;
    }
    writeln!(file)?;

    for chunk in data[bytes_written..].chunks(BYTES_PER_LINE) {
        for (i, &byte) in chunk.iter().enumerate() {
            if bytes_written + i == data.len() - 1 {
                write!(file, "0x{:02x}}};", byte)?;
            } else {
                write!(file, "0x{:02x},", byte)?;
            }
        }
        bytes_written += chunk.len();
        if bytes_written != data.len() {
            writeln!(file)?;
        }
    }

    Ok(())
}

fn generate_key(len: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    (0..len).map(|_| rng.gen::<u8>()).collect()
}
