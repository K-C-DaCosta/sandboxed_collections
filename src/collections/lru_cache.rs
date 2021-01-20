use super::linked_list::*;
use std::{collections::HashMap, hash::Hash};

/// # Description
/// A generic LRU cache implemented using a hashtable and doubly-linked-list
/// # Comments
/// The implementation clones keys very liberally, so keys should be `Copy` but
/// if that's not possible try to use `Clone`-friendly keys
pub struct LruCache<K, V> {
    key_table: HashMap<K, u32>,
    list: LinkedList<(K, V)>,
    cache_size: usize,
}

impl<K, V> LruCache<K, V>
where
    K: Clone + Eq + Hash,
{
    /// # Description
    /// Creates a new LruCache of size `cache_size`
    /// # Comments
    /// No allication takes place here
    pub fn new(cache_size: usize) -> Self {
        Self {
            key_table: HashMap::new(),
            list: LinkedList::new(),
            cache_size,
        }
    }
    
    /// # Description
    /// Puts a `key`-`value` pair into the `LruCache`
    /// # Comments
    /// If there isn't enough space, the Least Recently Used
    /// Key-Value pair gets removed
    pub fn put(&mut self, key: K, val: V) {
        if self.list.len() < self.cache_size {
            //cache isnt full so just push front
            self.list.push_front((key.clone(), val));
            let node_ptr = self.list.front;
            self.key_table.insert(key, node_ptr);
        } else {
            //cache full
            match self.key_table.get(&key) {
                // key exists  ( move existing node to front, update hashtable )
                Some(&cur_node) => {
                    let (rkey, _rval) = self.list.remove(cur_node).unwrap();
                    // push to the top with updated value
                    self.list.push_front((rkey.clone(), val));
                    let new_ptr = self.list.front;
                    *self.key_table.get_mut(&rkey).expect("Key should exist") = new_ptr;
                }
                // key doesnt exist ( remove LRU, push new val front, update hashtable)
                None => {
                    let (rkey, _rval) = self.list.pop_rear().unwrap();
                    self.key_table.remove(&rkey).expect("Key should exist");
                    self.list.push_front((key.clone(), val));
                    let new_node = self.list.front;
                    self.key_table.insert(key, new_node);
                }
            }
        }
    }

    /// # Description
    /// fetches value associated with `key`, once called
    /// value priority gets upgraded
    pub fn get(&mut self, key: &K) -> Option<&mut V> {
        let key_table = &mut self.key_table;
        let list = &mut self.list;
        let &node_ptr = key_table.get(&key)?;

        let (rkey, rval) = list.remove(node_ptr).expect("val should exist");

        list.push_front((rkey.clone(), rval));
        let new_node = list.front;

        *key_table.get_mut(&key).expect("key should_exist") = new_node;

        // return newly prioritized node
        list[new_node].get_data_mut().map(|(_, v)| v)
    }
    
    /// # Description 
    /// An iterator that walks through all items in the cache
    pub fn iter(&self) -> impl Iterator<Item = &(K, V)> {
        self.list.iter().map(|node| node.get_data().unwrap())
    }
}

#[test]
fn simple_test(){
    let to_vec = |c:&LruCache<String,i32>| -> Vec<_>{
        c.iter().map(|(k,_v)| k.clone() ).collect()
    };

    let tostr = |a|{
        String::from(a)
    };

    let mut lru = LruCache::<String, i32>::new(4);

    lru.put(String::from("a"), 1);
    assert_eq!( to_vec(&lru) , [tostr("a")] );

    lru.put(String::from("b"), 1);
    assert_eq!( to_vec(&lru) , [tostr("b"),tostr("a")] );

    lru.put(String::from("c"), 1);
    assert_eq!( to_vec(&lru) , [tostr("c"),tostr("b"),tostr("a")] );

    lru.put(String::from("d"), 1);
    assert_eq!( to_vec(&lru) , [tostr("d"),tostr("c"),tostr("b"),tostr("a")] );

    lru.put(String::from("e"), 1);
    assert_eq!( to_vec(&lru) , [tostr("e"),tostr("d"),tostr("c"),tostr("b")]);

    lru.get(&String::from("b"));
    assert_eq!( to_vec(&lru) , [tostr("b"),tostr("e"),tostr("d"),tostr("c")]);

    lru.get(&String::from("c"));
    assert_eq!( to_vec(&lru) , [tostr("c"),tostr("b"),tostr("e"),tostr("d")]);
}