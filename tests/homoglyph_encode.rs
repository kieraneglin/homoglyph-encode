extern crate homoglyph_encode;

use homoglyph_encode::{encode, decode};

// CHARSET = "349ACEFHJKLMNPRTUVWXYabefghkpqstuxyz";
// reference "0123456789abcdefghijklmnopqrstuvwxyz"

#[test]
fn encodes_under_36() {
    assert_eq!(encode(0), "3");
    assert_eq!(encode(10), "L");
    assert_eq!(encode(35), "z");
}

#[test]
fn encodes_over_36() {
    assert_eq!(encode(36), "43");
    assert_eq!(encode(72), "93");
    assert_eq!(encode(1234567890), "YT49fW");
}

#[test]
fn decodes_under_36() {
    assert_eq!(decode("3"), 0);
    assert_eq!(decode("L"), 10);
    assert_eq!(decode("z"), 35);
}

#[test]
fn decodes_over_36() {
    assert_eq!(decode("43"), 36);
    assert_eq!(decode("93"), 72);
    assert_eq!(decode("YT49fW"), 1234567890);
}

#[test]
#[should_panic]
fn doesnt_decode_invalid() {
    decode("B");
}

#[test]
fn encode_is_reversible() {
    let num = 10;

    assert_eq!(decode(&encode(num)), num);
}
