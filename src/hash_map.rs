use std::{
    alloc::{self, Layout},
    sync::RwLock,
    mem
};

use owning_ref::*;

/// An error that may be due to insertion of duplicate key.
#[derive(Debug)]
pub struct DupErr {
    pub key: i32
}

#[derive(Clone, Debug)]
struct Item<V> {
    key: i32,
    value: V,
    state: CellState
}

/// Thread-safe HashMap.
/// A wrapper over the `HashMap<V>`
pub struct TsHashMap<V> {
    hm: RwLock<HashMap<V>>
}

impl<V: Eq + Clone> TsHashMap<V> {
    /// Creates an empty `TsHashMap`.
    /// 
    /// # Examples
    ///
    /// ```
    /// use mk_collections::TsHashMap;
    ///
    /// let map = TsHashMap::<i32>::new();
    /// assert_eq!(map.capacity(), 0);
    /// ```
    pub fn new() -> TsHashMap<V> {
        TsHashMap{
            hm: RwLock::new(HashMap::new())
        }
    }

    /// Creates an empty `TsHashMap` with the specified capacity.
    /// 
    /// # Examples
    ///
    /// ```
    /// use mk_collections::TsHashMap;
    ///
    /// let map = TsHashMap::<i32>::with_capacity(10);
    /// assert_eq!(map.capacity(), 10);
    /// ```
    pub fn with_capacity(capacity: usize) -> TsHashMap<V> {
        TsHashMap {
            hm: RwLock::new(HashMap::with_capacity(capacity))
        }
    }

    /// Gets capacity 
    pub fn capacity(&self) -> usize {
        let lock = self.hm.read().unwrap();

        lock.capacity()
    }

    /// Inserts the new key-value pair if it's not or updates the value if key is aloredy present in the map.
    /// If it updates the old value will be returned, otherwise - [`None`].
    /// 
    /// # Examples
    ///
    /// ```
    /// use mk_collections::TsHashMap;
    ///
    /// let mut map = TsHashMap::new();
    /// assert!(map.insert(3, "a").is_none());
    /// assert_eq!(map.insert(3, "b").unwrap(), "a");
    /// ```
    pub fn insert(&self, key: i32, value: V) -> Option<V> {
        let mut map = self.hm.write().unwrap();

        map.insert(key, value)
    }

    /// Returns a reference to the value corresponding to the key, or [`None`] if it didn't found in the map.
    /// 
    /// # Examples
    ///
    /// ```
    /// use mk_collections::TsHashMap;
    ///
    /// let mut map = TsHashMap::new();
    /// assert!(map.insert(3, "a").is_none());
    /// assert_eq!(*map.find(3).unwrap(), "a");
    /// assert!(map.find(4).is_none());
    /// ```
    pub fn find<'ret, 'me: 'ret>(&'me self, key: i32) -> Option<RwLockReadGuardRef<'ret, HashMap<V>, V>> {
        let lock = self.hm.read().unwrap();
        
        if lock.contains_key(key) {
            let guard_ref = RwLockReadGuardRef::new(lock);
        
            return Some(guard_ref.map(|hm| hm.find(key).unwrap()));
        } else {
            return None;
        }
    }

    /// Removes a key from the map, returning the value at the key if the key
    /// was previously in the map.
    /// 
    /// # Examples
    ///
    /// ```
    /// use mk_collections::TsHashMap;
    ///
    /// let mut map = TsHashMap::new();
    /// assert!(map.insert(3, "a").is_none());
    /// 
    /// map.remove(3);
    /// assert!(map.find(3).is_none());
    /// ```
    pub fn remove(&self, key: i32) {
        let mut write_lock = self.hm.write().unwrap();
        
        write_lock.remove(key);
    }
}

/// A hash map implemented with linear probing.
pub struct HashMap<V> {
    ht: Vec<Item<V>>,
    count: usize
}

#[derive(Clone, PartialEq, Debug)]
enum CellState {
    Empty,
    Filled,
    Deleted
}

impl<V> HashMap<V> {
    /// Creates an empty `HashMap`.
    /// 
    /// # Examples
    ///
    /// ```
    /// use mk_collections::HashMap;
    ///
    /// let mut map = HashMap::<i32>::new();
    /// assert_eq!(map.capacity(), 0);
    /// ```
    pub fn new() -> HashMap<V> {
       HashMap::with_capacity(0)
    }

    /// Creates an empty `HashMap` with the specified capacity.
    /// 
    /// # Examples
    ///
    /// ```
    /// use mk_collections::HashMap;
    ///
    /// let mut map = HashMap::<i32>::with_capacity(10);
    /// assert_eq!(map.capacity(), 10);
    /// ```
    pub fn with_capacity(capacity: usize) -> HashMap<V> {
        HashMap { 
            ht: init_table(capacity),
            count: 0
        }
    }

    /// Gets capacity 
    pub fn capacity(&self) -> usize {
        self.ht.capacity()
    }

    /// Returns a reference to the value corresponding to the key, or [`None`] if it didn't found in the map.
    /// 
    /// # Examples
    ///
    /// ```
    /// use mk_collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// assert!(map.insert_new(3, "a").is_ok());
    /// assert_eq!(*map.find(3).unwrap(), "a");
    /// assert!(map.find(4).is_none());
    /// ```
    pub fn find(&self, key: i32) -> Option<&V> {
        if let Some(found) = self.find_index(key) {
            return Some(&self.ht[found].value);
        } else {
            return None;
        }
    }

    /// Inserts a new key-value pair into the map.
    /// 
    /// If the map already have the key present, it returns error result `DupErr`.
    /// To modify the value of already present key use the put method.
    /// 
    /// # Examples
    ///
    /// ```
    /// use mk_collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// assert!(map.insert_new(3, "a").is_ok());
    /// assert_eq!(*map.find(3).unwrap(), "a");
    /// ```
    pub fn insert_new(&mut self, key: i32, value: V) -> Result<(), DupErr> {
        if self.count == self.ht.capacity() {
            self.resize();
        }

        let res = self.insert_inner(key, value);
        
        self.count += 1;

        res
    }

    /// Returns `true` if the map have this key present, and `false` - otherwise.
    /// 
    /// # Examples
    ///
    /// ```
    /// use mk_collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// assert!(map.insert(3, "a").is_none());
    /// assert!(map.insert(5, "a").is_none());
    /// 
    /// assert!(map.contains_key(3));
    /// assert!(map.contains_key(5));
    /// ```
    pub fn contains_key(&self, key: i32) -> bool {
        self.find_index(key).is_some()
    }

    /// Inserts the new key-value pair if it's not or updates the value if key is aloredy present in the map.
    /// If it updates the old value will be returned, otherwise - [`None`].
    /// 
    /// # Examples
    ///
    /// ```
    /// use mk_collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// assert!(map.insert(3, "a").is_none());
    /// assert_eq!(map.insert(3, "b").unwrap(), "a");
    /// ```
    pub fn insert(&mut self, key: i32, value: V) -> Option<V> {
        if let Some(index) = self.find_index(key) {
            Some(mem::replace(&mut self.ht[index].value, value))
        } else {
            self.insert_new(key, value).expect("cannot insert key-value pair");
            None
        }
    }

    /// Removes a key from the map, returning the value at the key if the key
    /// was previously in the map.
    /// 
    /// # Examples
    ///
    /// ```
    /// use mk_collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// assert!(map.insert(3, "a").is_none());
    /// 
    /// assert_eq!(*map.remove(3).unwrap(), "a");
    /// assert!(map.remove(3).is_none());
    /// ```
    pub fn remove(&mut self, key: i32) -> Option<&V> {
        if let Some(index) = self.find_index(key) {
            self.ht[index].state = CellState::Deleted;
            self.count -= 1;
        
            return Some(&self.ht[index].value);
        } else {
            return None;
        }
    }

    fn find_index(&self, key: i32) -> Option<usize> {
        if self.capacity() == 0 {
            return None;
        }

        let i = self.index(key);
        let item = &self.ht[i];

        if item.key == key && item.state == CellState::Filled {
            return Some(i);
        } else {
            let mut index = self.next_index(i);
            while index != i && {
                        let item = &self.ht[index];
                        ((item.state == CellState::Filled && item.key != key) 
                            || item.state == CellState::Deleted)
                    } {
                index = self.next_index(index);
            }

            if index == i || self.ht[index].state == CellState::Empty {
                return Option::None;
            } else {
                return Option::Some(index)
            }
        }
    }

    fn insert_inner(&mut self, key: i32, value: V) -> Result<(), DupErr> {
        let index = self.index(key);

        if self.ht[index].state == CellState::Filled {
            let item = &self.ht[index];
            if item.key == key {
                return Err(DupErr { key });
            } else {
                let mut index = self.next_index(index);
                while self.ht[index].state == CellState::Filled {
                    if self.ht[index].key == key {
                        return Err(DupErr { key });
                    }
                    index = self.next_index(index);
                }

                self.put_to_index(index, key, value);
            }
        } else {
            self.put_to_index(index, key, value);
        }

        Ok(())
    }

    fn put_to_index(&mut self, index: usize, key: i32, value: V) {
        self.ht[index] = Item { key, value, state: CellState::Filled };
    }

    fn resize(&mut self) {
        let capacity = 
            if self.ht.is_empty() { 1 }
            else { self.capacity() * 2 };

        let ht = init_table(capacity);

        let mut old_ht = mem::replace(&mut self.ht, ht);

        for item in old_ht.drain(..)
                    .enumerate()
                    .filter(|(_, item)| item.state == CellState::Filled)
                    .map(|(_, item)| item) {
            self.insert_inner(item.key, item.value).unwrap();
        }
    }

    fn index(&self, key: i32) -> usize {
        key as usize % self.ht.capacity()
    }

    fn next_index(&self, index: usize) -> usize {
        (index + 1) % self.ht.capacity()
    }
} 

fn init_table<V>(capacity: usize) -> Vec<Item<V>> {
    
    let align = mem::align_of::<Item<V>>();
    let elem_size = mem::size_of::<Item<V>>();

    let num_bytes = capacity * elem_size;
    let ptr = unsafe { alloc::alloc(
        Layout::from_size_align(num_bytes, align)
            .expect("Bad layout")) };

    let mut res = unsafe { Vec::from_raw_parts(ptr as *mut Item<V>, capacity, capacity) };

    for i in 0..capacity {
        res[i].state = CellState::Empty;
    }

    res
}

