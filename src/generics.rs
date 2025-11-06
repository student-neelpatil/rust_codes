
//generics with function

fn printvalue<T>(x:T)->T{
     
     return x;
    
}

fn main(){
  
  println!("{}",printvalue(20));
}