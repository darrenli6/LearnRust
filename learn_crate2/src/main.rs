// use mylib::factory::produce_refrigerator as A;
// use mylib::factory::produce_refrigerator::produce_re;
// factory所有模块导入
use mylib::factory::*;
fn main() {
    // mylib::factory::produce_refrigerator::produce_re(); // 绝对路径
    // produce_refrigerator::produce_re(); // 通过路径引用
    // A::produce_re();
    // produce_re();
    // produce_refrigerator::produce_re();
    println!("Hello, world!");
}
