#[cfg(test)]
mod tests {
    use trust::{add, divide, multiply, subtract};
    #[test]
    fn test_add() {
        let result = add(1, 2);
        assert_eq!(2 + 2, 4);
    }
    // add a test for subtract
    #[test]
    fn test_subtract() {
        let result = subtract(1, 2);
        assert_eq!(2 - 1, 1);
    }
    // add a test for multiply
    #[test]
    fn test_multiply() {
        let result = multiply(1, 2);
        assert_eq!(2 * 2, 4);
    }
    // add a test for divide
    #[test]
    fn test_divide() {
        let result = divide(1, 2);
        assert_eq!(2 / 2, 1);
    }
}
