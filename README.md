# Rust library for german umlauts

## Examples

```rust
extern crate umlauts;

use umlauts::UmlautsOwned;

let mut s = "Öl Ärmel Übermut".to_string();
s.make_utf8_umlauts_lowercase();
assert_eq!("öl ärmel übermut", s);
```
