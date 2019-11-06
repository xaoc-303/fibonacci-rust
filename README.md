# fibonacci-rust

[![Build Status](https://travis-ci.org/xaoc-303/fibonacci-rust.svg?branch=master)](https://travis-ci.org/xaoc-303/fibonacci-rust)

## recursive if-else

| v | # | 30 | 35 | 40 | 45 |
| --- | --- | --- | --- | --- | --- |
| 1.38.0 | [Rust](./fibo.rs) (compiled) | 0.01515268 | 0.13358528 | 1.34242942 | 14.76075383 |
| | [Total](https://github.com/xaoc-303/fibonacci) | | | | |

## optimization

| v | # | 30 | 35 | 40 | 45 |
| --- | --- | --- | --- | --- | --- |
| 1.38.0 | [Rust](./fibo.rs) (compiled) | 0.00000255 |  0.00000261 | 0.00000303 | 0.00000330 |
| | [Total](https://github.com/xaoc-303/fibonacci) | | | | |

## run

```
rustc src/main.rs
time ./main f1 30
time ./main f2 30
```

```
cargo build
cargo test
```
