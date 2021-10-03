# rsvart
A re-implementation of [svart](https://github.com/exomiser/svart) in rust. Svart is a small library for representing genomic variants and regions. It attempts to solve several common issues:

- Coordinate system off-by-one errors
- Representation of VCF small, structural and breakend variants with a consistent API
- Different variant trimming strategies
- The library provides a consistent API for creating, manipulating and comparing variation of different types by providing default 
- implementations and extensible base classes and interfaces along with immutable default implementations which developers can utilise to gain maximum utility from a minimum amount of code without having to address issues in bioinformatics which are a common source of duplicated code and frequently errors.

The code is completely free of external dependencies.

## Motivation
This is intended to be used as a standard library for bioinformatics in rust. Its motivation is a way for us to become rustaceans.


## Development

Testing
```
cargo test
```
Building
```
cargo build
```


