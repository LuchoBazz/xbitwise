# xbitwise

A Rust library that extends the basic functionality of bitwise operations

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
xbitwise = "0.1"
```
Or you can add for the different features

```toml
[dependencies]
xbitwise = { version = "0.1", features = ["i32", "u32"] }
```

`default = ["signed", "unsigned"]`

`signed = ["i8", "i16", "i32", "i64", "i128"]`

`unsigned = ["u8", "u16", "u32", "u64", "u128"]`

`i8`, `i16`, `i32`, `i64`, `i128`, `u8`, `u16`, `u32`, `u64`, `u128`

*Version requirement: xbitwise supports rustc 1.31 and up.*

## Bug reports

You can report any bugs [here](https://github.com/LuisMBaezCo/xbitwise/issues).

# License

xbitwise is distributed under the terms of both the MIT license.

See [LICENSE-MIT](LICENSE-MIT)