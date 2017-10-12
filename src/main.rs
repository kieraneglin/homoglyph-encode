const CHARSET: &str = "349ACEFHJKLMNPRTUVWXYabefghkpqstuxyz";
// Not included:
// B8G6I1l0OoQDS5Z72ij (obvious)
// rn -> m,vv -> w, cl -> d

fn main() {
    // println!("{:?}", "test".char_at(0));
    // encode(0);
    let test = encode(43324234234234);
    println!("{:?}", test);

    let t2 = decode(&test);
    println!("{:?}", t2);
    // println!("{:?}", usize::from_str_radix(&test, 36));
}

fn encode(num: usize) -> String {
    let encoded: Vec<char> = base36encode(num).chars().collect();
    let allowed_chars: Vec<char> = CHARSET.chars().collect();
    let mut result: String = "".to_owned();

    for ch in encoded {
        let index = usize::from_str_radix(&ch.to_string(), 36).unwrap();
        result += &allowed_chars[index].to_string()
    }

    result
}

fn decode(string: &str) -> usize {
    println!("{:?}", CHARSET.find("a"));
    2
}

fn base36encode(mut num: usize) -> String {
    if num == 0 {
        return 0.to_string();
    }

    let mut encoded: String = "".to_owned();
    let mut remainder: usize;
    let chars: Vec<char> = "0123456789abcdefghijklmnopqrstuvwxyz".chars().collect();

    while num > 0 {
        remainder = num % chars.len();
        num = num / chars.len();
        encoded = chars[remainder].to_string() + &encoded;
    }

    encoded
}

// def base36encode(integer):
//     chars, encoded = '0123456789abcdefghijklmnopqrstuvwxyz', ''
//
//     while integer > 0:
//         integer, remainder = divmod(integer, 36)
//         encoded = chars[remainder] + encoded
//
//     return encoded
