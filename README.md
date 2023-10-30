# Number palindromes

A palindrome is a number that is the same when the digits are reversed.
For example, 121, 2332, and 6 are all palindromes.
But 10 is not a palindrome (since leading zeroes are not allowed).
0 is treated as a palindrome.

To check if a number is a palindrome, use **is_palindrome** function, e.g:

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

- [MIT license](https://opensource.org/licenses/MIT) ([LICENSE-MIT](LICENSE-MIT)), or
- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([LICENSE-APACHE](LICENSE-APACHE))

at your option.