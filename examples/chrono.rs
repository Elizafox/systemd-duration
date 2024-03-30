use systemd_duration::chrono::parse;
use chrono::TimeDelta;

fn main() {
    let td = parse("3d").expect("Failed to parse duration");
    assert_eq!(td, TimeDelta::days(3));
}
