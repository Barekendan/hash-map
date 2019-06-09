use std::{
    alloc::{self, Layout},
    mem
};

#[derive(Clone, Debug)]
pub struct Item<V: Eq + Clone> {
    key: i32,
    value: V
}

const DEFAULT_CAPACITY: usize = 16;
pub struct HashMap<V: Eq + Clone> {
    pub ht: Vec<Item<V>>,
    cap: usize,
    state: Vec<CellState>,
    count: usize
}

#[derive(Clone, PartialEq)]
enum CellState {
    Empty,
    Filled,
    Deleted
}

impl<V: Eq + Clone> HashMap<V> {
    pub fn new() -> HashMap<V> {
       HashMap::with_capacity(DEFAULT_CAPACITY)
    }

    pub fn with_capacity(capacity: usize) -> HashMap<V> {
        HashMap { 
            ht: init_table(capacity),
            cap: capacity,
            state: init_state(capacity),
            count: 0
        }
    }

    pub fn find(&self, key: i32) -> Option<&V> {
        let i = key as usize % self.cap;
        let item = &self.ht[i];

        if item.key == key && self.state[i] == CellState::Filled {
            return Some(&item.value);
        } else {
            let mut index = (i + 1) % self.cap;
            while index != i && 
                ((self.state[index] == CellState::Filled && self.ht[index].key != key) 
                    || self.state[index] == CellState::Deleted) {
                index = (index + 1) % self.cap;
            }

            if index == i || self.state[index] == CellState::Empty {
                return Option::None;
            } else {
                return Option::Some(&self.ht[index].value)
            }
        }
    }

    pub fn insert(&mut self, key: i32, value: V) {
        if self.count == self.cap {
            self.resize();
        }

        let index = key as usize % self.cap;

        if self.state[index] == CellState::Filled {
            let item = &self.ht[index];
            if item.key == key {
                panic!("key duplication");
            } else {
                let mut index = (index + 1) % self.cap;
                while self.state[index] == CellState::Filled {
                    if self.ht[index].key == key {
                        panic!("key duplication");
                    }
                    index = (index + 1) % self.cap;
                }

                self.insert_inner(index, Item { key, value });
            }
        } else {
            self.insert_inner(index, Item { key, value });
        }
    }

    pub fn remove(&mut self, key: i32) -> Option<&V> {
        let i = key as usize % self.cap;
        let item = &self.ht[i];

        if item.key == key && self.state[i] == CellState::Filled {
            self.state[i] = CellState::Deleted;
            self.count -= 1;
            return Some(&item.value);
        } else {
            let mut index = (i + 1) % self.cap;
            while index != i && 
                ((self.state[index] == CellState::Filled && self.ht[index].key != key) 
                    || self.state[index] == CellState::Deleted) {
                index = (index + 1) % self.cap;
            }

            if index == i || self.state[index] == CellState::Empty {
                return Option::None;
            } else {
                self.state[index] = CellState::Deleted;
                self.count -= 1;
                return Option::Some(&self.ht[index].value)
            }
        }
    }

    fn insert_inner(&mut self, index: usize, item: Item<V>) {
        self.ht[index] = item;
        self.state[index] = CellState::Filled;
        self.count += 1;
    }

    fn resize(&mut self) {
        let capacity = 
            if self.cap == 0 { 1 }
            else { self.cap * 2 };

        let ht = init_table(capacity);
        let state = init_state(capacity);

        let mut old_ht = mem::replace(&mut self.ht, ht);
        let old_state = mem::replace(&mut self.state, state);
        mem::replace(&mut self.cap, capacity);

        for item in old_ht.drain(..)
                    .enumerate()
                    .filter(|(index, _)| old_state[*index] == CellState::Filled)
                    .map(|(_, item)| item) {
            self.insert(item.key, item.value);
        }
    }
} 

fn init_table<V: Eq + Clone>(capacity: usize) -> Vec<Item<V>> {
    
    let align = mem::align_of::<Item<V>>();
    let elem_size = mem::size_of::<Item<V>>();

    let num_bytes = capacity * elem_size;
    let ptr = unsafe { alloc::alloc(
        Layout::from_size_align(num_bytes, align)
            .expect("Bad layout")) };

    unsafe { Vec::from_raw_parts(ptr as *mut Item<V>, capacity, capacity) }
}

fn init_state(capacity: usize) -> Vec<CellState> {
    vec![CellState::Empty; capacity]
}

