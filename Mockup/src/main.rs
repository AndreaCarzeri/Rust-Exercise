use std::ops::Add;

fn main() {
    println!("{}",1u8.clone_and_double());
    println!("{}",1i8.clone_and_double());
    println!("{}",2u16.clone_and_double());
    println!("{}",2i16.clone_and_double());
    println!("{}",3u32.clone_and_double());
    println!("{}",3i32.clone_and_double());
    println!("{}",4u64.clone_and_double());
    println!("{}",4i64.clone_and_double());
}

trait CloneAndDouble{
    fn clone_and_double(&self)->Self;
}

impl <T> CloneAndDouble for T where T: Clone + Add<Output = T>{
    fn clone_and_double(&self) -> T {
        self.clone()+self.clone()
    }
}