
- entry
  - 获取我们想要检查的键作为参数。entry 函数的返回值是一个枚举
```shell
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Blue")).or_insert(50);
```
- insert
  - 覆盖一个值
- or_insert
  - 只有在没有键的时候再插入
- or_insert_with