# ArrayVec with Const Generics

[![Build Status](https://travis-ci.com/Michael-F-Bryan/const-arrayvec.svg?branch=master)](https://travis-ci.com/Michael-F-Bryan/const-arrayvec)

([API Docs])

A copy of the [arrayvec](https://crates.io/crates/arrayvec) crate implemented 
using const generics.

> **Warning:** This isn't meant for production. Even `rustc` says const generics
> may crash the compiler. Use at your own risk.

To get a better understanding of this crate's architecture, check out [the
accompanying blog post][blog].

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[API Docs]: https://michael-f-bryan.github.io/const-arrayvec
[blog]: http://adventures.michaelfbryan.com/posts/const-arrayvec.md