fn main() {
    // 因为有类型说明，编译器知道 `elem` 的类型是 u8。
    let elem = 5u8;

    // 创建一个空向量（vector，即不定长的，可以增长的数组）。
    let mut vec = Vec::new();

    vec.push(elem);
    // 现在编译器知道 `vec` 是 u8 的向量了（`Vec<u8>`）

    println!("{:?}", vec);
}
