/// Greatest Common Divisor
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Least Common Multiplier
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

/// Shoelace formula
/// points = vector of (x,y) tuple points
pub fn shoelace(points: &Vec<(f64, f64)>) -> f64 {
    let n = points.len();
    let mut sum1 = 0.0;
    let mut sum2 = 0.0;

    for i in 0..n {
        let next_i = (i + 1) % n;
        sum1 += points[i].0 * points[next_i].1;
        sum2 += points[i].1 * points[next_i].0;
    }

    return 0.5 * (sum1 - sum2).abs();
}

/// Picks theorem to find out how many inner points exist within a point boundary
/// i = A - b/2 - h + 1
///
/// area = total area of defined boundary
/// boundary_point_count = total points within the boundary
/// h = holes within boundary (a donut shape will always have a hole size of 1)
pub fn picks_theorem_inner_points(area: f64, boundary_point_count: usize, h: usize) -> f64 {
    return area - (boundary_point_count / 2) as f64 - h as f64 + 1.0;
}

/// Picks theoream for area
/// A = i + b/2 + h − 1 = 8
pub fn picks_theorem_area(inner_points: usize, boundary_point_count: usize, h: usize) -> f64 {
    return inner_points as f64 + (boundary_point_count / 2) as f64 + h as f64 - 1.0;
}
