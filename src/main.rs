#[allow(dead_code)]
#[allow(unused_variables)]

const CHARSET: &str = "349ACEFHJKLMNPRTUVWXYabefghkpqstuxyz";
// REFERENCE          "0123456789abcdefghijklmnopqrstuvwxyz"

// Not included:
// B8G6I1l0OoQDS5Z72ij (obvious)
// rn -> m,vv -> w, cl -> d

fn main() {
    // println!("{:?}", "test".char_at(0));
    // encode(0);
    let test = encode(73);
    println!("{:?}", test);

    let t2 = decode(&test);
    // println!("{:?}", t2);
    // println!("{:?}", usize::from_str_radix(&test, 36));
}

fn encode(mut num: usize) -> String {
    let chars: Vec<char> = CHARSET.chars().collect();

    if num == 0 {
        return chars[0].to_string();
    }

    let mut encoded: String = "".to_owned();
    let mut remainder: usize;

    while num > 0 {
        remainder = num % chars.len();
        num = num / chars.len();
        encoded = chars[remainder].to_string() + &encoded;
    }

    encoded
}

fn decode(string: &str) -> usize {
    let chars: Vec<char> = string.chars().collect();
    let base = 10;

    for ch in chars {
        println!("{:?}", CHARSET.find(ch));
    }

    // println!("{:?}", CHARSET.find("a"));
    2
}

// fn base36encode(mut num: usize) -> String {
//     if num == 0 {
//         return 0.to_string();
//     }
//
//     let mut encoded: String = "".to_owned();
//     let mut remainder: usize;
//     let chars: Vec<char> = "0123456789abcdefghijklmnopqrstuvwxyz".chars().collect();
//
//     while num > 0 {
//         remainder = num % chars.len();
//         num = num / chars.len();
//         encoded = chars[remainder].to_string() + &encoded;
//     }
//
//     encoded
// }
// fn base36encode(mut num: usize) -> String {
//     if num == 0 {
//         return 0.to_string();
//     }
//
//     let mut encoded: String = "".to_owned();
//     let mut remainder: usize;
//     let chars: Vec<char> = "0123456789abcdefghijklmnopqrstuvwxyz".chars().collect();
//
//     while num > 0 {
//         remainder = num % chars.len();
//         num = num / chars.len();
//         encoded = chars[remainder].to_string() + &encoded;
//     }
//
//     encoded
// }


// def base36encode(integer):
//     chars, encoded = '0123456789abcdefghijklmnopqrstuvwxyz', ''
//
//     while integer > 0:
//         integer, remainder = divmod(integer, 36)
//         encoded = chars[remainder] + encoded
//
//     return encoded
