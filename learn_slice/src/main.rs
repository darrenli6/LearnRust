// 1 字符串slice是String中一部分值的引用
// 2 字面值就是slice
// 3 其他类型是slice

fn main() {
    let s= String::from("hello world");
    // 左开右闭
    let h = &s[0..5];
    let h = &s[0..=4];
    let h = &s[..=4];
    let h = &s[..5];
    println!(" h = {}",h);

    let w = &s[6..11];
    let w = &s[6..=10];
    let w = &s[6..];
    println!(" w = {}",w);


    // let ss =String::from("你好");
    // let w1 =&ss[0..1];

    // 字面值存储在二进制中的  // &str 不可变的引用
    let s3 ="hh";

    let a = [1,2,4,5];
    let sss =&a[1..3];
    println!("sss[0] = {}",sss[0]);


    println!("Hello, world!");
}
