[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-license-url]
[![Apache 2.0 licensed][apache-badge]][apache-license-url]
[![Contributor Covenant][cc-badge]][cc-url]

[crates-badge]: https://img.shields.io/crates/v/palindronum.svg

[crates-url]: https://crates.io/crates/palindronum

[mit-badge]: https://img.shields.io/badge/License-MIT-blue.svg

[mit-url]: https://opensource.org/licenses/MIT

[mit-license-url]: https://github.com/EngosSoftware/palindronum/blob/main/LICENSE-MIT

[apache-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg

[apache-url]: https://www.apache.org/licenses/LICENSE-2.0

[apache-license-url]: https://github.com/EngosSoftware/palindronum/blob/main/LICENSE

[apache-notice-url]: https://github.com/EngosSoftware/palindronum/blob/main/NOTICE

[cc-badge]: https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg

[cc-url]: https://github.com/EngosSoftware/palindronum/blob/main/CODE_OF_CONDUCT.md

# Number palindromes

A palindrome is a number that is the same when the digits are reversed.
For example, 121, 2332, and 6 are all palindromes.
But 10 is not a palindrome (since leading zeroes are not allowed).
0 is treated as a palindrome.

To check if a number is a palindrome, use **is_palindrome** function, e.g.:

```rust
 let x = 123; // no, this is not a palindrome
 let is_palindrome = palindronum::is_palindrome(x);
 println!("{x} is a palindrome: {is_palindrome}");
```
 output:

```text
 123 is a palindrome: false
```

```rust
 let x = 121; // yes, this is a palindrome
 let is_palindrome = palindronum::is_palindrome(x);
 println!("{x} is a palindrome: {is_palindrome}");

```
 output:

```text
 121 is a palindrome: true
```

 To generate first _n_ palindromes, use **first_n_palindromes** function, e.g.:

 ```rust
 let first_10_palindromes = palindronum::first_n_palindromes(10);
 for x in first_10_palindromes {
   println!("{x:2} is a palindrome");
 }
 ```
 output:

```text
  1 is a palindrome
  2 is a palindrome
  3 is a palindrome
  4 is a palindrome
  5 is a palindrome
  6 is a palindrome
  7 is a palindrome
  8 is a palindrome
  9 is a palindrome
 11 is a palindrome
```

## License

Licensed under either of

- [MIT license][mit-url] (see [LICENSE-MIT][mit-license-url]) or
- [Apache License, Version 2.0][apache-url] (see [LICENSE][apache-license-url] and [NOTICE][apache-notice-url])

at your option.

## Contribution

Any contributions to [palindronum](https://github.com/EngosSoftware/palindronum) are greatly appreciated.
All contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.
