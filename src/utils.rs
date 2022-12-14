use std::f64;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn degree_to_radian(degree: f64) -> f64 {
    (2.0 * f64::consts::PI / 360.0) * degree
}

pub fn round_radian(radian: f64) -> f64 {
    radian % (2.0 * f64::consts::PI)
}
