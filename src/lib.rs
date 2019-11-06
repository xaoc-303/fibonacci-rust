mod fibo;

#[cfg(test)]
mod tests {
    use fibo;

    #[test]
    fn f1_test() {
        assert_eq!(fibo::f1(4), 3);
        assert_eq!(fibo::f1(5), 5);
        assert_eq!(fibo::f1(6), 8);
    }

    #[test]
    fn f2_test() {
        assert_eq!(fibo::f2(4), 3);
        assert_eq!(fibo::f2(5), 5);
        assert_eq!(fibo::f2(6), 8);
    }
}
