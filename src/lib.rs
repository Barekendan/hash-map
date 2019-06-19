#[allow(dead_code)]
pub mod hash_map;

pub use self::hash_map::HashMap;

#[cfg(test)]
mod tests {
    use super::hash_map::*;

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
        hm.insert(1, 11).unwrap();
        hm.insert(2, 22).unwrap();
        hm.insert(3, 33).unwrap();
        assert_eq!(hm.capacity(), 4);
    }

    #[test]
    fn test_insert_find() {
        let mut hm = HashMap::new();

        hm.insert(33, "value: V").unwrap();

        let value = hm.find(33).unwrap();
        assert_eq!(*value, "value: V");

        assert!(hm.find(22).is_none());
    }

    #[test]
    fn test_contains_key() {
        let mut hm = HashMap::new();

        hm.insert(33, "value: V").unwrap();

        assert!(hm.contains_key(33));
        assert!(!hm.contains_key(22));
    }

    #[test]
    fn test_insert_duplicate() {
        let mut hm = HashMap::with_capacity(2);
        hm.insert(4, 14).unwrap();
        hm.insert(1, 11).unwrap();
        hm.insert(2, 12).unwrap();
        hm.insert(3, 13).unwrap();
        
        assert!(hm.insert(4, 4).is_err());
        assert!(hm.insert(3, 3).is_err());
        assert!(hm.insert(2, 2).is_err());
    }

    #[test]
    fn test_put() {
        let mut hm = HashMap::with_capacity(2);

        hm.insert(3, 13).unwrap();
        hm.insert(0, 10).unwrap();

        let old = hm.put(3, 33).unwrap();
        assert_eq!(old, 13);

        hm.insert(1, 11).unwrap();
        hm.insert(2, 12).unwrap();
        hm.insert(4, 14).unwrap();

        hm.put(2, 22);        

        assert_eq!(*hm.find(3).unwrap(), 33);
        assert_eq!(*hm.find(2).unwrap(), 22);
    }

    #[test]
    fn test_remove() {
        let mut hm = HashMap::with_capacity(2);

        hm.insert(1, 11).unwrap();
        hm.insert(2, 22).unwrap();

        let removed = hm.remove(1).unwrap();
        assert_eq!(*removed, 11);
        assert!(hm.find(1).is_none());
        assert!(hm.remove(1).is_none());

        assert_eq!(*hm.find(2).unwrap(), 22);
    }
}
