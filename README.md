# brotli
A native Rust implementation of Google's Brotli compression format, which can be found here:

https://github.com/google/brotli

This is a "learn Rust" exercise for me, with an eye towards investigating the practicalities
and issues of porting a relatively complex and performance-sensitive codebase.

Initially, only the decoder will be implemented, as a more-or-less 1:1 port, with only minor
changes to convert "C-style" coding to "Rust-style" where appropriate.

To embrace Rust's safety aspect, additional checks will be added to Google's code, and 
unsafe code will only be used if absolutely necessary. This is likely to cause performance 
regressions, but should make the Rust implementation a better match for security-sensitive
applications such as web browers.
