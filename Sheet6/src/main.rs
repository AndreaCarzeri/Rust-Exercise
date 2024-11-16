use std::cell::RefCell;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;

fn main() {
    let mut v = Vec::new();
    v.push(3);
    v.push(1);
    v.push(6);
    v.push(4);
    v.push(7);
    v.push(0);
    let mut tree = TreeNode::from_vec(v);
    tree.pre_order_visit();
    //ES2

    let v: Vec<CarRef> = Vec::new();
    let mut car_dealer = CarDealer::new(v);
    car_dealer.add_car(Car { price: 10000, rent: false, year: 2010, model: "Toyota".to_string() });
    car_dealer.add_car(Car { price: 15000, rent: false, year: 2015, model: "Volvo".to_string() });
    car_dealer.add_car(Car { price: 30000, rent: false, year: 2020, model: "Tesla".to_string() });
    let mut user: User = User { car: None };
    car_dealer.rent_user(&mut user, "Toyota");
    user.print_car();
    CarDealer::end_rental(&mut user);
    user.print_car();
}

//ES3
trait Sound {
    fn make_sound(&self) -> String;
}

struct Dog;
struct Cat;
struct Pig;

impl Sound for Dog {
    fn make_sound(&self) -> String {
        "Woof".to_string()
    }
}
impl Sound for Pig {
    fn make_sound(&self) -> String {
        "Oink".to_string()
    }
}
impl Sound for Cat {
    fn make_sound(&self) -> String {
        "Miao".to_string()
    }
}
struct FarmCell {
    animal: Box<dyn Sound>,
    next: Option<Box<FarmCell>>,
}

impl FarmCell {
    fn new(v: Box<dyn Sound>) -> Self {
        FarmCell { animal: v, next: None }
    }
    fn insert(&mut self, v: Box<dyn Sound>) {
        match self.next {
            None => { self.next = Some(Box::new(FarmCell::new(v))) }
            Some(ref mut next) => { next.insert(v) }
        }
    }
}

impl Sound for FarmCell {
    fn make_sound(&self) -> String {
        let mut s = String::new();
        s.push_str(self.animal.make_sound().as_str());
        s.push_str("\n");

        if let Some(ref next) = self.next {
            s.push_str(Self::make_sound(&*next).as_str())
        }
        s
    }
}


//ES2

type CarRef = Rc<RefCell<Car>>;
#[derive(Debug)]
struct Car {
    model: String,
    year: u32,
    price: u32,
    rent: bool,
}

impl Car {
    fn new(price: u32, year: u32, rent: bool, model: String) -> Self {
        Self { price, year, rent, model }
    }

    fn default() -> Self {
        Car { price: 10000, rent: false, year: 2010, model: "Yaris".to_string() }
    }
}

struct CarDealer {
    cars: Vec<CarRef>,
}

#[derive(Debug)]
struct User {
    car: Option<CarRef>,
}

impl User {
    fn print_car(&self) {
        if let Some(ref v) = self.car {
            println!("{:?}", v);
        } else {
            println!("User has no car");
        }
    }
}

impl CarDealer {
    fn new(cars: Vec<CarRef>) -> Self {
        Self { cars }
    }

    fn add_car(&mut self, car: Car) {
        self.cars.push(Rc::new(RefCell::new(car)))
    }

    fn print_cars(&self) {
        for c in &self.cars {
            println!("{:?}", c);
        }
    }

    fn rent_user(&mut self, user: &mut User, model: &str) {
        for car_rc in &self.cars {
            let mut car = car_rc.borrow_mut();
            if car.model == model && !car.rent {
                car.rent = true;
                user.car = Some(Rc::clone(car_rc)); // Ownership condivisa con l'utente
                return; // Esci immediatamente dopo aver trovato e noleggiato la macchina
            }
        }

        println!("Car not found");
    }

    fn end_rental(user: &mut User) {
        match user.car.clone() {
            None => { println!("User has no car") }
            Some(car) => {
                car.borrow_mut().rent = false;
                user.car = None;
            }
        }
    }
}


//ES1
struct TreeNode<T: PartialOrd + Clone + Debug> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: PartialOrd + Clone + Debug,
{
    pub fn new(value: T) -> Self {
        Self { value, left: None, right: None }
    }

    pub fn from_vec(vec: Vec<T>) -> Self {
        let mut root = TreeNode::new(vec[0].clone());
        for i in 1..vec.len() {
            Self::insert(&mut root, vec[i].clone())
        }
        root
    }

    pub fn insert(&mut self, value: T) {
        if value > self.value {
            match self.right {
                None => self.right = Some(Box::new(TreeNode::new(value))),
                Some(ref mut right) => right.insert(value)
            }
        } else {
            match self.left {
                None => self.left = Some(Box::new(TreeNode::new(value))),
                Some(ref mut left) => left.insert(value)
            }
        }
    }

    pub fn pre_order_visit(&self) {
        println!("{:?}", self.value);
        if let Some(ref left) = self.left {
            left.pre_order_visit()
        }
        if let Some(ref right) = self.right {
            right.pre_order_visit()
        }
    }
}
