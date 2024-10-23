fn add(n1: f64, n2: f64) -> f64 {
    n1 + n2
}

fn subtract(n1: f64, n2: f64) -> f64 {
    n1 - n2
}

fn multiply(n1: f64, n2: f64) -> f64 {
    n1 * n2
}

fn divide(n1: f64, n2: f64) -> Result<f64, &'static str> {
    if n2 == 0.0 {
        return Err("Cannot divide by zero");
    }

    Ok(n1 / n2)
}

fn odd_number(num: u32) -> bool {
    num % 2 != 0
}

fn factorial(num: u32) -> u32 {
    if num == 0 || num == 1 {
        return 1
    } else {
        num * factorial(num - 1)
    }
}

fn panic_function() {
    panic!("Panic function invoked");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2.0, 3.0), 5.0);
        assert_ne!(add(2.0, 3.0), 6.0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(5.0, 3.0), 2.0);
        assert_eq!(subtract(3.0, 5.0), -2.0);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(2.0, 3.0), 6.0);
        assert_eq!(multiply(-2.0, 3.0), -6.0);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(6.0, 3.0), Ok(2.0));
        assert_eq!(divide(6.0, 0.0), Err("Cannot divide by zero"));
    }

    #[test]
    fn test_odd_number() {
        assert!(odd_number(3));
        assert!(!odd_number(4));
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(6), 720);
    }

    #[test]
    #[should_panic]
    fn test_panic_function() {
        panic_function();
    }
}