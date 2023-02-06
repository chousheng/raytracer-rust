use rand::Rng;

pub fn random_double() -> f64 {
    rand::thread_rng().gen::<f64>()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    x
}
