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
/// A = i + b/2 + h − 1
pub fn picks_theorem_area(inner_points: usize, boundary_point_count: usize, h: usize) -> f64 {
    return inner_points as f64 + (boundary_point_count / 2) as f64 + h as f64 - 1.0;
}

/// Calculats how many distinct tuples exist between the amount of objects and the sample size (the amount of distinct numbers in a tuple)
/// e.g. For 9 distinct numbers, where a unique tuple has 2 distinct numbers, there are a total of 36 distinct pairs
/// 
/// C(n,r) = n! / (r! * (n−r)!)
pub fn combination_formula(objects: usize, sample: usize) -> Option<usize> {
    if let (Some(objects_factorial), Some(sample_factorial), Some(diff_factorial)) = (
        factorial(objects),
        factorial(sample),
        factorial(objects - sample),
    ) {
        return Some(objects_factorial / (sample_factorial * diff_factorial));
    }

    return None;
}

pub fn factorial(n: usize) -> Option<usize> {
    (1..n).try_fold(n, usize::checked_mul)
}

/// Calculates the distance on a 2D plane
/// 
/// https://simple.wikipedia.org/wiki/Manhattan_distance
pub fn manhattan_distance(x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
    (x2 - x1).abs() + (y2 - y1).abs()
}