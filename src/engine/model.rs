use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool)
}

#[derive(Debug)]
pub struct SimpleItem {
    pub hash_key_value: Value,
    pub attributes: HashMap<String, Value>
}

#[derive(Debug)]
pub struct SimpleStore {
    pub name: String,
    pub hash_key_name: String,
    pub item_by_hash_key: HashSet<String, SimpleItem>
}

#[derive(Debug)]
pub struct PartitionedItem {
    pub hash_key_value: Value,
    pub sort_key_value: Value,
    pub attributes: HashMap<String, Value>
}

#[derive(Debug)]
pub struct Partition {
    pub sort_key_name: String,
    pub items: BTreeSet<PartitionedItem>
}

#[derive(Debug)]
pub struct PartitionStore {
    pub name: String,
    pub hash_key_name: String,
    pub sort_key_name: String,
    pub partition_by_hash_key: HashMap<String, Partition>
}

#[derive(Debug)]
pub enum Store {
    Simple(SimpleStore),
    Partition(PartitionStore)
}

#[derive(Debug)]
pub struct Database {
    pub name: String,
    pub stores: Vec<Store>
}
