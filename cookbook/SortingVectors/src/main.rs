fn main() {
    // 整数排序
    let mut vec = vec![1, 5, 10, 2, 15];
    vec.sort();
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);

    // 浮点f64排序
    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];
    // vec.sort(); 直接sort是不行的，为什么这么设计呢？
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(vec, vec![1.1, 1.123, 1.15, 2.0, 5.5]);

    let mut people = vec![
        Person::new("Zoe", 18),
        Person::new("Al", 20),
        Person::new("John", 19),
    ];
    people.sort();
    assert_eq!(people, vec![
        Person::new("Al", 20),
        Person::new("John", 19),
        Person::new("Zoe", 18),
    ]);
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    pub fn new(name: &str, age: u8) -> Self {
        Person { name: name.to_string(), age }
    }
}