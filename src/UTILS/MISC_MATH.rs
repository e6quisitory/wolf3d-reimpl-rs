
/*********************************** MISC_MATH ***********************************/

pub fn GetDecimal(f: f64) -> f64 {
    let f_abs = f.abs();
    return f_abs - f_abs.floor();
}

pub fn IsInteger(f: f64) -> bool {
    if GetDecimal(f) == 0.0 {
        true
    } else {
        false
    }
}

pub fn DegreesToRadians(degrees: f64) -> f64 {
    degrees*super::CONVENTIONS::PI/180.0
}