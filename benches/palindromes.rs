//! # Palindrome checker and generator benchmarks
//!
//! ```text
//! test bench_1_palindrome           ... bench:           0 ns/iter (+/- 0)
//! test bench_max_palindrome         ... bench:           0 ns/iter (+/- 0)
//! test bench_max_value              ... bench:           0 ns/iter (+/- 0)
//! test bench_get_2_palindromes      ... bench:           6 ns/iter (+/- 1)
//! test bench_get_100_palindromes    ... bench:       2,630 ns/iter (+/- 26)
//! test bench_get_1_000_palindromes  ... bench:     406,261 ns/iter (+/- 8,784)
//! ```
//!
//! Estimation of getting all palindromes for u32:
//! 2_240_000_000_000 ns = 2_240 s = 0.6 h 😢
//!

#![feature(test)]

extern crate test;

use palindronum::{first_n_palindromes, is_palindrome, Palindrome};
use test::Bencher;

#[bench]
fn bench_1_palindrome(b: &mut Bencher) {
  b.iter(|| {
    let _ = is_palindrome(0);
  });
}

#[bench]
fn bench_9_palindrome(b: &mut Bencher) {
  b.iter(|| is_palindrome(111111111));
}

#[bench]
fn bench_max_value(b: &mut Bencher) {
  b.iter(|| is_palindrome(Palindrome::MAX));
}

#[bench]
fn bench_get_2_palindromes(b: &mut Bencher) {
  b.iter(|| for _ in first_n_palindromes(2) {});
}

#[bench]
fn bench_get_100_palindromes(b: &mut Bencher) {
  b.iter(|| for _ in first_n_palindromes(100) {});
}

#[bench]
fn bench_get_1_000_palindromes(b: &mut Bencher) {
  b.iter(|| for _ in first_n_palindromes(1_000) {});
}
