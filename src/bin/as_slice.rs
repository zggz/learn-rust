fn main() {
    let mut string = String::from("hello wolrd");
    let sub_str = first_word(&string);
    println!("sub_str is {}", sub_str);

    string.clear();

    // let  str1 = &string[..2];
    // println!("str1 is {}", str1);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
