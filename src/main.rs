use std::collections::HashMap;

struct Book {
    title: String,
    author: String,
}
struct DVD {
    title: String,
}
fn main() {
    let mut data = HashMap::new();
    data.insert("Chronicles of Narnia".to_string(), "C.S. Lewis".to_string());
}
