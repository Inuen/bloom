use bitvec::bitvec;
use bitvec::order::Msb0;
use bitvec::prelude::BitVec;
use twox_hash::xxhash3_64::Hasher;

pub struct Bloom {
    number_of_elements_to_be_inserted: i32,
    desired_false_positive_probability: f64,
    bits_in_the_filter: usize,
    number_of_hash_functions: u64,
    bitmap: BitVec<usize, Msb0>,
}

impl Bloom {
    pub fn get_optimal_number_of_hash_functions(
        bitmap_size: usize,
        number_of_elements_to_be_inserted: i32,
    ) -> u64 {
        (std::f64::consts::LN_2 * (bitmap_size as f64) / number_of_elements_to_be_inserted as f64)
            as u64
    }

    pub fn get_optimal_bitmap_size(
        number_of_elements_to_be_inserted: i32,
        desired_false_positive_probability: f64,
    ) -> usize {
        (-number_of_elements_to_be_inserted as f64 * desired_false_positive_probability.ln()
            / (std::f64::consts::LN_2 * std::f64::consts::LN_2).ceil()) as usize
    }

    /// Set bitmap to true for certain index. Index should be calculated as hash of an item.
    fn set(&mut self, idx: usize) {
        self.bitmap.set(idx, true)
    }

    /// Using double hashing (h1 + i * h2) to avoid computing N hashes, so should be much faster.
    /// Also reduces clustering in bloom filter allowing for more equal distribution.
    pub fn insert_item(&mut self, item: &str) {
        let item_bytes = item.as_bytes();
        let base_hash = Hasher::oneshot(item_bytes);
        let delta_hash = Hasher::oneshot(base_hash.to_string().as_bytes());
        for i in 0..self.number_of_hash_functions {
            let hash_i = base_hash.wrapping_add(i * delta_hash) as usize;
            self.set(hash_i % self.bits_in_the_filter)
        }
    }

    fn check(&self, index: usize) -> bool {
        self.bitmap[index]
    }

    /// Check if an item is present in the set, there can be false positives but no false negatives.
    /// Rationale is, if item appeared all the indexes in bitmap should be set to true, since
    /// hashing is deterministic. Please note it does not work for substrings, since they produce
    /// entirely different hash. It only provides an answer to "Did I insert exact same item?",
    /// so for strings the result also depends on how the items were inserted.
    /// e.g. if "TOFU" was inserted and "TOF" is checked, it will likely say that it's not in the set
    /// # Arguments
    /// * `item` - &str
    ///
    /// # Returns
    /// * `bool` - where false means the item is not was not seen in the set, and true it MIGHT be in the set
    pub fn might_contain(&self, item: &str) -> bool {
        let item_bytes = item.as_bytes();
        let base_hash = Hasher::oneshot(item_bytes);
        let delta_hash = Hasher::oneshot(base_hash.to_string().as_bytes());
        for i in 0..self.number_of_hash_functions {
            let hash_i = base_hash.wrapping_add(i * delta_hash) as usize;
            let idx = hash_i % self.bits_in_the_filter;
            if !self.check(idx) {
                return false;
            }
        }
        true
    }

    pub fn get_number_of_elements_to_be_inserted(self) -> i32 {
        self.number_of_elements_to_be_inserted
    }

    pub fn get_desired_false_positive_probability(self) -> f64 {
        self.desired_false_positive_probability
    }
}

impl Default for Bloom {
    fn default() -> Self {
        let number_of_elements_to_be_inserted = 100;
        let desired_false_positive_probability = 0.05;
        let bits_in_the_filter = Self::get_optimal_bitmap_size(
            number_of_elements_to_be_inserted,
            desired_false_positive_probability,
        );
        let number_of_hash_functions = Self::get_optimal_number_of_hash_functions(
            bits_in_the_filter,
            number_of_elements_to_be_inserted,
        );
        Self {
            number_of_elements_to_be_inserted,
            desired_false_positive_probability,
            bits_in_the_filter,
            number_of_hash_functions,
            bitmap: bitvec![usize, Msb0; 0; bits_in_the_filter],
        }
    }
}
