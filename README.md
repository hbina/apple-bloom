# Apple Bloom

> Rust crate for serializing and deserializing [open api](http://swagger.io/specification/) documents

Fork of [softprops/openapi](https://github.com/softprops/openapi)

## Install

add the following to your `Cargo.toml` file

```toml
[dependencies]
apple_bloom = "0.1"
```

## Usage

```rust
extern crate apple_bloom;

fn main() {
  match apple_bloom::from_path("path/to/openapi.yaml") {
    Ok(spec) => println!("spec: {:?}", spec),
    Err(err) => println!("error: {}", err)
  }
}
```
