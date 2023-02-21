use super::*;

fn check<T: Map<String, f32>>(f: fn(Vec<DictEntry<String, f32>>) -> T) {
    let map = f(vec![
        DictEntry {
            key: String::from("k1"),
            value: 1.0,
        },
        DictEntry {
            key: String::from("k2"),
            value: 2.2,
        },
        DictEntry {
            key: String::from("k4"),
            value: 4.4,
        },
    ]);

    assert_eq!(map.get(&String::from("k0")), None);
    assert_eq!(map.get(&String::from("k1")), Some(1.0));
    assert_eq!(map.get(&String::from("k2")), Some(2.2));
    assert_eq!(map.get(&String::from("k3")), None);
    assert_eq!(map.get(&String::from("k4")), Some(4.4));
    assert_eq!(map.get(&String::from("k5")), None);
}

#[test]
fn check_brute() {
    check(BruteMap::from);
}
