extern crate rand;

use std::fs::File;
use std::io::{Write, BufReader, BufRead, Result};
use rand::Rng;

pub fn handle_encode(encode_args: (&String, &String, &Option<String>)) -> () {
    let (input_filepath, output_file, keyfile) = encode_args;
    let input_file = File::open(input_filepath).unwrap();
    let reader = BufReader::new(input_file);

    let unencoded_bytes = read_input_file(reader);
    let key = generate_key(unencoded_bytes.len());
    let mut encoded_bytes = Vec::new();
    for (i, unencoded_byte) in unencoded_bytes.into_iter().enumerate(){
        let encoded_byte = unencoded_byte ^ key[i];
        encoded_bytes.push(encoded_byte);
    }

    write_to_file(output_file, "buf", encoded_bytes);
    write_to_file("key.txt", "key", key);
}

fn write_to_file(filename: &str, variable_name: &str, data: Vec<u8>) -> Result<()> {
    let mut file = File::create(filename)?;
    write!(file, "byte[] {} = new byte[{}] {{", variable_name, data.len())?;

    // Handle the first line separately
    let mut bytes_written = 0;
    for &byte in data.iter().take(6) {
        write!(file, "0x{:02x},", byte)?;
        bytes_written += 1;
    }
    writeln!(file)?;

    // Handle the rest of the data, 12 bytes per line
    for chunk in data[bytes_written..].chunks(12) {
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

fn string_to_byte(byte_string: &str) -> Option<u8> {
    if let Some(hex_str) = byte_string.strip_prefix("0x") {
        Some(u8::from_str_radix(hex_str, 16).unwrap())
    } else {
        None
    }
}

fn read_input_file(input_reader: BufReader<File>) -> Vec<u8> {
    let mut result = Vec::new();

    let mut prev_line: Option<String> = None;
    let mut is_first = true;

    for line_result in input_reader.lines() {
        match line_result {
            Ok(line) => {
                if let Some(old_line) = prev_line {
                    let parts = if is_first {
                        let sub_part = old_line.split_once('{').unwrap().1;
                        is_first = false;
                        sub_part.split(',')
                    } else {
                        old_line.split(',')
                    };
                    parts.for_each(|part: &str| {
                        if let Some(byte_val) = string_to_byte(part) {
                            result.push(byte_val)
                        }
                    });
                }
                prev_line = Some(line);
            }
            Err(e) => println!("Error reading a line: {}", e),
        }
    }
    //have to do extra parsing for last line :/
    prev_line
        .unwrap()
        .replace("};", "")
        .split(',')
        .for_each(|part: &str| {
            if let Some(byte_val) = string_to_byte(part) {
                result.push(byte_val)
            }
        });

    result
}
