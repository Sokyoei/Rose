/// Rust 容器
///
///
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};

fn main() {
    // Array 数组，固定长度，存储相同类型的数据
    let a = [1, 2, 3];
    println!("Array: {:?}", a);

    // Tuple 元组，固定长度，存储不同类型的数据
    let t = (1, "hello", 3.14);
    println!("Tuple: {:?}", t);

    // Silce 切片，动态长度的视图，不拥有数据，索引访问
    let s = [1, 2, 3, 4, 5];
    let slice = &s[1..3];
    println!("Slice: {:?}", slice);

    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // [`std::collections`] 标准库提供的容器
    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // Vector 动态数组
    let mut v: Vec<i32> = vec![1, 2, 3];
    v.push(4);
    v.push(5);
    v.push(6);
    println!("Vec: {:?}", v);

    // VecDeque 双端队列
    let mut deque = VecDeque::new();
    deque.push_back(1);
    deque.push_back(2);
    deque.push_back(3);
    println!("VecDeque: {:?}", deque);

    // LinkedList 双向链表
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    println!("LinkedList: {:?}", list);

    // HashSet 哈希集合
    let mut set = HashSet::new();
    set.insert(1);
    set.insert(2);
    set.insert(3);
    println!("HashSet: {:?}", set);

    // HashMap 哈希表
    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    println!("HashMap: {:?}", map);

    // BTreeSet 有序集合
    let mut btree_set = BTreeSet::new();
    btree_set.insert(1);
    btree_set.insert(2);
    btree_set.insert(3);
    println!("BTreeSet: {:?}", btree_set);

    // BTreeMap 有序映射
    let mut btree_map = BTreeMap::new();
    btree_map.insert("a", 1);
    btree_map.insert("b", 2);
    btree_map.insert("c", 3);
    println!("BTreeMap: {:?}", btree_map);

    // BinaryHeap 优先队列
    let mut heap = BinaryHeap::new();
    heap.push(1);
    heap.push(2);
    heap.push(3);
    println!("BinaryHeap: {:?}", heap);
}
