//! Module that implements solution to this problem: IQ Test from Codewars.

pub mod problem {

    pub fn iq_test(numbers: &str) -> u64 {
        let mut odds = 0;
        let mut evens = 0;

        let mut first_odd = 0usize;
        let mut first_even = 0usize;
        for (index, number) in numbers.split(" ").into_iter().enumerate() {
            let integer = str::parse::<i32>(number).unwrap();
            if integer & 0b1 == 0i32 {
                odds += 1;
                if first_odd == 0usize {
                    first_odd = index;
                }
            }
            else {
                evens += 1;
                if first_even == 0usize {
                    first_even = index;
                }
            }
        }

        if odds < evens {
            return (first_odd + 1) as u64;
        }

        (first_even + 1) as u64
    }


    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn iq_test_valid() {
            assert_eq!(iq_test("1 2 1 1"), 2);
            assert_eq!(iq_test("4 2 8 6 9 2 12"), 5);
        }
    }
}

use crate::problem::iq_test;

fn main() {
    println!("Different number: {}", iq_test("1 2 1 1"));
}
