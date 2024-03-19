# is-digit
Detect decimal digit in char or first char of the str and String.

## Install
Specify the dependencty in Cargo.toml:

```yaml
[dependencies]
is-digit = "~0.1.1"

Fetch it with cargo:
```bash
$ cargo build
```

## Usage

```rust
extern crate is_digit;
use is_digit::IsDigit;

let _i = '1';
println!("{}", _i.is_dec_digit()); // prints true
let _j = "2";
println!("{}", _j.is_dec_digit()); // prints true
let _k = String::from("3");
println!("{}", _k.is_dec_digit()); // prints true
```

## About
### License
Copyright © 2024, [255doesnotexist](https://github.com/255doesnotexist).
Released under the [MIT License](LICENSE).