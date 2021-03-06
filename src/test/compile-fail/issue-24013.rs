// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn main() {
    use std::mem::{transmute, swap};
    let a = 1;
    let b = 2;
    unsafe {swap::<&mut _>(transmute(&a), transmute(&b))};
    //~^ ERROR unable to infer enough type information about `_`
}
