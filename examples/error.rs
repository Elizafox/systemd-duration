use systemd_duration::{error, stdtime::parse};
use std::time::Duration;

fn fail_parse() -> Result<Duration, error::Error> {
    parse("3x")?;
    unreachable!();
}

fn main() {
    let td = fail_parse();
    assert!(td.is_err());
}
