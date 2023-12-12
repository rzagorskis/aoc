pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

pub fn calculate_lcm(numbers: &[u64]) -> u64 {
    if numbers.is_empty() {
        0
    } else {
        numbers.iter().cloned().fold(1, lcm)
    }
}