use {
    crate::num_tools::{self, Num},
    num_bigint::BigUint
};
/* Problem 27 */
/*
    Euler discovered the remarkable quadratic formula: n^2 + n + 41
    It turns out that the formula will produce 40 primes for the consecutive integer values 0 ≤ n ≤ 39.
    However, when n = 40, 40^2 + 40 + 41 = 40(40 + 1) + 41 is divisible by 41, and certainly when n = 41, 
    41^2 + 41 + 41 is clearly divisible by 41.

    The incredible formula n^2 - 79n + 1601 was discovered, which produces 80 primes for the consecutive
    values 0 <= n <= 79. The product of the coefficients, −79 and 1601, is −126479.

    Considering quadratics of the form: 
        n^2 + an + b, where |a| < 1000 and |b| ≤ 1000

        where |n| is the modulus/absolute value of
        e.g. |11| = 11 and |-4| = 4

    Find the product of the coefficients, a and b, for the quadratic expression that produces the maximum
    number of primes for consecutive values of n, starting with n = 0.
*/
pub fn problem_27() -> isize {
    const ABUELO: i32 = 1000;
    let mut max_count = 0;
    let mut max_product = 0;
    for a in -(ABUELO - 1)..ABUELO {
        for b in -(ABUELO - 1)..ABUELO {
            let mut n = 0;
            while num_tools::cuadratic(n, a, b).is_prime() {
                n += 1;
            }
            if max_count < n {
                max_count = n;
                max_product = a * b;
            }
        }
    }
    max_product as isize
}
/* Problem 28 */
/*
    Starting with the number 1 and moving to the right in a clockwise direction a 5 by 5 spiral is 
    formed as follows:

    21 22 23 24 25
    20  7  8  9 10
    19  6  1  2 11
    18  5  4  3 12
    17 16 15 14 13

    It can be verified that the sum of the numbers on the diagonals is 101.

    What is the sum of the numbers on the diagonals in a 1001 by 1001 spiral formed in the same way?
*/
pub fn problem_28() -> isize {
    const ABUELO: i32 = 1001;
    let mut sum = 1;
    for i in (3..=ABUELO).step_by(2) {
        let sq = i * i;
        sum += sq + (sq - (i - 1)) + (sq - (i - 1) * 2) + (sq - (i - 1) * 3);
    }
    sum as isize
}
/* Prolem 29 */
/*
    Consider all integer combinations of a^b for 2 ≤ a ≤ 5 and 2 ≤ b ≤ 5:

        2^2=4, 2^3=8, 2^4=16, 2^5=32
        3^2=9, 3^3=27, 3^4=81, 3^5=243
        4^2=16, 4^3=64, 4^4=256, 4^5=1024
        5^2=25, 5^3=125, 5^4=625, 5^5=3125

    If they are then placed in numerical order, with any repeats removed, we get the following sequence of 15 distinct terms:

    4, 8, 9, 16, 25, 27, 32, 64, 81, 125, 243, 256, 625, 1024, 3125

    How many distinct terms are in the sequence generated by a^b for 2 ≤ a ≤ 100 and 2 ≤ b ≤ 100?

*/

pub fn problem_29() -> isize {
    const ABUELO: u128 = 100;
    let mut vic = Vec::new();
    let mut clones = Vec::new();
    for a in 2..=ABUELO {
        for b in 2..=ABUELO {
            let ab = num_tools::pow(&BigUint::from(a), &BigUint::from(b));
            if vic.contains(&ab.clone()) {
                clones.push((a, b, ab.clone()));
            }
            vic.push(ab);
        }
    }
    (vic.len() - clones.len()) as isize
}
/* PROBLEM 30 */
/*
    Surprisingly there are only three numbers that can be written as the sum of fourth powers of their digits:

        1634 = 14 + 64 + 34 + 44
        8208 = 84 + 24 + 04 + 84
        9474 = 94 + 44 + 74 + 44

    As 1 = 14 is not a sum it is not included.

    The sum of these numbers is 1634 + 8208 + 9474 = 19316.

    Find the sum of all the numbers that can be written as the sum of fifth powers of their digits.
 */
pub fn problem_30() -> isize {
    const ABUELO: u32 = 5;
    let mut sum = 0;
    for i in 2..=200000 {
        if num_tools::power_digit_sum(i, ABUELO) == i {
            sum += i;
        }
    }
    sum as isize
}