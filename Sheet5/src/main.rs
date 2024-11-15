use std::cmp::min;
use std::fmt::{Debug, Display};
use std::path::Iter;
use rand::{Rng, RngCore};
use std::ops::{Add, Sub, Mul};
use crate::CarrotState::{Burnt, Cooked, Fried, Raw};

fn main() {
    let mut library = Library::default();
    library.populate();
    println!("{:?}", library.bookcases);
}

//ES7
trait Heatable {
    fn cook(&mut self);
}

trait Friable {
    fn cook(&mut self);
}

trait Heater {
    fn heat(&self, heatable: &mut dyn Heatable);
}

trait Frier {
    fn fry(&self, friable: &mut dyn Friable);
}

struct Oven {}
struct Pan {}

impl Heater for Oven {
    fn heat(&self, heatable: &mut dyn Heatable) {
        heatable.cook()
    }
}

impl Heater for Pan {
    fn heat(&self, heatable: &mut dyn Heatable) {
        heatable.cook()
    }
}

impl Frier for Pan {
    fn fry(&self, friable: &mut dyn Friable) {
        friable.cook()
    }
}

struct Pie {
    ready: bool,
}

struct Carrot {
    carrot_state: CarrotState,
}

#[derive(Eq, PartialEq)]
enum CarrotState {
    Raw,
    Cooked,
    Fried,
    Burnt,
}

trait Edible {
    fn eat(&self);
}

impl Heatable for Pie {
    fn cook(&mut self) {
        if self.ready {
            println!("you burned the pie");
        } else {
            self.ready = !self.ready
        }
    }
}

impl Heatable for Carrot {
    fn cook(&mut self) {
        if self.carrot_state != Raw {
            self.carrot_state = Burnt
        } else {
            self.carrot_state = Cooked
        }
    }
}

impl Friable for Carrot {
    fn cook(&mut self) {
        if self.carrot_state == Fried {
            self.carrot_state = Burnt
        } else {
            self.carrot_state = Fried
        }
    }
}

impl Edible for Pie{
    fn eat(&self) {
        if !self.ready {
            println!("you got stomach ache")
        }else{
            println!("yummy")
        }
    }
}

impl Edible for Carrot {
    fn eat(&self) {
        match self.carrot_state {
            Raw => {println!("mmh, crunchy")}
            Cooked => {println!("mmh, yummy")}
            Fried => {println!("mmh, crispy")}
            Burnt => {println!("mmh, burnt")}
        }
    }
}


//ES6
#[derive(Debug)]
struct Open;
#[derive(Debug)]
struct Closed;
#[derive(Debug)]
struct Stopped {
    _reason: String,
}
#[derive(Debug)]
struct Gate<S> {
    _state: S,
}

impl Gate<Open> {
    pub fn new() -> Gate<Open> {
        Gate { _state: Open }
    }
    fn close(self) -> Result<Gate<Closed>, Gate<Stopped>> {
        let r = rand::thread_rng().gen_range(0..20);
        match r {
            0..=12 => Ok(Gate { _state: Closed }),
            13..=15 => Err(Gate { _state: Stopped { _reason: "Motor error".to_string() } }),
            _ => Err(Gate { _state: Stopped { _reason: "External error".to_string() } })
        }
    }
}

impl Gate<Stopped> {
    pub fn new(reason: &str) -> Gate<Stopped> {
        Gate { _state: Stopped { _reason: reason.to_string() } }
    }
    fn open(self) -> Gate<Open> {
        Gate { _state: Open }
    }
    fn close(self) -> Gate<Closed> { Gate { _state: Closed } }
}

impl Gate<Closed> {
    fn new() -> Gate<Closed> {
        Gate { _state: Closed }
    }
    fn open(self) -> Result<Gate<Open>, Gate<Stopped>> {
        let r = rand::thread_rng().gen_range(0..20);
        match r {
            0..=12 => Ok(Gate { _state: Open }),
            13..=15 => Err(Gate { _state: Stopped { _reason: "Motor error".to_string() } }),
            _ => Err(Gate { _state: Stopped { _reason: "External error".to_string() } })
        }
    }
}

//ES5
struct Pair(i32, String);

impl Add<i32> for Pair {
    type Output = Self;

    fn add(self, rhs: i32) -> Self::Output {
        Self { 0: self.0 + rhs, 1: self.1 }
    }
}

impl Sub<i32> for Pair {
    type Output = Self;

    fn sub(self, rhs: i32) -> Self::Output {
        Self { 0: self.0 - rhs, 1: self.1 }
    }
}

impl Add<&str> for Pair {
    type Output = Self;

    fn add(self, rhs: &str) -> Self::Output {
        Self { 0: self.0, 1: self.1.add(rhs) }
    }
}

impl Sub<&str> for Pair {
    type Output = Self;

    fn sub(self, rhs: &str) -> Self::Output {
        Self { 0: self.0, 1: self.1.replace(rhs, "") }
    }
}

impl Add<Pair> for Pair {
    type Output = Self;

    fn add(self, rhs: Pair) -> Self::Output {
        self + rhs.0 + rhs.1.as_str()
    }
}

impl Mul<i32> for Pair {
    type Output = Self;

    fn mul(self, n: i32) -> Self::Output {
        Self { 0: self.0.pow(n as u32), 1: self.1.repeat(n as usize) }
    }
}

//ES4
struct Tasks {
    tasks: Vec<Task>,
}

struct Task {
    name: String,
    priority: i32,
    done: bool,
}

impl Task {
    fn new() -> Self {
        Self { name: "".to_string(), priority: 0, done: false }
    }
}

impl Tasks {
    fn new() -> Self {
        Self { tasks: Vec::new() }
    }
}

impl Iterator for Tasks {
    type Item = Task;

    fn next(&mut self) -> Option<Self::Item> {
        self.tasks.iter().position(|t| !t.done).map(|i| self.tasks.remove(i))
    }
}

//ES3
fn restricted<T, U>(t1: T, t2: T, u: U) -> impl Debug + PartialEq + Ord
where
    T: Debug + PartialOrd + Ord,
    U: Display,
{
    let minor = if t1 < t2 { t1 } else { t2 };
    println!("minor: {:?}", minor);
    println!("u: {:}", u);
    minor
}

//ES2
#[derive(Debug, Default, PartialEq)]
enum Category {
    CAT1,
    CAT2,
    #[default]
    CAT3,
}

#[derive(Debug)]
struct Book {
    title: String,
    cat: Category,
}

#[derive(Debug, Default)]
struct Library {
    bookcases: [Vec<Book>; 10],
}

impl Default for Book {
    fn default() -> Self {
        let mut title = String::new();
        for _ in 0..8 {
            title.push((rand::thread_rng().next_u32() % 26 + ('a' as u32)) as u8 as char)
        }
        Self { title, cat: Category::default() }
    }
}

impl Book {
    fn default_with_cat(cat: Category) -> Self {
        Book {
            cat,
            ..Self::default()
        }
    }
}

trait Populatable {
    fn populate(&mut self);
}

impl Populatable for Library {
    fn populate(&mut self) {
        for floor in &mut self.bookcases {
            floor.push(Book::default());
            floor.push(Book::default());
            floor.push(Book::default());
        }
    }
}

//ES1
trait Printable {
    fn print(&self);
}

impl Printable for i32 {
    fn print(&self) {
        println!("{}", self)
    }
}
impl Printable for String {
    fn print(&self) {
        println!("{}", self)
    }
}
impl<T> Printable for Vec<T>
where
    T: Printable,
{
    fn print(&self) {
        for el in self { el.print() }
    }
}

fn print<T>(t: impl Printable) {
    t.print()
}
