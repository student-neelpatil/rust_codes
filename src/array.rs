


fn main(){

    let mut array:[&str;5];//array declaration  

    array=["om","jay","ramesh","sam","john"];

    println!("{:?}",array);

    // Output:

    // ["om", "jay", "ramesh", "sam", "john"]

    array[2]="neel";

     println!("{:?}",array);

     println!("{:?}",array.len());



}

