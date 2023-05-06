use {
    num::{Integer, Zero},
    num_bigint::BigUint
};
pub fn cuadratic(n: i32, a: i32, b: i32) -> i32 {
    (n * n) + (a * n) + b
}
pub fn pow(base: &BigUint, exp: &BigUint) -> BigUint {
    if exp.is_zero() {
        return BigUint::from(1u32);
    }
    let mut baso = base.clone();
    let mut expo = exp.clone();
    let mut result = BigUint::from(1u32);
    while !expo.is_zero() {
        if expo.is_odd() {
            result *= &baso;
        }
        baso *= baso.clone();
        expo >>= 1;
    }
    result
}
pub fn power_digit_sum(n: usize, p: u32) -> usize {
    let nstr = n.to_string();
    let mut digits = Vec::new();
    for i in nstr.chars() {
        digits.push(i.to_string().parse::<usize>().unwrap());
    }
    digits.iter().map(|x| x.pow(p)).sum()
}
pub trait Num {
    fn is_prime(&self) -> bool;
}
impl<T: Integer + Clone + Copy> Num for T {
    fn is_prime(&self) -> bool  {
        if *self <= T::one() {
            return false;
        }
        let mut i = T::one() + T::one();
        while i * i <= *self {
            if *self % i == T::zero() {
                return false;
            }
            i = i + T::one();
        }
        true
    }
}
