//infinite loop 
fn main(){

    loop{
        println!("hello");
    }
}

//while loop

fn main(){

    let mut count:u16=0;

    while count<20{
        println!("count is:{}",count);

        count=count+1;
    }
}


//for-loop-generally it is used for array 

fn main(){

    let array1:[u8;5];
    array1=[10,20,30,40,50];

    for element in &array1{
        println!("The element is :{}",element);
    }

} 
