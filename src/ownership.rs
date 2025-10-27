
fn main(){

    // let str1:String = String::from("this is str1");

    // let str2=str1;

    // println!("the first string is {}",str1); //here str1 transfers his ownership to str2 so str1 is invalid here
    //println!("the second string is {}",str2);

    let x:String=String::from("hello");

    process_string(x);

    //println!("{}",x); // this give error because x is no longer owner

}


//using functions

fn process_string(item:String){//here the new owner of hello is item not x
    println!("The val of x in function is:{}",item);
}