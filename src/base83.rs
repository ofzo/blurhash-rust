static CHARACTERS: &str =
    "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz#$%*+,-.:;=?@[]^_{|}~";

pub fn encode83(n: u32, len: u32) -> String {
    let mut result = format!("");
    for i in 1..len + 1 {
        let digit = (n as u64 / 83u64.pow(len - i)) % 83;
        result += CHARACTERS
            .chars()
            .nth(digit as usize)
            .unwrap()
            .to_string()
            .as_str();
    }
    result
}

pub fn decode83(chars: &str) -> f64 {
    let mut v = 0;
    for c in chars.chars() {
        let index = CHARACTERS.chars().position(|x| x == c).unwrap();
        v = v * 83 + index;
    }
    return v as f64;
}
