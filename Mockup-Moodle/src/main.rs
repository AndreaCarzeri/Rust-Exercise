use std::cmp::PartialEq;
use crate::AirplaneCompany::Boeing;

pub fn main() {
    let mut fleet = AirFleet {
        fleet: Vec::new(),
    };

    let airplane1 = Airplane {
        company: AirplaneCompany::Airbus,
        model: "A380".to_string(),
    };

    let airplane2 = Airplane {
        company: AirplaneCompany::Boeing,
        model: "747".to_string(),
    };

    let airplane3 = Airplane {
        company: AirplaneCompany::Airbus,
        model: "A320".to_string(),
    };

    fleet.add_airplane(airplane1);
    fleet.add_airplane(airplane2);
    fleet.add_airplane(airplane3);

    println!("{:?}", fleet.search_airplane("A380"));
    println!("{:?}", fleet.search_airplane("747"));
    println!("{:?}", fleet.search_airplane("A320"));
    println!("{:?}", fleet.search_airplane("A330"));
}

#[derive(PartialEq, Debug)]
enum AirplaneCompany {
    Boeing,
    Airbus,
}

struct Airplane {
    company: AirplaneCompany,
    model: String,
}

struct AirFleet {
    fleet: Vec<Airplane>,
}

impl AirFleet {
    fn remove_boeing(&mut self) {
        let mut new_list: Vec<Airplane> = Vec::new();
        for a in &self.fleet {
            if a.company != Boeing {
                new_list.push(a.);
            }
        }
        self.fleet = new_list;
    }

    fn add_airplane(&mut self, a: Airplane){
        self.fleet.push(a);
    }

    fn search_airplane(&mut self, model: &str) -> Result<AirplaneCompany,String>{
        for a in self.fleet {
            if a.model==model {
                return Ok(a.company);
            }
        }
        Err("Not in this fleet".to_string())
    }
}



/*use std::fmt;
use std::fmt::{format, write, Formatter};
use std::ops::Index;

pub fn main() {
    let s1 = Student::new("marco", 1);
    let s2 = Student::new("anto", 2);
    let s3 = Student::new("anna", 3);
    let mut university = University::new("Trento", &vec![s1, s2, s3]);

    println!("{}", university);

    println!("{}", university.remove_student(1).unwrap().id);
}



#[derive(Clone, Debug)]
struct Student{
    name: String,
    id: u32
}

impl Student{
    fn new(str: &str, id: u32) -> Self{
        Self{name: str.to_string(), id}
    }
}

impl fmt::Display for Student{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"Student {{ name: \"{}\", id: {} }}", self.name, self.id)
    }
}

struct University{
    name: String,
    vec: Vec<Student>
}

impl University{
    fn new(str: &str,vec: &[Student]) -> Self{
        let mut students = Vec::from(vec);
        Self{name: str.to_string(), vec: students}
    }
    fn remove_student(&mut self, id: u32) -> Result<Student, &str>{
        let mut cont=0;
        let max = &self.vec.len();
        let mut trovato = false;
        let mut s: Student = Student::new("",0);
        while cont < *max {
            s = self.vec.get(cont).unwrap().clone();
            if s.id == id {
                trovato=true;
                break;
            }
            cont+=1;
        }

        if trovato {
            self.vec.remove(cont);
            return Ok(s.clone());
        }
        Err("Not found")
    }
}

impl fmt::Display for University{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        s.push_str(format!("{}\n",self.name).as_str());
        s.push_str(format!("{:?}",self.vec).as_str());
        s.push_str("Students: ");
        write!(f,"{s}")
    }
}




/*
pub fn main() {
    let person1: NameSurname = NameSurname {
        name: "Ernesto".to_string(),
        surname: "Bianchi".to_string(),
    };
    let surname = "sassi".to_string();
    let a = replace_surname(person1, surname);
    println!("{}", a);
}

struct NameSurname{
    name: String,
    surname: String
}
fn replace_surname(mut nm: NameSurname, str: String) -> String {
    let rit = String::from(nm.surname);
    nm.surname = str.to_string();
    rit
}

struct X{
    s: Option<String>,
    i: i32
}

impl X {
    fn new(s: &str, i: i32) -> Self{
        Self{s: Some(String::from(s)), i}
    }

    fn take_str(&mut self) -> Option<String> {
        let ret = self.s.clone();
        self.s = None;
        ret
    }
}

fn prev_char(c: char) -> char {
    (c as u8 - 1) as char
}

fn prev_str(str: &str) -> String {
    let mut string = String::new();
    for c in str.chars() {
        if c.is_alphabetic() && c != 'a' && c != 'A' {
            string.push(prev_char(c))
        } else {
            string.push(c)
        }
    }
    string
}
*/*/