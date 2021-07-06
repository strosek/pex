//! Module that implements solution to this problem: Clean String from Codewars.

pub mod problem {

    pub fn clean_string(s: &str) -> String {
        let mut clean = String::new();

        for character in s.chars() {
            if character != '#' {
                clean.push(character);
            } else {
                clean.pop();
            }
        }

        clean
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn iq_test_valid() {
            assert_eq!(clean_string("a#bb#"), "b");
            assert_eq!(clean_string(""), "");
            assert_eq!(clean_string("#####"), "");
        }
    }
}

use crate::problem::clean_string;

fn main() {
    println!("Clean string: {}", clean_string("a#bb#"));
}
