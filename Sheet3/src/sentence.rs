pub struct Sentence{
    pub words: Vec<String>
}

impl Sentence{
    pub fn new_default() -> Self{
        Self{words: Vec::new()}
    }

    pub fn new(str: &str) -> Self{
        let it= str.split(' ');
        let mut words = Vec::new();
        for v in it {
            words.push(String::from(v))
        }
        Self{words}
    }
}