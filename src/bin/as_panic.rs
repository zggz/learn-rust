use std::fs::File;

fn main() {
    // panic!("crash and burn");
    //     let v = vec![1, 2, 3];
    //
    //     v[99];
    let f = File::open("./src/bin/as_slice.rs");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("{:?}", error);
        }
    };
}
