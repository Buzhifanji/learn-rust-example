fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // 编译器会对未使用的变量绑定产生警告；可以给变量名加上下划线前缀来消除警告。
    let _unused_variable = 3u32;
    let _noisy_unused_variable = 2u32;

    let _immutable_binding = 1; // 变量绑定默认是不可变的
    let mut mutable_binding = 1; // 加上 mut 修饰语后变量就可以改变

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding)
    // _immutable_binding += 1
}
