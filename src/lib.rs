use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap};
use std::io;

pub enum MenuOption {
    HashMap,
    BTreeMap,
    BTreeSet,
    BinaryHeap,
    Exit,
    Invalid,
}
pub fn menu_option_from_i8(value: i8) -> MenuOption {
    match value {
        1 => MenuOption::HashMap,
        2 => MenuOption::BTreeMap,
        3 => MenuOption::BTreeSet,
        4 => MenuOption::BinaryHeap,
        5 => MenuOption::Exit,
        _ => MenuOption::Invalid,
    }
}

pub fn get_user_option() -> i8 {
    let mut selected_option_value: String = String::new();
    io::stdin()
        .read_line(&mut selected_option_value)
        .expect("Fail to read the value");

    let entered_value: i8 = match selected_option_value.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Opps, Please enter a valid option");
            0
        }
    };
    entered_value
}
pub fn print_menu() {
    println!("\n===== Data Structure Menu =====");
    println!("1. Build a HashMap     (unordered, fast key lookup)");
    println!("2. Build a BTreeMap    (ordered map by key)");
    println!("3. Build a BTreeSet    (unique sorted values)");
    println!("4. Build a BinaryHeap  (priority queue / max-heap)");
    println!("5. Exit");
    println!("===============================\n");
    print!("Enter your choice: ");
}

pub fn build_hashmap() {
    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("One".to_string(), 1);
    map.insert("Two".to_string(), 2);
    println!("📦 HashMap: {:?}", map);
}

pub fn build_btreemap() {
    let mut btree_map: BTreeMap<String, i32> = BTreeMap::new();
    btree_map.insert("One".to_string(), 1);
    btree_map.insert("Two".to_string(), 2);
    println!("🌳 BTreeMap: {:?}", btree_map);
}

pub fn build_btreeset() {
    let mut btree_set: BTreeSet<i32> = BTreeSet::new();
    btree_set.insert(1);
    btree_set.insert(2);
    println!("🌲 BTreeSet: {:?}", btree_set);
}

pub fn build_binaryheap() {
    let mut binary_heap: BinaryHeap<i32> = BinaryHeap::new();
    binary_heap.push(1);
    binary_heap.push(2);
    println!("🔺 BinaryHeap (max-heap): {:?}", binary_heap);
}
