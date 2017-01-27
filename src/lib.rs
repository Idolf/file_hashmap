// Copyright 2017 Mathias Svensson. See the COPYRIGHT file at the top-level
// directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(pub_restricted, dropck_eyepatch, generic_param_attrs, shared, unique, sip_hash_13, heap_api, oom, alloc, core_intrinsics)]
#![cfg_attr(test, feature(test))]

#[macro_use]
extern crate lazy_static;
extern crate rand;
extern crate alloc;
extern crate nix;
extern crate cast;

pub mod raw_alloc;
mod hashmap;

pub use self::hash_map::HashMap;
pub use self::hash_set::HashSet;

pub mod hash_map {
    //! A hash map implementation which uses linear probing with Robin
    //! Hood bucket stealing.
    pub use super::hashmap::map::*;
}

pub mod hash_set {
    //! An implementation of a hash set using the underlying representation of a
    //! HashMap where the value is ().
    pub use super::hashmap::set::*;
}
