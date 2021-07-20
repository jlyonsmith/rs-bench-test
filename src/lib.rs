// Boring old Fibonacci.
pub fn fibonacci_1(n: u64) -> u64 {
    match n {
        0 | 1 => n,
        _ => fibonacci_1(n - 1) + fibonacci_1(n - 2),
    }
}

// But, this version will be optimize by the compiler with tail recursion.
// Here's an SO answer that explains what is happening, and why even
// though it's cool, it's not something that you should generally rely
// on for optimizing your code. https://stackoverflow.com/a/59418785/576235
// Useful for demonstrating micro benchmarking though!
pub fn fibonacci_2(n: u64) -> u64 {
    fn fibonacci_inner(a: u64, b: u64, n: u64) -> u64 {
        match n {
            0 => a,
            _ => fibonacci_inner(b, a + b, n - 1),
        }
    }

    fibonacci_inner(0, 1, n)
}

// fibonacci_2 is essentially compiled to this
pub fn fibonacci_3(mut n: u64) -> u64 {
    let (mut a, mut b) = (0, 1);

    while n > 0 {
        n -= 1;
        a = a + b;
        b = a - b; // Yes, b now equals a's previous value
    }
    a
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fib_1() {
        assert!(fibonacci_1(0) == 0);
        assert!(fibonacci_1(1) == 1);
        assert!(fibonacci_1(2) == 1);
        assert!(fibonacci_1(3) == 2);
        assert!(fibonacci_1(4) == 3);
        assert!(fibonacci_1(15) == 610);
    }

    #[test]
    fn test_fib_2() {
        assert!(fibonacci_2(0) == 0);
        assert!(fibonacci_2(1) == 1);
        assert!(fibonacci_2(2) == 1);
        assert!(fibonacci_2(3) == 2);
        assert!(fibonacci_2(4) == 3);
        assert!(fibonacci_2(15) == 610);
    }

    #[test]
    fn test_fib_3() {
        assert!(fibonacci_3(0) == 0);
        assert!(fibonacci_3(1) == 1);
        assert!(fibonacci_3(2) == 1);
        assert!(fibonacci_3(3) == 2);
        assert!(fibonacci_3(4) == 3);
        assert!(fibonacci_3(15) == 610);
    }
}
