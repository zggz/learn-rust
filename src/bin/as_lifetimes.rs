// 生命周期
/*
生命周期语法是用于将函数的多个参数与其返回值的生命周期进行关联的。
一旦他们形成了某种关联，Rust 就有了足够的信息来允许内存安全的操作并阻止会产生悬垂指针亦或是违反内存安全的行为。
 */
fn main() {
    let str1 = String::from("ab");
    let str2 = "xyz";
    let r = longest(str1.as_str(), &str2);
    println!("{}", r);
}

fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}
