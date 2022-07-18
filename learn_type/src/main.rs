fn main() {
    //bool 
    let is_true: bool = true; 
    println!("is_true  = {}",is_true);
    let is_false: bool = false; 
    println!("is_false  = {} , {}",is_false,is_true);

    // char 在rust里面 是32位的  C++ 8位的
    let a = 'a';
    println!(" a = {}",a);

    let b ='你';
    println!(" b = {}",b);

    // i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 
    let c: i8=-11;
    println!(" c = {}",c);


    let d :f32 = 0.0007;
    println!(" d = {}",d);

    // 自适应类型
    // 输出无符号最大
    println!("max = {}",usize::max_value());
    
    // 数组 
    // [type;size] ,size 也是数组类型的一部分
    // size是数组类型的一部分
    let arr: [u32;5] = [1,2,3,4,5];
    println!("arr[0]=  {}",arr[0]);
     
    let arr1: [u32;3] = [1,2,3];
    show(arr1);

    // 元组
    let tup :(i32,f32,char) = (-3,3.34,'年');
    let tup =  (-3,3.34,'年');
    println!("{} ",tup.0);
    println!("{} ",tup.1);
    println!("{} ",tup.2);

    let (x,y,z) = tup;
    println!("{}",x);
    println!("{}",y);
    println!("{}",z);
 
}

fn show(arr:[u32;3]){
    println!("------------");
    for i in &arr{
        println!("{}",i);
    }
    println!("------------");
}
