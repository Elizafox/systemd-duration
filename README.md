# systemd-duration
`systemd-duration` is a crate that implements parsing of the [systemd duration format] in Rust.

This library can convert a systemd duration string to the following:
* [std::time::Duration]
* [time::Duration] \(available with the `with-time` feature\)
* [chrono::TimeDelta] \(available with the `with-chrono` feature\)

## Usage
See the examples directory for code examples.

[systemd duration format]: https://www.freedesktop.org/software/systemd/man/latest/systemd.time.html
[std::time::Duration]: https://doc.rust-lang.org/std/time/struct.Duration.html
[time::Duration]: https://docs.rs/time/latest/time/struct.Duration.html
[chrono::TimeDelta]: https://docs.rs/chrono/latest/chrono/struct.TimeDelta.html
