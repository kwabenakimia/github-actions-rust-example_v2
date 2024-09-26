// add function that adds two numbers
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// add a function that subtracts two numbers
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

// add a function that multiplies two numbers
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

// add a function that divides two numbers
pub fn divide(a: i32, b: i32) -> i32 {
    a / b
}

// add a function that calculates the power of a number
pub fn power(base: i32, exponent: u32) -> i32 {
    base.pow(exponent)
}

// add a function that calculates the modulus of two numbers
pub fn modulus(a: i32, b: i32) -> i32 {
    a % b
}

// add a function that calculates the square root of a number
pub fn sqrt(x: f64) -> f64 {
    x.sqrt()
}

// add a function that calculates the factorial of a number
pub fn factorial(n: u64) -> u64 {
    if n == 0 || n == 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}
