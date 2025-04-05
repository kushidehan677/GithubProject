use std::collections::HashMap;

fn main() {
    let mut data = HashMap::new();

    data.insert(String::from("key1"), String::from("value1"));
    data.insert(String::from("key2"), String::from("value2"));

    for (key, value) in &data {
        println!("Key: {}", key);
        println!("Value: {}", value);
    }
}
