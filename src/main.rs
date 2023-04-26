// use learn_rust::guess_game;

fn main() {
    let slice = ['l', 'o', 'r', 'e', 'm'];
    let iter = slice.chunks(2);
    // let string: str = "banana";
    let arr = [1,2,3,4];
    let arr_s = arr.as_slice();

    for i in iter {
        println!("{:?}", i)
    }
}
