pub fn fibonacci_1(n: u64) -> u64 {
    match n {
        0 | 1 => n,
        _ => fibonacci_1(n - 1) + fibonacci_1(n - 2),
    }
}

pub fn fibonacci_2(n: u64) -> u64 {
    fn fibonacci_inner(a: u64, b: u64, n: u64) -> u64 {
        match n {
            0 => a,
            _ => fibonacci_inner(b, a + b, n - 1),
        }
    }

    fibonacci_inner(0, 1, n)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sequence() {
        assert!(fibonacci_1(0) == 0);
        assert!(fibonacci_1(1) == 1);
        assert!(fibonacci_1(2) == 1);
        assert!(fibonacci_1(3) == 2);
        assert!(fibonacci_1(4) == 3);
        assert!(fibonacci_1(15) == 610);
    }
}
