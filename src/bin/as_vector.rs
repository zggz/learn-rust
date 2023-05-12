use std::collections::HashMap;

fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 5];
    let mut total = 0;
    for i in &mut v {
        total += *i;
    }
    println!("average is {}", total / v.len());

    let mut h = HashMap::new();
    for i in &mut v {
        let c = h.entry(*i).or_insert(0);
        *c += 1;
    }

    println!("{:?}", h);
}
