// SPDX-License-Identifier: CC0-1.0
//
// This file is part of systemd-duration.
//
// To the extent possible under law, the author(s) have dedicated all copyright
// and related and neighboring rights to this software to the public domain
// worldwide. This software is distributed without any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication along
// with this software. If not, see <https://creativecommons.org/publicdomain/zero/1.0/>.

use std::time::Duration;
use systemd_duration::stdtime::parse;

fn main() {
    let td = parse("3d").expect("Failed to parse duration");
    assert_eq!(td, Duration::from_secs(259200));
}
