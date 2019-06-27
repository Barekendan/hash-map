#[allow(dead_code)]
pub mod hash_map;

pub use self::hash_map::HashMap;
pub use self::hash_map::TsHashMap;

extern crate owning_ref;

#[cfg(test)]
mod tests {
    use super::hash_map::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn spam_insert() {
        let m = Arc::new(TsHashMap::new());
        let mut joins = Vec::new();

        for t in 0..10 {
            let m = m.clone();
            joins.push(thread::spawn(move || {
                for i in t * 1000..(t + 1) * 1000 {
                    assert!(m.insert(i, !i).is_none());
                    assert_eq!(m.insert(i, i).unwrap(), !i);
                }
            }));
        }

        for j in joins.drain(..) {
            j.join().unwrap();
        }

        for t in 0..5 {
            let m = m.clone();
            joins.push(thread::spawn(move || {
                for i in t * 2000..(t + 1) * 2000 {
                    assert_eq!(*m.find(i).unwrap(), i);
                }
            }));
        }

        for j in joins {
            j.join().unwrap();
        }
    }

    #[test]
    fn test_create() {
        let hm = HashMap::<&str>::new();
        assert_eq!(hm.capacity(), 0);

        let hm = HashMap::<&str>::with_capacity(12);
        assert_eq!(hm.capacity(), 12);
    }

    #[test]
    fn test_resize() {
        let mut hm = HashMap::with_capacity(2);
        assert_eq!(hm.capacity(), 2);
        hm.insert(1, 11);
        hm.insert(2, 22);
        hm.insert(3, 33);
        assert_eq!(hm.capacity(), 4);
    }

    #[test]
    fn test_insert_find() {
        let mut hm = HashMap::new();

        assert!(hm.insert(33, "value: V").is_none());

        let value = hm.find(33).unwrap();
        assert_eq!(*value, "value: V");

        assert!(hm.find(22).is_none());
    }

    #[test]
    fn test_contains_key() {
        let mut hm = HashMap::new();

        assert!(hm.insert(33, "value: V").is_none());

        assert!(hm.contains_key(33));
        assert!(!hm.contains_key(22));
    }

    #[test]
    fn test_insert_duplicate() {
        let mut hm = HashMap::with_capacity(2);
        hm.insert(4, 14);
        hm.insert(1, 11);
        hm.insert(2, 12);
        hm.insert(3, 13);
        
        assert!(hm.insert_new(4, 4).is_err());
        assert!(hm.insert_new(3, 3).is_err());
        assert!(hm.insert_new(2, 2).is_err());
    }

    #[test]
    fn test_insert() {
        let mut hm = HashMap::with_capacity(2);

        assert!(hm.insert(3, 13).is_none());
        assert!(hm.insert(0, 10).is_none());

        let old = hm.insert(3, 33);
        assert_eq!(old, Some(13));

        assert!(hm.insert(1, 11).is_none());
        assert!(hm.insert(2, 12).is_none());
        assert!(hm.insert(4, 14).is_none());

        hm.insert(2, 22);        

        assert_eq!(*hm.find(3).unwrap(), 33);
        assert_eq!(*hm.find(2).unwrap(), 22);
    }

    #[test]
    fn test_remove() {
        let mut hm = HashMap::with_capacity(2);

        assert!(hm.insert(1, 11).is_none());
        assert!(hm.insert(2, 22).is_none());

        let removed = hm.remove(1).unwrap();
        assert_eq!(*removed, 11);
        assert!(hm.find(1).is_none());
        assert!(hm.remove(1).is_none());

        assert_eq!(*hm.find(2).unwrap(), 22);
    }
}
