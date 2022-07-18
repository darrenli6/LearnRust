// 1 在任意给定时间 要么只能有一个可变引用 要么只能由多个不可变引用
// 有了可变引用 不能再有不可变引用
//2 引用必须有效
fn main() {
    let ref_s = dangle();
    println!("Hello, world!");
}

fn dangle()-> &String {
    // s的作用域是到 }

    let s = String::from("hello");
    &s
}