// use learn_rust::guess_game;

fn main() {
    // guess_game::start_game();
    let n = fibonacci(5);
    println!("{}", n)
}

fn fibonacci(n: i32) -> i32 {
    match n {
        i if i < 2 => n,
        _ => fibonacci(n - 1) + fibonacci(n - 2)
    }
}
