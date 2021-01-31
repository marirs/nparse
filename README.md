nparse <sup>0.0.3</sup>
----------
[![Build Status](https://travis-ci.com/marirs/nparse.svg?branch=master)](https://travis-ci.com/marirs/nparse)

Parser for various [Rust](https://www.rust-lang.org/) Strings.

Parsers for:
- Well Indent Strings (eg: `dmidecode` output)
- KV pair Strings (eg: `lscpu` output)
- Multiline KV pair strings (eg: `systeminfo` output of windows)
- Dotted tree Strings (eg: `sysctl` output)

### Requirements

- Rust 1.36+

## Usage

```toml
nparse = "0.0.3"
```

### Example use

- Converting an `Indent` string into json

```rust
use std::{fs::File, io::Read};
use nparse::*;

fn main () {
    let path = "data/dmidecode.txt";
    let mut out = String::new();
    {
        let mut f = File::open(path).unwrap();
        f.read_to_string(&mut out).unwrap();
    }
    let result = out.indent_to_json();
    println!("{:?}", result);
}
```

- Converting a `K:V` string into json

```rust
use std::{fs::File, io::Read};
use nparse::*;

fn main () {
    let path = "data/lscpu.txt";
    let mut out = String::new();
    {
        let mut f = File::open(path).unwrap();
        f.read_to_string(&mut out).unwrap();
    }
    let result = out.kv_str_to_json();
    println!("{:?}", result);
}
```

- Converting a `dotted` string into json (eg: `parent.sub.sub: val`)

```rust
use std::{fs::File, io::Read};
use nparse::*;

fn main () {
    let path = "data/sysctl.txt";
    let mut out = String::new();
    {
        let mut f = File::open(path).unwrap();
        f.read_to_string(&mut out).unwrap();
    }
    let result = out.dotted_tree_to_json();
    println!("{:?}", result);
}
```

---

## Tests, Build

- Test
```bash
cargo t 
```

- Build Release
```bash
cargo b --release
```

## Examples

- Parse dmidecode output to json
```bash
cargo run --example dmidecode
```

- Parse sysctl output to json
```bash
cargo run --example sysctl
```

- Parse lscpu to json
```bash
cargo run --example lscpu
```

- Parse windows systeminfo to json
```bash
cargo run --example systeminfo
```

---
