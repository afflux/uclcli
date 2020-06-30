#![no_main]
use std::convert::TryInto;
use libfuzzer_sys::fuzz_target;

use uclcli::{ucl_init, decompress};

fuzz_target!(|data: &[u8]| {
    ucl_init();

    let _result = decompress(data, (data.len() * 1024).try_into().unwrap());
});
