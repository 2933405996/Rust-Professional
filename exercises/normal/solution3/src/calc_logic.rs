pub fn new_birthday_probability(n: u32) -> f64 {
    // TODO: 这里写逻辑
    if n < 2 || n > 365 {
        return 0.0;
    }
    let mut probability = 1.0;
    for i in 0..n {
        probability *= (365.0 - i as f64) / 365.0;
    }
    1.0 - probability
}
