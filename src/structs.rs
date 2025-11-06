
struct Person{
  
 // name:&str,//In Rust, if a struct contains a reference (like &str),
           //you must specify a lifetime — otherwise the compiler 
           //doesn’t know how long that reference will be valid.
           
           
  name:String,         
  age:u8,
  
  
}

fn main() {
    let p1 = Person { name: String::from("neel"), age: 20 };
    println!("name is: {}", p1.name);
    println!("age is: {}", p1.age);
}