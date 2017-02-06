## ffi confusion:

This is a demonstration of something I'm confused about around Rust/C interop.

From Rust, there is struct that has a `Vec<*mut c_char>` member. From C, I seem to be able to treat this as a `**c_char`. This doesn't make sense to me. :|

