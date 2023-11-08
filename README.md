# Toy Bloom Filter

[![Test](https://github.com/eduardonunesp/toy-bloom-filter/actions/workflows/test.yml/badge.svg)](https://github.com/eduardonunesp/toy-bloom-filter/actions/workflows/test.yml)

## Introduction

This is a toy implementation of a Bloom filter in Rust. It is not meant to be used in production, but rather as a learning tool to understand how Bloom filters work.

## Testing

```cargo
cargo test
```

## Implementation

```rust
///! Hash functions
///! H1(x) = x mod M
///! H2(x) = (2x + 3) mod M
///! H3(x) = 8x mod M
pub enum Hash {
    H1,
    H2,
    H3,
}

///! Display the formula for the hash functions implementation for the Set
impl std::fmt::Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Hash::H1 => write!(f, "H1(x mod M)"),
            Hash::H2 => write!(f, "H2(2x + 3 mod M)"),
            Hash::H3 => write!(f, "H3(8x mod M)"),
        }
    }
}

///! Hash functions implementation
impl Hash {
    pub fn hash(hash: Hash, element: u8, m: usize) -> usize {
        match hash {
            Hash::H1 => element as usize % m,
            Hash::H2 => (2 * element + 3) as usize % m,
            Hash::H3 => (8 * element) as usize % m,
        }
    }
}

///! Set implementation
///! The set is implemented as a bit array of size M
///! The set is initialized with 0s
///! When an element is added to the set, the bits at the indexes
///! H1(x), H2(x) and H3(x) are set to 1
pub struct Set {
    bits: Vec<u8>,
}

impl std::fmt::Display for Set {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let sba = self
            .bits
            .iter()
            .map(|&x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        write!(f, "{}", sba)
    }
}

impl Default for Set {
    fn default() -> Self {
        Self::with_size(256)
    }
}

impl Set {
    ///! Create a new set with a bit array of size 256
    pub fn new() -> Self {
        Self::default()
    }

    ///! Create a new set with a bit array of size M
    pub fn with_size(size: usize) -> Self {
        Self {
            bits: vec![0u8; size],
        }
    }

    ///! Add an element to the set
    ///! The bits at the indexes H1(x), H2(x) and H3(x) are set to 1
    pub fn add(&mut self, element: u8) {
        let m = self.bits.len();
        self.bits[Hash::hash(Hash::H1, element, m)] = 1;
        self.bits[Hash::hash(Hash::H2, element, m)] = 1;
        self.bits[Hash::hash(Hash::H3, element, m)] = 1;
    }

    ///! Query an element in the set
    ///! The bits at the indexes H1(x), H2(x) and H3(x) are checked
    ///! If all the bits are set to 1, the element is in probably in the set
    pub fn query(&mut self, element: u8) -> bool {
        let m = self.bits.len();
        self.bits[Hash::hash(Hash::H1, element, m)] == 1
            && self.bits[Hash::hash(Hash::H2, element, m)] == 1
            && self.bits[Hash::hash(Hash::H3, element, m)] == 1
    }
}
```