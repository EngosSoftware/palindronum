//! # Number palindrome checker and generator implementation

/// Type alias used as a base type for checked or generated palindromes.
///
/// Users are encouraged to use this type alias in theirs applications,
/// to avoid inconsistencies and compilation errors when the number type
/// changes in future versions.
pub type Palindrome = u32;

/// Returns **true** if the specified number is a palindrome.
///
/// # Example
///
/// ```
/// use palindronum::is_palindrome;
///
/// assert!(is_palindrome(121));
/// assert!(!is_palindrome(123));
///
/// ```
#[inline(always)]
pub fn is_palindrome(n: Palindrome) -> bool {
  let mut num: u64 = n.into();
  let mut rev = 0u64;
  while num > 0 {
    rev = rev * 10 + num % 10;
    num /= 10;
  }
  rev == n.into()
}

/// Returns an iterator over first **n** palindromes starting from 1.
///
/// Current algorithm is a brute force checking if each number is a palindrome,
/// starting from 1. If a number is a palindrome then is returned by the iterator.
/// Iterator stops issuing palindromes when the requested number is reached
/// or there is no more palindromes in base type [Palindrome].
///
///  # Example
///
///```
/// use palindronum::first_n_palindromes;
/// use std::fmt::Write;
///
/// let mut buf = String::new();
/// _ = write!(&mut buf, "First 10 palindromes:");
/// for x in first_n_palindromes(10) {
///    _ = write!(&mut buf, " {x}");
/// }
/// assert_eq!("First 10 palindromes: 1 2 3 4 5 6 7 8 9 11", buf);
///```
pub fn first_n_palindromes(n: Palindrome) -> impl Iterator<Item = Palindrome> {
  PalindromeIteratorState { next: 0, count: 0, max: n }
}

/// Internal state of the palindrome iterator.
struct PalindromeIteratorState {
  /// Next number to be checked against palindrome attribute.
  next: Palindrome,
  /// The number of already issued palindromes.
  count: Palindrome,
  /// Maximum requested number of palindromes.
  max: Palindrome,
}

impl Iterator for PalindromeIteratorState {
  type Item = Palindrome;
  /// Advances the iterator and returns the next palindrome.
  fn next(&mut self) -> Option<Self::Item> {
    while self.next < Palindrome::MAX && self.count < self.max {
      self.next += 1;
      if is_palindrome(self.next) {
        self.count += 1;
        return Some(self.next);
      }
    }
    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  /// Test numbers being palindromes.
  #[test]
  fn test_palindromes() {
    assert!(is_palindrome(0));
    assert!(is_palindrome(1));
    assert!(is_palindrome(2));
    assert!(is_palindrome(3));
    assert!(is_palindrome(4));
    assert!(is_palindrome(5));
    assert!(is_palindrome(6));
    assert!(is_palindrome(7));
    assert!(is_palindrome(8));
    assert!(is_palindrome(9));
    assert!(is_palindrome(11));
    assert!(is_palindrome(22));
    assert!(is_palindrome(121));
    assert!(is_palindrome(2332));
    assert!(is_palindrome(4294884924));
  }

  /// Test numbers being non-palindromes.
  #[test]
  fn test_non_palindromes() {
    assert!(!is_palindrome(12));
    assert!(!is_palindrome(122));
    assert!(!is_palindrome(4294884925));
    assert!(!is_palindrome(Palindrome::MAX));
  }

  /// Test the iterator over n first palindromes.
  #[test]
  fn test_get_palindromes() {
    // utility comparator to avoid code duplication in the following tests
    fn eq(expected: Palindrome, n: Palindrome) {
      let actual = first_n_palindromes(n).nth((n - 1) as usize).unwrap();
      assert_eq!(expected, actual);
    }
    eq(1, 1);
    eq(2, 2);
    eq(9, 9);
    eq(99, 18);
    eq(191, 28);
    eq(90109, 1_000);
  }
}
