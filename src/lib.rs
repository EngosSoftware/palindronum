//! # Number palindromes checker and generator
//!
//! A palindrome is a number that is the same when the digits are reversed.
//! For example, 121, 2332, and 6 are all palindromes.
//! But 10 is not a palindrome (since leading zeroes are not allowed).
//! 0 is treated as a palindrome.
//!
//! The maximum accepted number value is [Palindrome::MAX].
//!
//! To check if a number is a palindrome, use **[is_palindrome]** function, e.g:
//!
//!```
//! let x = 123; // no, this is not a palindrome
//! let is_palindrome = palindronum::is_palindrome(x);
//! println!("{x} is a palindrome: {is_palindrome}");
//!```
//! output:
//!
//!```text
//! 123 is a palindrome: false
//!```
//!
//!```
//! let x = 121; // yes, this is a palindrome
//! let is_palindrome = palindronum::is_palindrome(x);
//! println!("{x} is a palindrome: {is_palindrome}");
//!
//!```
//! output:
//!
//!```text
//! 121 is a palindrome: true
//!```
//!
//! To generate first _n_ palindromes, use **[first_n_palindromes]** function, e.g.:
//!
//! ```
//! use palindronum::first_n_palindromes;
//!
//! let first_10_palindromes = first_n_palindromes(10);
//! for x in first_10_palindromes {
//!   println!("{x:2} is a palindrome");
//! }
//! ```
//! output:
//!
//!```text
//!  1 is a palindrome
//!  2 is a palindrome
//!  3 is a palindrome
//!  4 is a palindrome
//!  5 is a palindrome
//!  6 is a palindrome
//!  7 is a palindrome
//!  8 is a palindrome
//!  9 is a palindrome
//! 11 is a palindrome
//!```

#![no_std]
#![deny(missing_docs)]

mod palindromes;

pub use palindromes::{first_n_palindromes, is_palindrome, Palindrome};
