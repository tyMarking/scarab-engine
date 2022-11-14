pub fn normalize(vec: [f64; 2]) -> [f64; 2] {
    let mag = (vec[0] * vec[0] + vec[1] * vec[1]).sqrt();
    if mag > 0.0 {
        [vec[0] / mag, vec[1] / mag]
    } else {
        [0.0, 0.0]
    }
}
