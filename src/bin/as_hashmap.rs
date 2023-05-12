use std::collections::HashMap;

fn main() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();

    map.insert(field_name, field_value);

    let field_name = String::from("Favorite color");

    let v = map.get(&field_name);

    println!("{}", v.unwrap());

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
}
