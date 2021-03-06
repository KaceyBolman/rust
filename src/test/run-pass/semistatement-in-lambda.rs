// Copyright 2017 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(unused_must_use)]

pub fn main() {
    // Test that lambdas behave as unary expressions with block-like expressions
    -if true { 1 } else { 2 } * 3;
    || if true { 1 } else { 2 } * 3;

    // The following is invalid and parses as `if true { 1 } else { 2 }; *3`
    // if true { 1 } else { 2 } * 3
}
