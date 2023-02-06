//use std::f64::consts::PI;

const C: f64 = 2.998e+8;

pub fn time_dilation_calc(distance: f64, speed: f64) -> (f64, f64) {
    let speed = speed * C;
    let distance = distance * 9.461e+15;
    let proper_time_sec = distance / speed;
    let dilated_time_sec = proper_time_sec / (1.0 - (speed / C).powi(2)).sqrt();
    let dilated_time_year = dilated_time_sec / (60.0 * 60.0 * 24.0 * 365.0);
    let proper_time_year = proper_time_sec / (60.0 * 60.0 * 24.0 * 365.0);

    (proper_time_year, dilated_time_year)
}


