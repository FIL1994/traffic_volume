use std::num::{ParseFloatError, ParseIntError};

pub fn to_i32(string: String) -> Result<i32, ParseIntError> {
    Ok(string.parse::<i32>())?
}

pub fn to_f64(string: String) -> Result<f64, ParseFloatError> {
    let mut parseable_string = string.clone();

    match parseable_string.find(".") {
        Some(_v) => (),
        None => parseable_string.push_str(".0"),
    };

    Ok(parseable_string.parse::<f64>())?
}
