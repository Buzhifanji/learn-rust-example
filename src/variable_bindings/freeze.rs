fn main() {
    let mut _mutable_integer = 7i32;

    {
        // // 被不可变的 `_mutable_integer` 遮蔽
        let _mutable_integer = _mutable_integer;

        // _mutable_integer = 50;
    }

    _mutable_integer = 3;
}
