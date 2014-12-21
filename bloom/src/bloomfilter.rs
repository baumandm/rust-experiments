use std::num::Float;

use std::hash::Hasher;
use std::hash::sip::SipHasher;

pub struct BloomFilter {
    m: uint,             // Number of bits
    k: uint,             // Number of hashing functions
    buckets: Vec<bool>
}

impl BloomFilter {
    #[allow(dead_code)]
    pub fn new () -> BloomFilter {
        BloomFilter::new_with_options(128, 3)
    }

    pub fn new_with_options (m: uint, k: uint) -> BloomFilter {
        let m = (m as f32 / 32.0).ceil() as uint * 32u;

        BloomFilter {
            m: m,
            k: k,
            buckets: Vec::from_elem(m, false)
         }
    }

    pub fn get_m (&self) -> uint {
        self.m
    }

    pub fn get_k (&self) -> uint {
        self.k
    }

    pub fn add (&mut self, value: &str) {
        let hashes = self.get_hashes(value);
        let buckets = self.buckets.as_mut_slice();

        for hash in hashes.iter() {
            buckets[*hash] = true;
        }
    }

    pub fn test (&self, value: &str) -> bool {
        let hashes = self.get_hashes(value);

        for hash in hashes.iter() {
            if self.buckets[*hash] == false {
                return false;
            }
        }

        true
    }

    fn get_hashes (&self, value: &str) -> Vec<uint> {
        let mut l: Vec<uint> = Vec::with_capacity(self.m);

        let m64 = self.m as u64;
        for i in range(0, self.k) {
            let hash = SipHasher::new_with_keys(3, 8*i as u64).hash(value) % m64;
            l.push(hash as uint);
        }

        //println!("Locations: {}", l);
        l
    }
}

#[cfg(test)]
mod test {
    use super::BloomFilter;

    #[test]
    fn new () {
        let bloomer = BloomFilter::new();
        assert_eq!(128u, bloomer.get_m());
        assert_eq!(3u, bloomer.get_k());
    }

    #[test]
    fn new_with_options () {
        let bloomer = BloomFilter::new_with_options(256, 16);
        assert_eq!(256u, bloomer.get_m());
        assert_eq!(16u, bloomer.get_k());
    }

    #[test]
    fn new_with_options_2 () {
        let bloomer = BloomFilter::new_with_options(120, 16);
        assert_eq!(128u, bloomer.get_m());
        assert_eq!(16u, bloomer.get_k());
    }

    #[test]
    fn add () {
        let mut bloomer = BloomFilter::new_with_options(256, 16);
        bloomer.add("one");
        assert_eq!(true, bloomer.test("one"));
        assert_eq!(false, bloomer.test("two"));

        bloomer.add("two");
        assert_eq!(true, bloomer.test("two"));
    }

    #[test]
    fn small_size () {
        let mut bloomer = BloomFilter::new_with_options(4, 1);
        bloomer.add("one");
        assert_eq!(true, bloomer.test("one"));
        assert_eq!(false, bloomer.test("two"));

        bloomer.add("two");
        assert_eq!(true, bloomer.test("two"));

        bloomer.add("three");
        bloomer.add("four");
        bloomer.add("five");
        assert_eq!(true, bloomer.test("four"));
    }

    #[test]
    fn large_size () {
        let mut bloomer = BloomFilter::new_with_options(8192, 11);

        for i in range(0i, 1024i) {
            bloomer.add(format!("value-{}", i).as_slice());
        }

        assert_eq!(true, bloomer.test("value-1"));
        assert_eq!(true, bloomer.test("value-5"));

        assert_eq!(true, bloomer.test("value-1023"));

        let mut false_positives = 0i;
        for i in range(0i, 1024i) {
            let result = bloomer.test(format!("fake-{}", i).as_slice());
            if result == true {
                false_positives += 1;
            }
        }

        println!("False Positives: {}", false_positives);
        assert!(false_positives < 100);
    }
}
