fn main() {
    // 1 定义结构体
    #[derive(Debug)]
    struct User{
        name :String,
        count:String,
        nonce: u64,
        active:bool,
    }



    // 2 创建结构体实例

    let xiaoming  = User{
        name:String::from("xiaoming"),
        count:String::from("323232"),
        nonce: 10000,
        active: true,
    };


    // 3 修改结构体字段

    let mut xiaowang  = User{
        name:String::from("xiaowang"),
        count:String::from("11222"),
        nonce: 1000,
        active: false,
    };

    println!("xiaowang  -- {:?}",xiaowang);
    println!("xiaowang  -- {:#?}",xiaowang);

    xiaowang.active=true;



    // 4 参数名字和字段名字同名的简写方法

    let name =String::from("xiaoxiao");
    let count =String::from("888999");
    let nonce =20000;
    let active =false;


    let user1 = User{
        name,
        count,
        nonce,
        active 
    };



    // 5 从其他结构体创建实例

    let user2 = User {
        name:String::from("user2"),
        ..user1
    

    };

    println!("user2 name= {} ",user2.name);


    // 6元组结构体  
    struct Point(i32,i32);

    let a =Point(10,20);
    let b =Point(30,11);

    println!("a.x = {}  a.y={} ",a.0,a.1);




    // 7 没有任何字段类单元结构体

    struct A{};



    // 8 打印结构体








    println!("Hello, world!");
}
