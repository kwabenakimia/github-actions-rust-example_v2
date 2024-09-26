#[cfg(test)]
mod tests {
    use trust::{add, divide, multiply, subtract, factorial};
    #[test]
    fn test_add() {
        let result = add(1, 2);
        assert_eq!(result, 3);
    }
    // add a test for subtract
    #[test]
    fn test_subtract() {
        let result = subtract(1, 2);
        assert_eq!(result, -1);
    }
    // add a test for multiply
    #[test]
    fn test_multiply() {
        let result = multiply(4, 2);
        assert_eq!(result , 8);
    }
    // add a test for divide
    #[test]
    fn test_divide() {
        let result = divide(1, 2);
        assert_eq!(result, 0);
    }
    
    // add a test for factorial
    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
    }
}
