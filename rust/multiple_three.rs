//! Module that implements solution to
//! [this LeetCode problem](https://leetcode.com/problems/largest-multiple-of-three/).

pub mod problem {
    /// Build and return a string using the digits in `digits` vector, but removing the digits with
    /// indices present in the `removals` vector.
    ///
    /// # Arguments
    ///
    /// * `digits` - A vector of digits from which the number will be constructed
    /// * `removals` - A vector of indices of elements to be removed from `digits`
    ///
    /// # Examples
    ///
    /// ```
    /// use problems::problem::build_number_string;
    ///
    /// build_number_string(&vec![1, 2, 3, 4], &vec![1, 2]);
    /// build_number_string(&vec![1, 2, 3, 4], &vec![]);
    /// ```
    fn build_number_string(digits: &Vec<i32>, removals: &Vec<usize>) -> String {
        if digits.is_empty() {
            return String::from("");
        }

        let mut string_digits = Vec::with_capacity(digits.len());

        for i in 0..digits.len() {
            if !removals.contains(&i) {
                string_digits.push(digits[i].to_string());
            }
        }

        string_digits.join("")
    }

    pub fn largest_multiple_of_three(digits: Vec<i32>) -> String {
        let mut multiple = String::with_capacity(10_000usize);

        // Special case with empty vector.
        if digits.is_empty() {
            return multiple;
        }

        let sum: i32 = digits.iter().sum();

        // Special case when all digits are zeros.
        if sum == 0 {
            multiple.push('0');
            return multiple;
        }

        let mut sorted_digits = digits.to_vec();
        sorted_digits.sort_unstable();
        sorted_digits.reverse();

        // Return number if it's already divisible by 3.
        if sum % 3 == 0 {
            return build_number_string(&sorted_digits, &vec![]);
        }

        let mut reminder = sum % 3;
        // Try removing one digit.
        for i in (0..sorted_digits.len()).rev() {
            reminder = (sum - sorted_digits[i]) % 3;
            if reminder == 0 {
                multiple = build_number_string(&sorted_digits, &vec![i]);
                break;
            }
        }

        // Try removing two digits. All combinations of two starting from lowest numbers.
        if reminder != 0 {
            for i in (0..sorted_digits.len()).rev() {
                for j in ((i + 1)..sorted_digits.len()).rev() {
                    reminder = (sum - sorted_digits[i] - sorted_digits[j]) % 3;
                    if reminder == 0 {
                        multiple = build_number_string(&sorted_digits, &vec![i, j]);
                        break;
                    }
                }
            }
        }

        multiple
    }


    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn build_string_from_valid_vectors() {
            assert_eq!(build_number_string(&vec![1, 2, 3, 4], &vec![1, 2]), "14");
        }

        #[test]
        fn build_string_from_empty_digits() {
            assert_eq!(build_number_string(&vec![], &vec![1, 2]), "");
        }

        #[test]
        fn build_string_from_empty_removals() {
            assert_eq!(build_number_string(&vec![1, 2, 3, 4], &vec![]), "1234");
        }

        #[test]
        fn largest_multiple_with_valid_numbers() {
            assert_eq!(largest_multiple_of_three(vec![891]), "891");
            assert_eq!(largest_multiple_of_three(vec![7, 7, 0, 7, 7]), "7770");
            assert_eq!(largest_multiple_of_three(vec![1, 8, 9, 6, 7]), "9876");
            assert_eq!(largest_multiple_of_three(vec![8, 6, 7, 1, 0]), "8760");
        }

        #[test]
        fn largest_multiple_with_empty_digits() {
            assert_eq!(largest_multiple_of_three(vec![]), "");
        }

        #[test]
        fn largest_multiple_with_zeros() {
            assert_eq!(largest_multiple_of_three(vec![0, 0, 0]), "0");
        }

        #[test]
        fn largest_multiple_with_impossible_digits() {
            assert_eq!(largest_multiple_of_three(vec![1, 1]), "");
        }
    }
}

use crate::problem::largest_multiple_of_three;

fn main() {
    println!("Largest multiple of three: {}", largest_multiple_of_three(vec![1, 2, 3]));
}
