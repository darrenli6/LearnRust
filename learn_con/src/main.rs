fn main() {
     let y=2;
     if y==1{
        println!("y =1 ")
     }

     // if else

     if y==1{
        println!("y==1");
     }else{
        println!("y!=1");
     }

     // 多个if else
     let y=100;
     if y==1{
        println!("y==1");
     }else if y==0{
        println!("y==0");
     }else if y==-1{
        println!("y==-1");
     }else{
        println!("other");
     }


     // let中使用if 
     let condition =true;
     let x =if condition {
        5
     }else{
        6
        // "sixe"  error
     };
     println!("x ={}",x);


       // 循环
       let mut count = 0;
    loop {
        println!("loop ");
        if count ==10{
             break ;
        }
        count = count+1; //  count+=1;
    }


    let result = loop {
        count +=1;
        if count ==20 {
            break count*2;
        }
    };

    println!("result = {}",result);
  

    // while 循环
    let mut i = 0;
    while i!=10{
        i+=1;
    };
    println!("i = {}",i);


    // for 
    let arr:[u32;5] =[1,2,3,4,5];
    // for element in arr.iter() {
        for element in &arr {
        println!("element = {}",element);
    }

}
