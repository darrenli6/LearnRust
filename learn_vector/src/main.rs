// 创建空的vector   Vec<T>
// 创建包含初始值的vector
// 丢弃vector
// 读取元素
// 更新元素
// 遍历
// 使用枚举
fn main() {
    // 1 创建一个空的vector  默认是不可变的
    let mut v: Vec<i32> =Vec::new();
    v.push(1);

    // 2
    let v =vec![1,2,3];


    // 3丢弃vector
    {
        let v1 = vec![1,2,3];
        // 离开花括号进行丢弃
    }
    
    //读取元素
    let one: &i32 = &v[0];
    println!("one = {}",one);
    
    println!("one = {}",*one);

    // 推荐的方法
    // 如果索引取得不存在 容错处理比较好
    match v.get(1){
        Some(value)=> println!("value = {}",value),
        _ => println!("do nothing")
    }
    // 更新元素
    let mut v2:Vec<i32> = Vec::new();
     v2.push(2);
     v2.push(4);
     v2.push(2);

     // 6遍历
     // 不可变的遍历
     for i in &v2{
        println!("i = {} ",i);
     }

     // 可变的遍历
     for i in &mut v2 {
        *i +=1 ;
        println!("i = {}" , i );
     }



     // 7 
     enum Context { 
       Text(String),
       Float(f32),
       Int(i32),
     };
     let c = vec![
        Context::Text(String::from("string")),
        Context::Int(-1),
        Context::Float(0.001),
     ];



     // 8 补充
     let mut v = vec![1,2,3,4,5];
     // 使用了不可变引用
     let first = &v[0];
     // 可变引用 mutable borrow occurs here
     v.push(6);
     //  使用了不可变引用
    // println!("first= {}",first);





 
     


    println!("Hello, world!");
}
