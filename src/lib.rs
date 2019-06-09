mod hash_map;

#[cfg(test)]
mod tests {
    use super::hash_map::*;

    #[test]
    fn hashmap_creation() {
    }

    #[test]
    fn simple_insert_get() {
        let mut hm = HashMap::<&str>::new();

        hm.insert(33, "value: V");

        let value = hm.find(33);

        if let Some(s) = value {
            assert_eq!(*s, "value: V");
        } else {
            assert!(false);
        }
    }

    #[test]
    fn fullfill() {
        let mut hm = HashMap::<i32>::with_capacity(10);

        for i in 0..19 {
            hm.insert(i, i * 4);
        }

        for i in 0..9 {
            let value = hm.find(i);
            if let Some(v) = value {
                assert_eq!(*v, i * 4);
            }
        }

        print!("{:?}", hm.ht);
    }

    #[test]
    fn collision() {
        let mut hm = HashMap::<i32>::with_capacity(10);

        hm.insert(0, 0);
        hm.insert(1, 11);
        hm.insert(2, 22);
        hm.insert(12, 33);
        hm.insert(22, 44);
        hm.insert(5, 55);
        hm.insert(7, 77);

        if let Some(value) = hm.find(12) {
            assert_eq!(*value, 33);
        } else {
            assert!(false);
        }

        if let Some(value) = hm.find(22) {
            assert_eq!(*value, 44);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn remove() {
        let mut hm = HashMap::<i32>::with_capacity(10);

        hm.insert(1, 11);
        hm.insert(2, 22);
        
        match hm.remove(1) {
            None => panic!(),
            Some(removed) => assert_eq!(*removed, 11)
        }

        let found = hm.find(1);
        assert_eq!(found, None);
    }
}
