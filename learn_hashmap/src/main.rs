/*
1 HashMap<K,V>
2 创建hashmap
3 读取
4遍历
5更新

*/
use std::collections::HashMap;



fn main() {

    let mut scores:HashMap<String,i32> = HashMap::new();
    scores.insert(String::from("Blue"),32);
    scores.insert(String::from("red"),30);
    scores.insert(String::from("yellow"),10);

    let keys = vec![
        String::from("Blue"),
        String::from("Red"),
        ];
    let values =vec![10,2];
    
    let scores: HashMap<_,_> = keys.iter().zip(values.iter()).collect();
    
    let k =String::from("Blue");
    // option
    // if let Some(v) = scores.get(&k){
    //     println!("v= {}" ,v );
    // }

    let k = String::from("yellow");
    let v =scores.get(&k);

    match v{
        Some(value) => println!("v = {}",value),
        None => println!("None"),
    }

    println!("-----");
    //遍历hashmap
    for(key,value) in &scores {
        // 顺序是不一定的 
        // 遍历的时候任意的顺序
        println!("{} , {} ",key,value);
    }

    println!("-----");

    // 插入值 
    let mut ss = HashMap::new();
    ss.insert(String::from("one"),1);
    ss.insert(String::from("two"),2);
    ss.insert(String::from("three"),3);
    ss.insert(String::from("one"),3);


    println!("-----{:#?}",ss);
    //遍历hashmap
    for(key,value) in &ss {
        // 顺序是不一定的 
        // 遍历的时候任意的顺序
        println!("{} , {} ",key,value);
    }

    // 键不存在的时候插入

    let mut ss1 = HashMap::new();
    ss1.insert(String::from("one"),1);
    ss1.insert(String::from("two"),2);
    ss1.insert(String::from("three"),3);
    ss1.entry(String::from("one")).or_insert(3);
    println!("-----{:#?}",ss1);

    // 根据旧值更新一个新值

    let text= "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace(){
       let count = map.entry(word).or_insert(0);
       *count +=1;
    }

    println!(" {:?}" ,map);






    println!("-----");





    
   





    println!("Hello, world!");
}
