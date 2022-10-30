pub fn rgb_to_linear(v: f64) -> f64 {
    let v = v / 255.0;
    if v <= 0.04045 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}

pub fn linear_to_rgb(v: f64) -> f64 {
    let mut v = v.min(1.0).max(0.0);
    if v <= 0.0031308 {
        v = v * 12.92 * 255.0 + 0.5;
    } else {
        v = (v.powf(1.0 / 2.4) * 1.055 - 0.055) * 255.0 + 0.5;
    }
    return v.floor();
}

#[inline]
pub fn sign_pow(n: f64, exp: f64) -> f64 {
    n.abs().powf(exp) * if n < 0.0 { -1.0 } else { 1.0 }
}
