mod hash_map;


use hash_map::*;

fn main() {
    let mut hm = HashMap::<i32>::with_capacity(10);

    hm.insert(-1, -11);
    hm.insert(1, 11);
    hm.insert(2, 22);
    hm.insert(12, 33);
    hm.insert(3, 44);
    hm.insert(5, 55);
    hm.insert(7, 77);
    hm.insert(32, 4);
    hm.insert(42, 4);
    hm.insert(22, 44);
    hm.insert(21, 21);

    println!("{:?}", hm.ht);

    if let Some(value) = hm.find(-1) {
        assert_eq!(*value, -11);
    } else {
        assert!(false);
    }

    if let Some(value) = hm.find(22) {
        assert_eq!(*value, 44);
    } else {
        assert!(false);
    }

    
    if let Some(value) = hm.find(21) {
        assert_eq!(*value, 21);
    } else {
        assert!(false);
    }

    // hm.remove(21);
    match hm.remove(21) {
        None => panic!(),
        Some(removed) => assert_eq!(*removed, 21)
    }
    
    let found = hm.find(21);
    assert_eq!(found, None);

}