enum Message {
    Quit,
    Move { x: u32, y: u32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
fn main() {
    let mut v = vec![1, 2];

    v.push(3);
    let f = &v[0];

    println!("{}", f)
}
