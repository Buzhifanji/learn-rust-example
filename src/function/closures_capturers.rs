fn main() {
    use std::mem;
    let color = String::from("green");
    let print = || println!("color: {}", color);

    print();
    // 不可变借用
    let _color_moved = &color;
    print();
    // 不可变移动
    let _color_moved = color;

    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };
    // 可变 不能借用
    // let _reborrow = &count;
    inc();

    // 闭包 重新借用
    let _count_reborrowed = &mut count;

    let movable = Box::new(3);

    // `mem::drop` 要求 `T` 类型本身，所以闭包将会捕获变量的值。这种情况下，
    // 可复制类型将会复制给闭包，从而原始值不受影响。不可复制类型必须移动
    // （move）到闭包中，因而 `movable` 变量在这里立即移动到了闭包中。
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };
    // `consume` 消耗了该变量，所以该闭包只能调用一次。
    // println!("`movable`: {:?}", movable);
    consume();
    // consume();

    // `Vec` 在语义上是不可复制的。
    let haystack = vec![1, 2, 3];

    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    // println!("There're {} elements in vec", haystack.len());
    // ^ 取消上面一行的注释将导致编译时错误，因为借用检查不允许在变量被移动走
    // 之后继续使用它。
}
