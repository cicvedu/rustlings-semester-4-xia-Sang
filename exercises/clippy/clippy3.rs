// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.


#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // my_option.unwrap();
        println!("The option is None, cannot unwrap.");

    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // 使用 resize 方法时要注意，它修改向量本身而不返回新的向量
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    // my_empty_vec.resize(0, 5);
    my_empty_vec.clear(); // Use clear() to empty the vector
    println!("This Vec is empty, see? {:?}", my_empty_vec);
    // println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 交换这两个值的正确方法是使用临时变量或 mem::swap 函数
std::mem::swap(&mut value_a, &mut value_b); // Correctly swap value_a and value_b

    println!("value a: {}; value b: {}", value_a, value_b);
}
