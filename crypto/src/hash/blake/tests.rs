// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use super::{Blake3_256, ElementHasher, Hasher};
use math::{fields::f128ext::BaseElement, FieldElement};
use rand_utils::rand_array;

#[test]
fn hash_padding() {
    let b1 = [(1u8), (2u8), (3u8)];
    let b2 = [(1u8), (2u8), (3u8), (0u8)];

    // adding a zero bytes at the end of a byte string should result in a different hash
    let r1 = Blake3_256::<BaseElement>::hash(&b1);
    let r2 = Blake3_256::<BaseElement>::hash(&b2);
    assert_ne!(r1, r2);
}

#[test]
fn hash_elements_padding() {
    let e1: [BaseElement; 2] = rand_array();
    let e2 = [e1[0], e1[1], BaseElement::ZERO];

    // adding a zero element at the end of a list of elements should result in a different hash
    let r1 = Blake3_256::hash_elements(&e1);
    let r2 = Blake3_256::hash_elements(&e2);
    assert_ne!(r1, r2);
}
