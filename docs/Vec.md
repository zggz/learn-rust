
### push

### as_slice
将一个可变数组合不可变数组转换为一个切片

### chunks
从切片的开头开始，以（不重叠）块（一次为 chunk_size 元素）的形式在切片上进行迭代的迭代器。
```rust
fn main(){
  let slice = ['l', 'o', 'r', 'e', 'm'];
  let iter = slice.chunks(2);
  for i in iter {
    println!("{:?}", i);
    // ['l', 'o']
    // ['r', 'e']
    // ['m']
  }
}
```