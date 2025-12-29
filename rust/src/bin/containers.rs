/// Rust 容器
///
///
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

fn main() {
    // Vector
    let mut v: Vec<i32> = vec![1, 2, 3];
    v.push(4);
    v.push(5);
    v.push(6);
    println!("Vec: {:?}", v);

    // Array
    let a = [1, 2, 3];
    println!("Array: {:?}", a);

    // Tuple
    let t = (1, 2, 3);
    println!("Tuple: {:?}", t);

    // HashSet
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    println!("HashSet: {:?}", set);

    // HashMap
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    println!("HashMap: {:?}", map);

    // BTreeSet
    let mut btree_set = BTreeSet::new();
    btree_set.insert(1);
    btree_set.insert(2);
    btree_set.insert(3);
    println!("BTreeSet: {:?}", btree_set);

    // BTreeMap
    let mut btree_map = BTreeMap::new();
    btree_map.insert("a", 1);
    btree_map.insert("b", 2);
    btree_map.insert("c", 3);
    println!("BTreeMap: {:?}", btree_map);

    // LinkedList
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    println!("LinkedList: {:?}", list);

    // VecDeque
    let mut deque = VecDeque::new();
    deque.push_back(1);
    deque.push_back(2);
    deque.push_back(3);
    println!("VecDeque: {:?}", deque);

    // BinaryHeap
    let mut heap = BinaryHeap::new();
    heap.push(1);
    heap.push(2);
    heap.push(3);
    println!("BinaryHeap: {:?}", heap);
}
