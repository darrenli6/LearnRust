

fn main() {

    // 1 创建一个空string
    let mut s0= String::new();
    s0.push_str("hello");
    println!("s0 = {}",s0);

    // 2通过字面值创建一个String 
    let s1 = String::from("init some thing");
    println!("s1 = {} ",s1);

    let s1 = "init some thing1".to_string();

    println!("s1 = {} ",s1);




    // 2.1 使用String::from()


    // 2.2 使用str的方式


    // 3 更新String

    let mut s2 = String::from("hello");
    s2.push_str(", world ");

    let ss =" !".to_string();
    // 引用
    s2.push_str(&ss);
    println!("s2 = {} ",s2);

    println!("ss = {} ",ss);
    // 3.1 push_str 


    // 3.2 push 

 


    let mut s2=String::from("tea");
    // 只能用单引号  只能有一个字符
    s2.push('m');
    println!("s2 ={} ",s2);


    let s1 = "hello".to_string();
    let s2= String::from(",world ");
 

    // s1将所有权给了s3
    // 引用不会被获取所有权的
    let s3 = s1 + &s2;
    println!("s3 ={}",s3); 
    //value borrowed here after move
    // println!("s1 ={}",s1); 
     println!("s2 ={}",s2); 

     let s341 = String::from("tic");
     let s342 = String::from("tac");
     let s343 = String::from("toe");

     let s344 = format!("{}-{}-{}",s341,s342,s343);
     println!("s344 = {} ",s344);
     println!("s341 = {} ",s341);

    
     let s4= String::from("hello");
    //  let s41 = s4[0];
     println!("s4.len = {} " , s4.len());
  
     let s4 =String::from("你好");
     println!("s4.len= {}" , s4.len());



    let hello = "你好";
    // slice方式是可以的
    let h5 = &hello[0..3];
    println!(" h5 = {} ",h5);

     //thread 'main' panicked at 'byte index 2  is not a char boundary

    // let h6 = &hello[0..2];
    // println!(" h6 = {} ",h6);

    // chars 
    for c in s4.chars(){
        println!("c = {} ",c);
    }

    for b in s4.bytes(){
        println!("b = {} ",b);
    }
    println!("Hello, world!");
}
