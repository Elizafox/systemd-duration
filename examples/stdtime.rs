use systemd_duration::stdtime::parse;
use std::time::Duration;

fn main() {
    let td = parse("3d").expect("Failed to parse duration");
    assert_eq!(td, Duration::from_secs(259200));
}
