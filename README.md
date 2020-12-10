# Rust library for german umlauts

## Usage

```rust
# Cargo.toml

# Safe without manipulating &mut str
[dependencies]
umlauts = "0.1"

# Unsafe conversions with the ability to manipulate &mut str
umlauts = { version = "0.1", features = [ "unsafe" ] }
```

## Examples

```rust
extern crate umlauts;

use umlauts::UmlautsOwned;

let mut s = "Öl Ärmel Übermut".to_string();
s.make_utf8_umlauts_lowercase();
assert_eq!("öl ärmel übermut", s);
```

## Unsafe Features

### `&mut str` manipulation

By default, this crate itself does not use unsafe features,
except from the [memchr] dependency.
To allow manipulating of references to `String` and `&mut str`,
the `unsafe` feature can be used.
As the manipulating functions operate on `&mut [u8]` and will always
return valid UTF-8 if valid UTF-8 is given,
those unsafe features should perform the same.

[memchr]: https://github.com/BurntSushi/rust-memchr
