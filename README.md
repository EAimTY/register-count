# register-count

Counting `Register`s created by a `Counter`

[![Version](https://img.shields.io/crates/v/register-count.svg?style=flat)](https://crates.io/crates/register-count)
[![Documentation](https://img.shields.io/badge/docs-release-brightgreen.svg?style=flat)](https://docs.rs/register-count)
[![License](https://img.shields.io/crates/l/register-count.svg?style=flat)](https://github.com/EAimTY/register-count/blob/master/LICENSE)

## Usage

This crate helps you to count the number of currently un-dropped `Register`s created by a `Counter`.

```rust
use register_count::Counter;

let cnt = Counter::new();
println!("Number of registers: {}", cnt.count()); // 0
let reg1 = cnt.reg();
println!("Number of registers: {}", cnt.count()); // 1
let reg2 = reg1.clone();
println!("Number of registers: {}", cnt.count()); // 2
drop(reg1);
println!("Number of registers: {}", cnt.count()); // 1
```

## `no_std`

Opt-out of the default features to use this crate in `no_std` environments.

## License

MIT License
