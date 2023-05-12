fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let r = largest(&number_list);
    println!("r is {}", r);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

fn largest<T: PartialOrd >(list: &[T]) -> &T {
    let mut r = &list[0];

    for i in list {
        if i > r {
            r = i;
        }
    }
    r
}
