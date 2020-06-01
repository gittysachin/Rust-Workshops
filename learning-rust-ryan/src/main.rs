// use std::collections::hash_map::DefaultHasher;
// use std::hash::{Hash, Hasher};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

fn main() {
    let mut map = OurMap::new();
    map.insert(1, 2);

    let value = 10;
    map.insert(90, value);
    map.insert(91, value); // here value does implements copy trait

    // but
    // let mut map = HashMap::new();
    // let value: Vec<u8> = Vec::new();
    // map.insert(90, value.clone());
    // map.insert(91, value); // here it does not implement copy trait , so what we can do is, we can clone it

    // OurMap::insert(map, 1, 2); // this is same as above
    // let value = map.get(1).unwrap();
    let value = map
        .get(1)
        .expect("I really thought this was not none. Sorry!");
    let value2 = map.get(2);
    // assert!(value == Some(2));
    assert!(value == 2); // this works if we're using unwrap or expect above
    assert!(value2 == None);
}

#[derive(Copy, Clone)] // implement Copy and Clone  ---> https://doc.rust-lang.org/std/marker/trait.Copy.html
struct OurStruct {
    age: u16,
    weight: u16,
}

struct OurMap {
    // buckets: Vec<(u64, i8)>,
    buckets: Vec<Option<(u64, i8)>>,
}

// struct OurMap<K, V> {
//     buckets: Vec<Option<(K, V)>>
// }

const INITIAL_BUCKET_SIZE: usize = 5381;

// impl<K, V> OurMap<K, V> {}

impl OurMap {
    fn new() -> OurMap {
        let mut buckets = Vec::with_capacity(INITIAL_BUCKET_SIZE);
        // let buckets = Vec::new();
        for _ in 0..INITIAL_BUCKET_SIZE {
            buckets.push(None);
        }
        OurMap { buckets }
        // OurMap {}
        // OurMap {
        //     buckets: Vec::new(),
        // }
    }

    fn insert(&mut self, key: u64, value: i8) {
        let index = self.get_index_for_key(key);
        self.buckets[index] = Some((key, value));
        // self.buckets.push((key, value));
    }

    // fn get(&mut self, key: u64) -> Option<i8> {
    //     for pair in &mut self.buckets {
    //         if pair.0 == key {
    //             return Some(pair.1);
    //         }
    //     }
    //     None
    // }

    // fn get(&self, key: u64) -> Option<i8> {
    //     self.buckets
    //         .iter()
    //         .find(|pair| pair.0 == key) // .find(| (k, _) | *k == key)   or k = &key
    //         .map(|pair| pair.1) // map(| (_, v) | *v)
    // }

    fn get(&self, key: u64) -> Option<i8> {
        let index = self.get_index_for_key(key);
        self.buckets.get(index).and_then(|p| *p).map(|(_, v)| v)
    }

    fn get_index_for_key(&self, key: u64) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % self.buckets.len() as u64) as usize // here len() returns usize and it depends on the machine, so here I used u64 as I'm on a 64-bit machine
    }
}

// fn another_fuction() {
//     let mut my_string = "hello".to_string(); // String::from("hello");
//                                              // make_excited(&mut my_string);
//     make_excited(&my_string, &my_string);
//     println!("Our greeting is {}", my_string);
//     // let wowow = make_excited(my_string);
//     // println!("Our greeting is {}", wowow);
// }

// fn make_excited(mut arg1: String) -> String {
//     arg1.push_str("!!!!");
//     arg1
// }

// fn make_excited(arg2: &mut String){
//     println!("HELLO {}", arg1);
// }

// fn make_excited(mut arg1: &mut String) {
//     arg1.push_str("!!!!!!");
// }

// fn make_excited(arg1: &String, arg2: &String) {
//     println!("HELLO {} {}", arg1, arg2);
// }
