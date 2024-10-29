use std::collections::BTreeSet;
use std::collections::HashSet;
use std::collections::BTreeMap;

fn main() {
    let s: String = "She sells sea shells by the sea shore.".to_string();
    let vec: Vec<String> = s.split_whitespace().map(str::to_string).collect();

    println!("vector: {:?}", vec);

    let btree: BTreeSet<String> = s.split_whitespace().map(str::to_string).collect();

    println!("btree: {:?}", btree);

    let hashset: HashSet<String> = s.split_whitespace().map(str::to_string).collect();

    println!("hashset: {:?}", hashset);

    let mut btreemap: BTreeMap<usize, String> = BTreeMap::new();

    for index in 0..vec.len(){
        btreemap.insert(index, vec[index].clone());
    }

    println!("btreemap: {:?}", btreemap);

    let mut btreemap2: BTreeMap<String, usize> = BTreeMap::new();

    for index in 0..vec.len(){
        btreemap2.insert(vec[index], index);
    }

    println!("btreemap2: {:?}", btreemap2);
}
