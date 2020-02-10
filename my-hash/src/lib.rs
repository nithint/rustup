use std::collections::hash_map::DefaultHasher;
const NUM_BUCKETS: usize = 16; // usize is the arch-dependent unsigned int type
type Bucket<K,V> = Vec<(K, V)>;

pub struct HashMap<K,V> {
    buckets: Vec<Bucket<K, V>>,
}

// You could restrain the type K on the struct definition instead and that would be valid as well. 
// But this is saying that this particular implementation requires K to be constrained and is the general
// practice you'll see.
// There are multiple constraints on Hash, PartialEq is required to compare.
impl<K: std::hash::Hash + Eq,V> HashMap<K, V> { 

    pub fn new() -> Self { // Note that new is not a keyword in Rust.
        // Self can also be HashMap - Self allows you to not have to repeat the impl type.
        let mut buckets = Vec::new(); // Also do Vec::with_capacity(NUM_BUCKETS)
        for _ in 0..NUM_BUCKETS { 
            buckets.push(Vec::new());
        }

        // can change a mutable variable to immutable later on
        // let buckets = buckets;
        Self { buckets }
    }

    // String is a struct string, there is also str which is a view into a string.
    // You have ownership of a String but not of an str. A str is a splice.
    // pub fn insert(&mut self, key : K, val: V) -> Option<V> {  // This version, left as exercise to return old value.
    // pub fn insert(&mut self, key : K, val: V) { 
    //     use std::hash::Hasher;
    //     // todo!() // todo is a macro - just stands for todo - generates error at runtime.
    //     let mut hasher = DefaultHasher::new();
    //     key.hash(&mut hasher);
    //     let hash = hasher.finish();

    //     let bucket_index = hash % (NUM_BUCKETS as u64);
    //     self.buckets[bucket_index as usize] .push((key, val));
    // }

    pub fn insert(&mut self, key : K, val: V) { 
        // todo!() // todo is a macro - just stands for todo - generates error at runtime.
        use std::hash::Hasher;
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();

        let bucket_index = hash % (NUM_BUCKETS as u64);
        let bucket = &mut self.buckets[bucket_index as usize];
        for pair in bucket.iter_mut() { // pair is a mutable ref to each tuple
            if pair.0 == key {
                let mut new_pair = (key, val);
                std::mem::swap(pair, &mut new_pair);
                return;
            }
        }

        bucket.push((key, val));
    }

    pub fn get(&self, key: &K) -> std::option::Option<&V> {
        use std::hash::Hasher;
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();

        let bucket_index = hash % (NUM_BUCKETS as u64);
        let bucket = &mut self.buckets[bucket_index as usize];
        for pair in bucket.iter() { 
            if pair.0 == key {
                return std::option::Option::Some(&pair.1);
            }
        }

        None
     }
}

// Test are usually placed in same file as implementation in Rust.
#[cfg(test)] // cfg is compiler flag - only compile if running cargo test. '#' syntax are called attributes.
mod tests { // mod here defines a new module and a file itself is also a module. The tests module is a submodule.
    use super::*; // Use all types in my parent module. Super means parent.
    #[test]
    fn it_works() {
        // create a new HashMap
        let mut map = HashMap::new();

        // insert key/value pairs into the HashMap
        assert_eq!(map.insert("foo", "bar"), None);
        // if an item already exists for that value, it should return the old value
        assert_eq!(map.insert("foo", "lol"), Some("bar"));

        // get a value based on its key
        assert_eq!(map.get(&"foo"), Some(&"lol"));
        // you should be able to do this multiple times
        assert_eq!(map.get(&"foo"), Some(&"lol"));
        // if no value exists for a key, return None
        assert_eq!(map.get(&"qux"), None);

        // remove a value for a key
        assert_eq!(map.remove(&"foo"), Some("lol"));
        // once it no longer exists in the map, it should return None
        assert_eq!(map.get(&"foo"), None);
    }
}
