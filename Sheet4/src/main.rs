use std::collections::LinkedList;
use std::f32::consts::PI;
use std::fmt::{write, Debug, Display, Formatter};
use std::ops::{Add, Sub};
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
    let s = "MarcoPolo".to_string();
    println!("Stringa divisa {:?}", s.split());
    let mut lista: LinkedList<f64> = LinkedList::new();
    lista.push_back(1.0);
    lista.push_back(2.0);
    lista.push_back(3.0);
    lista.push_back(4.0);
    lista.push_back(5.0);
    println!("Lista divisa {:?}", lista.split());
}

//ES6
struct Point {
    x: i32,
    y: i32,
}

struct Circle {
    radius: f32,
    center: Point,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Default for Point {
    fn default() -> Self {
        Point { x: 0, y: 0 }
    }
}

impl Default for Circle {
    fn default() -> Self {
        Circle { radius: 1.0, center: Point::default() }
    }
}

impl Default for Rectangle {
    fn default() -> Self {
        Rectangle { top_left: Point { x: -1, y: 1 }, bottom_right: Point { x: 1, y: -1 } }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

struct Area {
    area: f32,
}

impl Default for Area {
    fn default() -> Self {
        Area { area: 0.0 }
    }
}

impl Display for Area {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Area is {} cm²", self.area)
    }
}

trait GetArea {
    fn get_area(&self) -> Area;
}

impl GetArea for Point {
    fn get_area(&self) -> Area {
        Area::default()
    }
}

impl GetArea for Circle {
    fn get_area(&self) -> Area {
        Area { area: PI * self.radius * self.radius }
    }
}


impl GetArea for Rectangle {
    fn get_area(&self) -> Area {
        let base = &self.top_left.x + self.bottom_right.x;
        let altezza = &self.top_left.y + self.bottom_right.y;
        Area { area: (base * altezza) as f32 }
    }
}

impl Add for Area {
    type Output = Area;

    fn add(self, rhs: Self) -> Self::Output {
        Area { area: self.area + rhs.area }
    }
}

impl<'a> Add for &dyn GetArea {
    type Output = &'a dyn GetArea;

    fn add(self, rhs: Area) -> Self::Output {
        self.get_area() + rhs
    }
}

fn sum_area(input: &[&dyn GetArea]) -> Area {
    let mut area: Area = Area::default();
    for v in input.iter() {
        area = area + v.get_area();
    }
    area
}

//ES5
trait Split<'a> {
    type ReturnType;
    fn split(&'a self) -> (Self::ReturnType, Self::ReturnType);
}

impl<'a> Split<'a> for String {
    type ReturnType = &'a str;
    fn split(&'a self) -> (Self::ReturnType, Self::ReturnType) {
        (&self[0..self.len() / 2], &self[self.len() / 2..self.len()])
    }
}

impl<'a> Split<'a> for &[i32] {
    type ReturnType = &'a [i32];

    fn split(&'a self) -> (Self::ReturnType, Self::ReturnType) {
        self.split_at(self.len() / 2)
    }
}

impl<'a> Split<'a> for LinkedList<f64> {
    type ReturnType = LinkedList<f64>;

    fn split(&'a self) -> (Self::ReturnType, Self::ReturnType) {
        let mut left = self.clone();
        let right = left.split_off(left.len() / 2);
        (left, right)
    }
}

//ES4
struct DoubleRef<'r, 's: 'r, T> {
    r: &'r T,
    s: &'s T,
}


//ES2
#[derive(Clone)]
struct Person<'a> {
    name: String,
    father: Option<&'a Person<'a>>,
    mother: Option<&'a Person<'a>>,
}

impl Debug for Person<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl<'a> Person<'a> {
    fn new(name: String, father: Option<&'a Person<'a>>, mother: Option<&'a Person<'a>>) -> Person<'a> {
        Person { name, father, mother }
    }
    fn private_find_relatives<'b>(&'b self, generations: u32, relatives: &mut Vec<&'b Self>) {
        relatives.push(self);
        if generations > 0 {
            if let Some(mother) = self.mother {
                mother.private_find_relatives(generations - 1, relatives);
            }
            if let Some(father) = self.father {
                father.private_find_relatives(generations - 1, relatives);
            }
        }
    }
    pub fn find_relatives(&self, generations: u32) -> Vec<&Self> {
        let mut relatives = Vec::new();
        self.private_find_relatives(generations, &mut relatives);
        relatives
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
