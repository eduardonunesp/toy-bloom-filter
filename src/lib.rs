///! Simple toy Bloom filter implementation
///
///! A Bloom filter is a space-efficient probabilistic data structure
///! that is used to test whether an element is a member of a set.
///! False positive matches are possible, but false negatives are not.
///! Elements can be added to the set, but not removed.
///! The more elements that are added to the set, the larger the probability of false positives.
///! The probability of false positives is determined by the number of hash functions and the size of the bit array.
///! This simple toy has 3 hash functions and a bit array of variable size.
///! A size of 256 bits is used by default.

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_empty_set() {
        let m = 256;
        let filter = Set::new();
        assert_eq!(format!("{}", filter), "0 ".repeat(m).trim());
    }

    #[test]
    fn add_element_to_set() {
        let mut filter = Set::new();
        filter.add(1);
        filter.add(2);
        filter.add(3);
        assert_eq!(
            format!("{}", filter),
            "0 1 1 1 0 1 0 1 1 1 0 0 0 0 0 0 1 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0",
        );
    }

    #[test]
    fn query_element_in_set() {
        let mut filter = Set::new();
        filter.add(1);
        filter.add(2);
        filter.add(3);
        assert_eq!(filter.query(1), true);
        assert_eq!(filter.query(2), true);
        assert_eq!(filter.query(3), true);
    }

    #[test]
    fn query_element_not_in_set() {
        let mut filter = Set::new();
        filter.add(1);
        filter.add(2);
        filter.add(3);
        assert_eq!(filter.query(4), false);
    }

    #[test]
    fn query_false_positive() {
        // Reduced the size of the bit array to 5 to increase the probability of false positives
        let mut filter = Set::with_size(5);
        filter.add(9);
        filter.add(11);
        assert_eq!(filter.query(16), true);
    }

    #[test]
    fn test_hash() {
        let m = 256;

        assert_eq!(Hash::hash(Hash::H1, 2, m), 2);
        assert_eq!(Hash::hash(Hash::H2, 2, m), 7);
        assert_eq!(Hash::hash(Hash::H3, 2, m), 16);
    }
}
