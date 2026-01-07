#![no_main]
use endpointo::parser::Parser;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let parser = Parser::new();
        let _ = parser.parse_js(s, None);
    }
});
