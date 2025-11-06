
// struct Person{
  
//  // name:&str,//In Rust, if a struct contains a reference (like &str),
//            //you must specify a lifetime — otherwise the compiler 
//            //doesn’t know how long that reference will be valid.
           
           
//   name:String,         
//   age:u8,
  
  
// }

// fn main() {
//     let p1 = Person { name: String::from("neel"), age: 20 };
//     println!("name is: {}", p1.name);
//     println!("age is: {}", p1.age);
// }



//using implementation block
// struct Person{
  
//   name:String,         
//   age:u8,
  
  
// }


// impl Person {
//     // Constructor
//     fn new(name: String, age: u8) -> Self {
//         Self { name, age }
//     }
// }


// fn main() {
//     let p1 = Person::new(String::from("neel"),20);
//     println!("name is: {}", p1.name);
//     println!("age is: {}", p1.age);
// }

//3]using other methods

// struct Person{
  
//   name:String,         
//   age:u8,
  
  
// }


// impl Person {
//     // Constructor
//     fn new(name: String, age: u8) -> Self {
//         Self { name, age }
//     }
    
    
//     fn display(&self){
//       println!("Name: {}, Age: {}", self.name, self.age);
//     }
// }


// fn main() {
//     let p1 = Person::new(String::from("neel"),20);
    
//     p1.display();
// }


//using mutable instance

struct Person{
  
  name:String,         
  age:u8,
  
  
}


impl Person {
    // Constructor
    fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }
    
    
    fn display(&self){
      println!("Name: {}, Age: {}", self.name, self.age);
    }
    
    //mutable function
    fn change(&mut self){
       self.name=String::from("Om prakash");
    }
}


fn main() {
    let mut p1 = Person::new(String::from("neel"),20);
    
    p1.display(); //Name: neel, Age: 20
    p1.change();
    p1.display();//Name: Om prakash, Age: 20
}