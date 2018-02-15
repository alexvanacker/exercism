pub fn raindrops(n: usize) -> String {
    let mut result = String::from("");
    if n.overflowing_rem(3) == (0, false) {
        result.push_str("Pling");
    }
    if n.overflowing_rem(5) == (0, false) {
        result.push_str("Plang");
    }
    if n.overflowing_rem(7) == (0, false) {
        result.push_str("Plong");
    }
    if result.len() > 0 {
        return result;
    }
    n.to_string()
}

