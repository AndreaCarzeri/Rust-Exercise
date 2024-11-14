use std::collections::{HashMap, LinkedList};
use std::f32::consts::PI;
use std::fmt::{write, Debug, Display, Formatter};
use std::ops::{Add, Sub};
use rand::Rng;
use crate::Permission::{EXECUTE, READ, WRITE};
use crate::Role::{ADMIN, GUEST, USER};

fn main() {
    println!("{:?}", find_equal("lollo", "alelo"));
    println!("{:?}", lucky_slice("lollololololandreaalebusola"));

    //ES2
    let p1 = Person::new("Genoveffo".to_string(), None, None);
    let p2 = Person::new("Adelaide".to_string(), None, None);
    let p3 = Person::new("Luigia".to_string(), None, None);
    let p4 = Person::new("Rapuano".to_string(), Some(&p1), Some(&p2));
    let p5 = Person::new("Vincenzo".to_string(), None, Some(&p3));
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
    //ES7
    let telephone_number = "+398721871827";
    let prefix = "+37";
    println!("Risultato telefono: {}", skip_prefix(telephone_number, prefix));
}

//ES9
#[derive(PartialEq, Eq)]
enum Role {
    GUEST,
    USER,
    ADMIN,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Permission {
    READ,
    WRITE,
    EXECUTE,
}

struct Actions {
    action: String,
    permission: HashMap<Permission, bool>,
}

struct User {
    name: String,
    role: Role,
    actions: Vec<Actions>,
}

trait Auth {
    fn check_permission(&self, action: &str, permission_type: &Permission) -> bool;
    fn can_write(&self, string: &str) -> bool;
    fn can_read(&self, string: &str) -> bool;
    fn can_execute(&self, string: &str) -> bool;
}

impl Auth for User {
    fn check_permission(&self, action: &str, permission_type: &Permission) -> bool {
        for a in self.actions.iter() {
            if a.action.eq(action) { //guardo se nella posizione "permission_type" è true o false
                return a.permission[permission_type];
            }
        }
        false
    }

    fn can_write(&self, string: &str) -> bool {
        self.check_permission(string, &WRITE)
    }

    fn can_read(&self, string: &str) -> bool {
        self.check_permission(string, &READ)
    }

    fn can_execute(&self, string: &str) -> bool {
        self.check_permission(string, &EXECUTE)
    }
}

impl Default for Actions {
    fn default() -> Self {
        let mut hm: HashMap<Permission, bool> = HashMap::new();
        hm.insert(WRITE, false);
        hm.insert(READ, false);
        hm.insert(EXECUTE, false);
        Self { action: "".to_string(), permission: hm }
    }
}

impl Actions {
    fn new(action: String, read: bool, write: bool, execute: bool) -> Self {
        let mut hm: HashMap<Permission, bool> = HashMap::new();
        hm.insert(WRITE, write);
        hm.insert(READ, read);
        hm.insert(EXECUTE, execute);
        Self { action, permission: hm }
    }
}

impl Default for User {
    fn default() -> Self {
        Self { name: "Guest".to_string(), role: GUEST, actions: Vec::new() }
    }
}

impl User {
    fn change_role(&mut self, role: Role) -> Result<(), String> {
        if self.role == ADMIN {
            self.role = role;
            Ok(())
        } else if self.role == USER {
            if role == ADMIN {
                Err("You can't be ADMIN".to_string())
            } else {
                self.role=role;
                Ok(())
            }
        } else {
            if role == GUEST {
                self.role=role;
                Ok(())
            } else {
                Err("You can't cange your role".to_string())
            }
        }
    }
}

fn sudo_change_permission(user: &mut User, string: String, permission: Permission, value: bool){
    for mut a in user.actions{
        if a.action==string {
            a.permission.insert(permission, value);
        }
    }
}


//ES8
#[derive(Debug)]
struct Chair<'a> {
    color: &'a str,
    quantity: &'a usize,
}

#[derive(Debug)]
struct Wardrobe<'a> {
    color: &'a str,
    quantity: &'a usize,
}

trait Object {
    fn build(&self) -> &str;
    fn get_quantity(&self) -> String;
}

impl<'a> Object for Chair<'a> {
    fn build(&self) -> &str {
        "Chair has been built"
    }

    fn get_quantity(&self) -> String {
        format!("We have {} chairs", self.quantity)
    }
}

impl<'a> Object for Wardrobe<'a> {
    fn build(&self) -> &str {
        "Wardrobe has been built"
    }

    fn get_quantity(&self) -> String {
        format!("We have {} wardrobe", self.quantity)
    }
}

impl<'a> Display for Chair<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.quantity <= &0 {
            write!(f, "There are no chair")
        } else if self.quantity == &1 {
            write!(f, "There is a {} chair", self.color)
        } else {
            write!(f, "There are {} {} chair", self.quantity, self.color)
        }
    }
}

impl<'a> Display for Wardrobe<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.quantity <= &0 {
            write!(f, "There are no wardrobe")
        } else if self.quantity == &1 {
            write!(f, "There is a {} wardrobe", self.color)
        } else {
            write!(f, "There are {} {} wardrobe", self.quantity, self.color)
        }
    }
}

//ES7
fn skip_prefix<'b, 'a>(telephone_number: &'a str, prefix: &'b str) -> &'a str {
    if !telephone_number.contains(prefix) {
        telephone_number
    } else {
        telephone_number.split_at(prefix.len()).1
    }
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

impl Add for &dyn GetArea {
    type Output = Area;

    fn add(self, rhs: Self) -> Self::Output {
        self.get_area() + rhs.get_area()
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
