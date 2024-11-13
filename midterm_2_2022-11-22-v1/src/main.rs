use std::fmt::Debug;



fn main() {
    let num = 5;
    let text = "what".to_string();

    print_double(&num);        // Expects to print: doubling 5 is 10
    print_double(&text);       // Expects to print: doubling "what" is "whatwhat"

    //ES 2

    let wrapper = Wrapper {
        v: vec![10, 20, 30, 40, 50, 60],
    };

    for val in wrapper.iter_odds() {
        println!("{}", val);
    }

}


//ES2
struct Wrapper{
    v : Vec<i32>,
}

impl Wrapper {
    fn iter_odds(&self) -> OddIndexIterator {
        OddIndexIterator {
            wrapper: self,
            current_index: 1, // Start at the first odd index
        }
    }
}


struct OddIndexIterator<'a>{
    wrapper: &'a Wrapper,
    current_index: usize,
}

impl<'a> Iterator for OddIndexIterator<'a> {
    type Item = &'a i32;

    fn next(&mut self) -> Option<Self::Item> {
        // Increment current_index to point to the next odd index
        self.current_index += 2;

        // Check if the index is within the bounds of the vector
        if self.current_index < self.wrapper.v.len() {
            Some(&self.wrapper.v[self.current_index])
        } else {
            None
        }
    }
}


//ES 1
trait Doublable {
    type Output: Debug;

    fn gimme_double(&self) -> Self::Output;
}

impl Doublable for i32 {
    type Output = i32;

    fn gimme_double(&self) -> Self::Output {
        2 * self
    }
}

impl Doublable for String{
    type Output = String;

    fn gimme_double(&self) -> Self::Output {
        format!("{}{}",self,self)
    }
}

fn print_double<T: Doublable + Debug>(item: &T) {
    println!("doubling {:?} is {:?}", item, item.gimme_double());
}
