// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // 修复：不要在已知为None的情况下使用unwrap
    if let Some(val) = my_option {
        // 处理Some情况
    }

    let my_arr = &[
        -1, -2, -3,  // 添加了缺失的逗号
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // 修复：创建一个正确的空向量
    let mut my_empty_vec = vec![1, 2, 3, 4, 5];
    my_empty_vec.clear();
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    // 修复：使用std::mem::swap正确交换值
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
