use std::io::Cursor;
use bit_vec::BitVec;
use murmur3::murmur3_x64_128;


/*
    A basic implementation of a bloom filter.
    This uses the MurmurHash as the hashing function
*/

pub struct BloomFilter {
    bits: BitVec,
    hash_count: usize
}


impl BloomFilter {
    pub fn new(size: usize, hash_count: usize) -> BloomFilter {
        BloomFilter {
            bits: BitVec::from_elem(size, false),
            hash_count: hash_count,
        }
    }

    pub fn insert(&mut self, item: &str) {
        for i in 0..self.hash_count {
            let hash_result = murmur3_x64_128(&mut Cursor::new(item), i as u32).unwrap() as usize;
            let index = hash_result % (self.bits.len());
            self.bits.set(index, true);
        }
    }

    pub fn has(&mut self, item: &str) -> bool {
        for i in 0..self.hash_count {
            let hash_result = murmur3_x64_128(&mut Cursor::new(item), i as u32).unwrap() as usize;
            let index = hash_result % (self.bits.len());

            if !(self.bits[index]) {
                return false;
            }
        }
        true
    }
    
}

#[test]
fn test_insert() {
    let mut bloom_filter = BloomFilter::new(100, 10);

    let test_data: [&str; 5] = ["pizza", "pepperoni", "cheese", "sauce", "crust"];

    for item in test_data {
        bloom_filter.insert(item);
    }

    for item in test_data {
        assert!(bloom_filter.has(item));
    }
}

#[test]
fn test_not_inserted() {
    let mut bloom_filter = BloomFilter::new(100, 10);

    let test_data: [&str; 2] = ["dominos", "pizza_hut"];

    for item in test_data {
        assert_eq!(bloom_filter.has(item), false);
    }
}
