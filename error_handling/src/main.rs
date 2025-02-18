use std::fmt::{Display, Formatter};

#[derive(Debug)]
enum DivError {
    DivisionByZero,
    DivisionByNegative(i32),
}

impl Display for DivError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", match self {
            DivError::DivisionByZero => "Division by zero".to_string(),
            DivError::DivisionByNegative(b) => format!("Division by negative: {}", b),
        })
    }
}

fn div(a: i32, b: i32) -> Result<i32, DivError> {
    if b == 0 {
        return Err(DivError::DivisionByZero);
    }

    if b < 0 {
        return Err(DivError::DivisionByNegative(b));
    }

    Ok(a / b)
}

fn div_the_divs(a: i32, b: i32, c: i32, d: i32) -> Result<i32, DivError> {
    // (a / b) / (c / d)
    div(div(a, b)?, div(c, d)?)
}

fn main() {
    let result = div(10, 2);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(e) => println!("Error: {e}"),
    }
}
