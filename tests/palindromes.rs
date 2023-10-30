//! # Palindrome checker and generator interface tests

use palindronum::{first_n_palindromes, is_palindrome, Palindrome};

/// Test the [is_palindrome] function with palindrome as input value.
#[test]
fn test_palindromes() {
  assert!(is_palindrome(111));
}

/// Test the [is_palindrome] function with non-palindrome as input value.
#[test]
fn test_non_palindromes() {
  assert!(!is_palindrome(100));
}

/// Test the [first_n_palindromes] function.
#[test]
fn test_get_palindromes() {
  assert_eq!(
    "[1, 2, 3, 4, 5, 6, 7, 8, 9, 11]",
    format!("{:?}", first_n_palindromes(10).collect::<Vec<Palindrome>>())
  );
}
