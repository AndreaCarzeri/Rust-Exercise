use std::fmt::Debug;


trait Unknown {
    fn serialize(&self) -> String;
}


fn main() {
    let mut v = get_vec();
    v.push(Box::new("hiii!".to_string()));
    v.push(Box::new(-587));
    v.push(Box::new("xyz".to_string()));
    v.push(Box::new(vec![4, 5, 6]));
    print_vec(&v);
}

impl Unknown for i32{
    fn serialize(&self) -> String {
        self.to_string()
    }
}

impl <T: Debug> Unknown for Vec<T>{
    fn serialize(&self) -> String {
        format!("{:?}", self)
    }
}

impl Unknown for String{
    fn serialize(&self) -> String {
        self.clone()
    }
}

fn get_vec() -> Vec<Box<dyn Unknown>>{
    Vec::new()
}

fn print_vec(v: &Vec<Box<dyn Unknown>>){
    for t in v.iter().clone(){
        println!("{}",t.serialize())
    }
}