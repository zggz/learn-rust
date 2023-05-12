
- unwrap
  - 值为OK 返回， Err调用panic!
- expect
  - 可以选择性的错误信息
```shell
 let f = File::open("hello.txt").expect("Failed to open hello.txt");
```
- 传播错误
  - 返回 Result<String, io::Error> 
- 传播错误的简写：? 运算符
  - ? 运算符可被用于返回 Result 的函数（注意函数的返回值类型）
  - ? 运算符所使用的错误会被返回
```shell

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```