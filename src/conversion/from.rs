use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

// 自定义类型转换
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My num is {:?}", num.value);
}
