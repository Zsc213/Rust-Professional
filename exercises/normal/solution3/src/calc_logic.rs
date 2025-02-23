pub fn new_birthday_probability(n: u32) -> f64 {
    // TODO: 这里写逻辑
    let year: f64 = 365.0;
    let mut res: f64 = 1.0;
    for i in 0..n {
        res *= (year - i as f64) / year;
    }
    1.0 as f64 - res
}
