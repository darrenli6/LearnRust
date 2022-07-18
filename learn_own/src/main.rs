fn main() {
  
    
    // 作用域, 看{}

    let x:i32 =1;
    {
        // 分配在栈上
        let y:i32 =2;
        println!("x ={}",x);
        println!("y ={}",y);

    }
    {
        // 分配在堆上的 编译器不知道它的大小的
        let mut s1=String::from("hello");
       // s1.push_str(" ss");
        println!("s1 = {}",s1); 
        // String类型离开作用域 会调用drop方法
        // 类似C++的析构函数

        let s2=s1;
        println!("s2 = {} ",s2);
        //error value borrowed here after move
        // 移动
        // println!("s1 = {} ",s1);  已经move到s2了 s1报错了 


        // 克隆 clone
       // 相当于c++的深拷贝
        let s3= s2.clone();
        println!("s3 = {}",s3);



    }

    // copy 
    let a =1 ;
    let b=a; 
    // 在栈上的数据 实际就是一个copy 
    println!("a = {} b= {}",a,b);
    // println!("y ={}",y);

    // 所有的整形
    // 浮点型 
    // 字符类型 char
    // 元组 
    // 具有copy的特征
}
