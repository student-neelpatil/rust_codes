fn main() {
    // println!("Hello, world!");

    print();  //function call

    //integer datatype
    // let num:u8 = 23;

    // println!("The number is:{}",num);


    //&str type-fixed sized string size
    // let string_literal:&str="hello guys";


    // string_literal.push_str("hii!!!");//wrong because it is fixed size

    // println!("The string is:{}",string_literal);

    //String type-dynaming string size

    // let mut str2:String=String::from("hi i m neel patil");

   //changing the string
    // str2.push_str("welcome");

    // println!("The string2 is:{}",str2);

    let result=add(10,20);

    println!("the sum of two numbers is :{}",result);

}


//function defination main function is entry point for all the function
fn print(){
    println!("my name is neel patil!!!");
}


//returning datatype like enteger or string
fn add(item1:u8,item2:u8)->u8{
  return item1+item2;
}