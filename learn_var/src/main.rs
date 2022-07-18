// 定义常量
const MAX_POINT :u32=1000000;

fn main() {
    
     //1. 变量定义
     // 定义变量用let 如果变量没有mut的话 那么是不可变的
     // let name:type =value
     let a =1;
   
    println!("a = {}",a);

    let mut b: u32 =2 ;
    println!("b = {}",b);

    // 修改变量 , 修改 可以用mut
    // 可变性和不可变性
    b=3;
    println!("b = {}",b);

    // 2. 隐藏性 后面的变量将前面的变量隐藏掉了

    let b: f32 =1.1;
    println!("b = {}",b);


    // 常量
    println!("max_pint = {}",MAX_POINT);

}
