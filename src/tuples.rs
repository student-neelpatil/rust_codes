//1st way


fn main(){
 
 let employee_info:(&str,u8)=("name",20);

 let employee_name:&str=employee_info.0;
 let employee_age:u8=employee_info.1;

 println!("{}",employee_name);
 println!("{}",employee_age);
 
 //or to direct print tuple
 
 println!("{:?}",employee_info); //output-("name", 20)

}

//2nd way-using destructuring


fn main(){
 
 let employee_info:(&str,u8)=("name",20);

//destructuring

 let (employee_name,employee_age)=employee_info;

 println!("{}",employee_name);
 println!("{}",employee_age);
 
 

}