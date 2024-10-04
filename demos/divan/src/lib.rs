//

pub fn fib_rec(n: usize) -> usize {
    if n < 2 {
        return n;
    }
    fib_rec(n - 2) + fib_rec(n - 1)
}

pub fn fib_iter(n: usize) -> usize {
    if n == 0 {
        return 0;
    }
    let (mut a, mut b) = (0, 1);
    for _ in 1..n {
        (a, b) = (b, a + b);
    }
    b
}

#[cfg(test)]
mod tests {
    #[test]
    fn fib_rec() {
        run_cases(super::fib_rec)
    }

    #[test]
    fn fib_iter() {
        run_cases(super::fib_iter)
    }

    fn run_cases(fib: fn(usize) -> usize) {
        assert_eq!(fib(0), 0);
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
        assert_eq!(fib(3), 2);
        assert_eq!(fib(4), 3);
        assert_eq!(fib(5), 5);
        assert_eq!(fib(6), 8);
        assert_eq!(fib(7), 13);
        assert_eq!(fib(8), 21);
        assert_eq!(fib(9), 34);
        assert_eq!(fib(10), 55);
        assert_eq!(fib(11), 89);
        assert_eq!(fib(12), 144);
        assert_eq!(fib(13), 233);
        assert_eq!(fib(14), 377);
        assert_eq!(fib(15), 610);
        assert_eq!(fib(16), 987);
        assert_eq!(fib(17), 1597);
        assert_eq!(fib(18), 2584);
        assert_eq!(fib(19), 4181);
    }
}
