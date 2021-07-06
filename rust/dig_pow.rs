//! Module that implements solution to Codewars problem: Playing with numbers.

pub mod problem {

    pub fn dig_pow(n: i64, p: i32) -> i64 {
        // Get a vector of the digits.
        let mut divisor = 1i64;
        let mut digits = vec![];

        while divisor < n {
            digits.push((n % (divisor * 10)) / divisor);

            divisor *= 10;
        }

        // Get the sum of the polinomial.
        let mut sum = 0i64;
        let mut p_increment = 0i32;
        for digit in digits.iter().rev() {
            sum += digit.pow((p + p_increment) as u32);

            p_increment += 1;
        }

        // Check for some k while we don't exceed n ^ k;
        let mut k = 1i64;
        let mut product = 0i64;
        while product < sum {
            product = n * k;
            if product == sum {
                return k;
            }

            k += 1;
        }

        -1i64
    }


    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn dig_pow_valid() {
            assert_eq!(dig_pow(89, 1), 1);
            assert_eq!(dig_pow(46288, 3), 51);
        }
    }
}

use crate::problem::dig_pow;

fn main() {
    println!("Digits power: {}", dig_pow(89, 1));
}

