#![no_main]
use libfuzzer_sys::fuzz_target;

use uclcli::{ucl_init, compress};

fuzz_target!(|data: &[u8]| {
    ucl_init();

    let _result = compress(data);
});
