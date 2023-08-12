#[macro_use]
extern crate afl;
extern crate nom_kconfig;

use nom_kconfig::{parse_kconfig, KconfigInput};

fn main() {
    fuzz!(|data: &[u8]| {
        if let Ok(s) = std::str::from_utf8(data) {
            let _ = parse_kconfig(KconfigInput::new_extra(&s, Default::default()));
        }
    });
}