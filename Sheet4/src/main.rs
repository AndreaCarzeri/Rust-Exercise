use rand::Rng;

fn main() {
    println!("{:?}", find_equal("lollo", "alelo"));
    println!("{:?}", lucky_slice("lollololololandreaalebusola"));

    //ES2
    let p1 = Person::new("Raffaele".to_string(), None, None);
    let p2 = Person::new("Adelaide".to_string(), None, None);
    let p3 = Person::new("Renata".to_string(), None, None);
    let p4 = Person::new("Barbara".to_string(), Some(&p1), Some(&p2));
    let p5 = Person::new("Michele".to_string(), None, Some(&p3));
    let p6 = Person::new("Andrea".to_string(), Some(&p5), Some(&p4));
    println!("Gen 2 from andrea: {:?}", p6.find_relatives(2));
    println!("Roots from andrea: {:?}", p6.find_roots());
}


//ES2
#[derive(Clone)]
struct Person<'a> {
    name: String,
    father: Option<&'a Person<'a>>,
    mother: Option<&'a Person<'a>>,
}

impl<'a> Person<'a> {
    fn new(name: String, father: Option<&'a Person<'a>>, mother: Option<&'a Person<'a>>) -> Person<'a> {
        Person { name, father, mother }
    }
    fn find_relatives<'b>(&'b self, generations: u32) -> &mut Vec<&'b String> {
        let mut v: Vec<&'b String> = Vec::new();
        v.push(&self.name.to_string());
        let m = self.mother.clone();
        let f = self.father.clone();
        if m.is_some() {
            v.append(m.unwrap().find_relatives(generations - 1));
        }
        if f.is_some() {
            v.append(f.unwrap().find_relatives(generations - 1));
        }

        &mut v
    }

    fn find_roots(&self) -> Vec<String> {
        let mut v: Vec<String> = Vec::new();
        if self.father.is_none() || self.mother.is_none() {
            v.push(self.name.to_string());
        }
        let m = self.mother.clone();
        let f = self.father.clone();
        if f.is_some() {
            v.append(&mut f.unwrap().find_roots());
        }
        if m.is_some() {
            v.append(&mut m.unwrap().find_roots());
        }
        v
    }
}


//ES1
const SIZE_SUB_STR: usize = 2;

fn find_equal<'a, 'b>(s1: &'a str, s2: &'b str) -> Option<(&'a str, &'b str)> {
    for s1_start in 0..(s1.len() - SIZE_SUB_STR + 1) {
        let s1_end = s1_start + SIZE_SUB_STR;
        let s1_slice = &s1[s1_start..s1_end];

        for s2_start in 0..(s2.len() - SIZE_SUB_STR + 1) {
            let s2_end = s2_start + SIZE_SUB_STR;
            let s2_slice = &s2[s2_start..s2_end];
            //println!("s1_slice: {}, s2_slice: {}", s1_slice,s2_slice);
            if s2_slice == s1_slice {
                return Some((s1_slice, s2_slice));
            }
        }
    }
    None
}

fn lucky_slice(input_str: &str) -> Option<&str> {
    let mut rng = rand::thread_rng();
    let mut s_random: String = "".to_string();
    for _index in 0..input_str.len() {
        s_random.push(rng.gen_range(97..=122) as u8 as char);
    }
    println!("Random string: {}", s_random);
    match find_equal(input_str, s_random.as_str()) {
        Some((str1, _)) => Some(&str1),
        _ => None
    }
}
