mod math_utils {
    // This function is private by default
    fn square(x: i32) -> i32 {
        x * x
    }

    // `pub` makes it public so it can be accessed outside
    pub fn cube(x: i32) -> i32 {
        x * x * x
    }
}

fn main() {
    // println!("{}", math_utils::square(3));
    println!("{}", math_utils::cube(3)); 
}