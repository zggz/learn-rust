use wasm;
fn main() {
    let c = wasm::add(1, 3);
    println!("Hello, world! {}", c);
}
