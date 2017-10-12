const CHARSET: &str = "349ACEFHJKLMNPRTUVWXYabefghkpqstuxyz";
// Not included:
// B8G6I1l0OoQDS5Z72ij (obvious)
// rn -> m,vv -> w, cl -> d

pub fn encode(mut num: usize) -> String {
    let chars: Vec<char> = CHARSET.chars().collect();
    let base = CHARSET.len();

    if num == 0 {
        return chars[0].to_string();
    }

    let mut encoded: String = "".to_owned();
    let mut remainder: usize;

    while num > 0 {
        remainder = num % base;
        num /= base;
        encoded = chars[remainder].to_string() + &encoded;
    }

    encoded
}

pub fn decode(string: &str) -> usize {
    let chars: Vec<char> = string.chars().collect();
    let base = CHARSET.len();

    let mut accumulator = 0;

    for ch in chars {
        accumulator *= base;
        accumulator += CHARSET.find(ch).expect("Invalid character in input string");
    }

    accumulator
}
