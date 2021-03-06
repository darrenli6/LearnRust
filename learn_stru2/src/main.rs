
#[derive(Debug)]
struct Dog{
    name:String,
    weight:f32,
    height:f32,
}

impl Dog{
    fn get_name(&self) ->&str{
        &(self.name[..])
    }

    fn get_weight(&self) ->f32{
        self.weight
    }

 

    fn show(){
        println!("oh oh ");
    }


}
impl Dog{
    fn get_height(&self) ->f32{
        self.height
    }
}

fn main() {

    let dog =Dog { 
        name:String::from("jack"),
        weight:23.22,
        height:22.11,
    };

    println!("dog  {:#?}",dog);

    println!("name  = {} ",dog.get_name());
    println!("weight  = {} ",dog.get_weight());
    println!("height  = {} ",dog.get_height());
    Dog::show();
    println!("Hello, world!");
}
