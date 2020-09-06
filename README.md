# driftwood
## some logs on the [tide](https://github.com/http-rs/tide)

* [CI ![CI][ci-badge]][ci]
* [API Docs][docs] [![docs.rs docs][docs-badge]][docs]
* [Releases][releases] [![crates.io version][version-badge]][lib-rs]
* [Contributing][contributing]

[ci]: https://github.com/jbr/driftwood/actions?query=workflow%3ACI
[ci-badge]: https://github.com/jbr/driftwood/workflows/CI/badge.svg
[releases]: https://github.com/jbr/driftwood/releases
[docs]: https://docs.rs/driftwood
[contributing]: https://github.com/jbr/driftwood/blob/master/.github/CONTRIBUTING.md
[lib-rs]: https://lib.rs/driftwood
[docs-badge]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[version-badge]: https://img.shields.io/crates/v/driftwood.svg?style=flat-square

## Installation
```sh
$ cargo add driftwood
```

## Usage
This crate currently provides three loggers:

### Apache common log format
```rust
let mut app = tide::new();
app.with(driftwood::ApacheCommonLogger);
```
Example:
`127.0.0.1 - frank [10/Oct/2000:13:55:36 -0700] "GET /apache_pb.gif HTTP/1.0" 200 2326`

### Apache combined log format
```rust
let mut app = tide::new();
app.with(driftwood::ApacheCombinedLogger);
```
Example:
`127.0.0.1 - frank [10/Oct/2000:13:55:36 -0700] "GET /apache_pb.gif HTTP/1.0" 200 2326 "http://www.example.com/start.html" "Mozilla/4.08 [en] (Win98; I ;Nav)"`

### Dev logger
```rust
let mut app = tide::new();
app.with(driftwood::DevLogger); // or ApacheCombinedLogger or ApacheCommonLogger
```

This logger colors the status code based on the status range and is
intended to be read by people

Example:
`GET http://localhost:8080/some/path 200 3.289292ms 227bytes`

# Intent
Eventually, this crate intends to support much of the functionality in
[https://www.npmjs.com/package/morgan](morgan) and may eventually
support output other than stdout.

## Safety
This crate uses ``#![deny(unsafe_code)]`` to ensure everything is implemented in
100% Safe Rust.

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br/>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
