# cargo eval

A cargo plugin to quickly evaluate some Rust source code.

## Installation

```console
$ cargo install cargo-eval
```

## Usage

Provide Rust code to be compiled and executed after `--`:

```
$ cargo eval -- 1 + 1
> 1 + 1
2
```

Dependencies can be specified with the `-d` (`--dep`) arguments:

```
$ cargo eval -d fastrand -- 'if fastrand::bool() {
    123 
} else {
    456
}'
> if fastrand::bool() {
    123 
} else {
    456
}
456
```

Add the `-v`/`--verbose` flag to inspect the files from the intermediate crate that is created behind-the-scenes:

```
$ cargo eval -v -d fastrand -- 'if fastrand::bool() {
    123 
} else {
    456
}'
Cargo.toml
[package]
name = "temp"
version = "0.0.0"
edition = "2021"

[dependencies]
fastrand = "*"

src/main.rs
fn main() -> Result<(), ()> {
    let input = if fastrand::bool() { 123 } else { 456 };
    println!("{}", input);
    Ok(())
}

> if fastrand::bool() {
    123 
} else {
    456
}
123
```
