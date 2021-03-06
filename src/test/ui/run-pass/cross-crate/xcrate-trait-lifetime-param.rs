// Copyright 2013-2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass
// aux-build:xcrate-trait-lifetime-param.rs

// pretty-expanded FIXME #23616

extern crate xcrate_trait_lifetime_param as other;

struct Reader<'a> {
    b : &'a [u8]
}

impl <'a> other::FromBuf<'a> for Reader<'a> {
    fn from_buf(b : &'a [u8]) -> Reader<'a> {
        Reader { b : b }
    }
}

pub fn main () {}
