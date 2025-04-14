# chorba

![](https://img.shields.io/badge/language-Rust-red) ![](https://img.shields.io/badge/version-0.1.0%20alpha-brightgreen) [![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/myyrakle/chorba/blob/master/LICENSE)

- simple binary serialization

## Basic

- Serializes a structure into binary format.
- The values ​​are placed in the order of the structure fields, so the actual order of the fields is important.

## Get Started

Install via cargo add

```bash
cargo add chorba
```

or, Modify Cargo.toml.

```toml
chorba = "0.1.0"
```

The usage is simple. Just apply the logic through derive and convert it through the encode/decode functions.

```rust
use chorba::{Decode, Encode, decode, encode};

#[derive(Encode, Decode, Debug)]
pub struct TestPacket {
    user_id: String,
    user_name: String,
    user_email: String,
}

fn main() {
    let encoded = encode(&TestPacket {
        user_id: "123".to_string(),
        user_name: "John Doe".to_string(),
        user_email: "myyrakle@naver.com".to_string(),
    });

    println!("encoded: {:?}", encoded);

    let decoded: TestPacket = decode(&encoded).unwrap();
    println!("decoded: {:?}", decoded);
}

```
