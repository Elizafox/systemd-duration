use systemd_duration::time::parse;
use time::Duration;

fn main() {
    let td = parse("3d").expect("Failed to parse duration");
    assert_eq!(td, Duration::days(3));
}
