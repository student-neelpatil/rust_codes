// fn main() {
// //1st way
//     // let v=vec![10,20,30,40];//intialization and declaration

//     // v.push(50);

//     // println!("{:?}",v);

// //2nd way

//     let mut v: Vec<u8> = vec![10, 20, 30, 40, 50, 60];

//     v.pop();

//     println!("{:?}", v);
// }



//passing vector in function and ownership transfer

fn main(){
    let v:Vec<u8>=vec![10,20,30,40];

    insert(v);

    // println!("{:?}",v); //here error occur because v is not remain as owner 

}

fn insert(mut vec:Vec<u8>){
  vec.push(120);
  println!("{:?}",vec);

} //output-[10, 20, 30, 40, 120]



//using borrowing

fn main(){
    let mut v:Vec<u8>=vec![10,20,30,40];

    insert(&mut v);

    println!("{:?}",v);

}

fn insert( vec:&mut Vec<u8>){
  vec.push(120);
  println!("{:?}",vec);

}