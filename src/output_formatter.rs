const CSHARP_MAX_LINE_LEN: usize = 100;
const VBA_MAX_LINE_LEN: usize = 1000;
const HEX_VBA_MAX_LINE_LEN: usize = 1000;

pub fn handle_vba_output(encoded_bytes: Vec<u8>, var_name: &str) -> String {
    let mut result = String::new();
    let prefix = format!("        {} = Array(", var_name);
    let suffix = ")";
    let mut current_line = prefix.clone();

    for (i, byte) in encoded_bytes.iter().enumerate() {
        let formatted_byte = if i == encoded_bytes.len() - 1 {
            format!("{}", byte) // No comma for the last byte
        } else {
            format!("{} ,", byte)
        };

        if current_line.len() + formatted_byte.len() > VBA_MAX_LINE_LEN {
            result.push_str(&current_line);
            result.push_str(" _\n");
            current_line.clear();
        }
        current_line.push_str(&formatted_byte);
    }

    result.push_str(&current_line);
    result.push_str(suffix);
    result
}


pub fn handle_hex_vba_output(encoded_bytes: Vec<u8>, var_name: &str) -> String {
    let mut result = String::new();
    let prefix = format!("Dim {} As String\n{} = \"", var_name,var_name);
    let mut current_line = prefix.clone();

    for (i, byte) in encoded_bytes.iter().enumerate() {
        let formatted_byte = if i == encoded_bytes.len() - 1 {
            format!("{:02x}\"", byte) // No comma for the last byte
        } else {
            format!("{:02x}", byte)
        };

        if current_line.len() + formatted_byte.len() > HEX_VBA_MAX_LINE_LEN {
            result.push_str(&current_line);
            result.push_str("\"\n");
            result.push_str(var_name);
            result.push_str(" = ");
            result.push_str(var_name);
            result.push_str(" & \"");
            current_line.clear();
        }
        current_line.push_str(&formatted_byte);
    }

    result.push_str(&current_line);
    result
}

pub fn handle_csharp_output(encoded_bytes: Vec<u8>, var_name: &str) -> String {
    let mut result = String::new();
    let prefix = format!("byte[] {} = new byte[{}] {{", var_name, encoded_bytes.len());
    let suffix = "};";
    let mut current_line = prefix.clone();

    for (i, byte) in encoded_bytes.iter().enumerate() {
        let formatted_byte = if i == encoded_bytes.len() - 1 {
            format!("0x{:02x}", byte) // No comma for the last byte
        } else {
            format!("0x{:02x},", byte)
        };

        if current_line.len() + formatted_byte.len() > CSHARP_MAX_LINE_LEN {
            result.push_str(&current_line);
            result.push('\n');
            current_line.clear();
        }
        current_line.push_str(&formatted_byte);
    }

    result.push_str(&current_line);
    result.push_str(suffix);
    result
}