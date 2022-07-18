// fn other_fun(){
//     println!("this is a function ")
// }

// fn other_fun1(a:i32,b:u32){
//     println!("a = {}, b = {}",a,b);
// }

// // 函数参数推导,不可以推导

// // fn other_fun2(a,b){
// //     println!("a = {}, b = {}",a,b);
// // }

// fn other_fun2(a:i32,b:i32) -> i32{
//    let result = a +b;
//    return result;
// }


// // 不能用；号  
// fn other_fun3(a:i32,b:i32) -> i32{
//     let result = a +b;
//      result
//  }
 
//  fn other_fun4(a:i32,b:i32) -> i32{
//     a +b
    
//  }

 fn takes_ownership(some_string : String) -> String {
    println!("{}", some_string);
    some_string
 }

 fn make_copy(i:i32){
    println!(" i = {}",i  );
 }


 fn main(){

    let s = String::from("hello");
    // s 回收到是到takes_ownership这个函数的{}
    let s1=takes_ownership(s);
    //value borrowed here after move error
    // println!("{}",s);
    
     println!("{}",s1);

    let x =5 ;
    make_copy(x);
    // 分配栈上的 具有copy 
    println!("x={}",x);

 }

// fn main1() {

//     other_fun();
//     let a:i32 =-1;
//     let b:u32 =2;
//     other_fun1(a,b);

//     let c:i32 =9;
//     let r:i32= other_fun2(a,c);
//     println!("r = {}",r);

//     let r2:i32= other_fun4(a,c);
//     println!("r2 = {}",r2);

//    // 语句是执行一些操作
//    // let y=1 ;  // 语句 没有返回值
   
//    // 表达式会计算一些值
//    let y = {
//      let x = 1; 
//      x+1
//    };
//    println!(" y = {}",y);




//     println!("Hello, world!");
 
// }


