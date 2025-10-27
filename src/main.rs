fn main() {
    println!("Hello, world!");

    print();

    //integer datatype
    let num:u8 = 23;

    println!("The number is:{}",num);


    //&str type-fixed sized string size
    let mut string_literal:&str="hello guys";


    // string_literal.push_str("hii!!!");//wrong because it is fixed size

    println!("The string is:{}",string_literal);

    //String type-dynaming string size

    let mut str2:String=String::from("hi i m neel patil");

   //changing the string
    str2.push_str("welcome");

    println!("The string2 is:{}",str2);


}

fn print(){
    println!("my name is neel patil!!!");
}