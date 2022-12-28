#![no_main]

use libfuzzer_sys::fuzz_target;

use powierza_coefficient::powierża_coefficient as powierża;

fuzz_target!(|params: (&str, &str)| {
    let (pattern, text) = params;

    let _ = powierża(pattern, text);
});
