

// fn main() {
//     let s1= gives_ownership();

//     println!("s1 = {}" ,s1);
//     let mut s2= String::from("hello ");
//     let s3 = takes_and_give_back(s2);
//     let s2 = takes_and_give_back(s3);

//     println!("s2 = {}",s2);
//     println!("Hello, world!");
// }

// fn gives_ownership() -> String {
//     let s = String::from("hello");
//     s
//    //调用drop
//    // 如果有人引用 将所有权转移到s1 
// }

// fn takes_and_give_back(s: String) ->String{
//     s
// }

// 引用
// &
// 让我创建一个指向值的引用，但是我们不拥有它 
// 当引用离开值指向的作用域之也不会被丢弃。
// 借用
fn calcute_length(s :&String)  -> usize{
    s.len()
}

fn modify_s(s: &mut String){
    s.push_str(", world");
}
fn main(){
   let mut s1 =String::from("hello");
//    let s=&s1;
//    let len =calcute_length(s);
//    println!("s1 = {} ",s1);
//    println!("len = {}",len);

//    let ms = &mut s1;
   // 借用 &mut
//    modify_s(ms);

   //println!("s1 = {}",s1);
   //println!("s = {}",s);
   //println!("ms = {}",ms);



   let r1= &s1;
   let r2= &s1;
   
   println!("{} {} ",r1,r2);

   let r3= &mut s1;
   r3.push_str(", world");
  



}

