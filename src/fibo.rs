pub fn f1(n: u128) -> (u128) {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    return f1(n-1) + f1(n-2);
}

pub fn f2(n: u128) -> (u128) {
    let mut a = 0;
    let mut b = 1;

    for _i in 2..=n {
        let next = a + b;
        a = b;
        b = next;
    }

    return b;
}
