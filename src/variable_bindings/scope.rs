fn main() {
    let long_lived_binding = 1;

    // 块级作用域
    {
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);

        let long_lived_binding = 5_f32;

        println!("inner long: {}", long_lived_binding);
    }

    // println!("outer short: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);

    let long_lived_binding = 'a';

    println!("outer long: {}", long_lived_binding);

    let mut outer_var = 1;

    {
        outer_var = 2;
        println!("inner_var: {}", outer_var);
    }
    println!("outer_var: {}", outer_var);
}
