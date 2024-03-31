use chrono::TimeDelta;
use systemd_duration::chrono::parse;

fn main() {
    let td = parse("3d").expect("Failed to parse duration");
    assert_eq!(td, TimeDelta::days(3));
}
