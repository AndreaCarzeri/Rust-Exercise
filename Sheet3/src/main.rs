mod bup;

use std::collections::HashMap;
use std::fmt;
use std::fmt::{write, Formatter};
use crate::Item::{Chips, Coffee, Coke};

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum Item {
    Coke,
    Coffee,
    Chips,
}

enum Coin {
    TenCents,
    TwentyCents,
    FiftyCents,
    Euro1,
    Euro2,
}

impl Coin {
    fn to_cents(&self) -> u32 {
        match self {
            Coin::TenCents => 10,
            Coin::TwentyCents => 20,
            Coin::FiftyCents => 50,
            Coin::Euro1 => 100,
            Coin::Euro2 => 200,
        }
    }
}

struct VendingMachine {
    coins: u32, //soldi all'interno
    items: HashMap<Item, usize>, //item e quantita disponibili
}

impl VendingMachine {
    fn new(mut items: HashMap<Item, usize>) -> Self {
        Self { coins: 0, items }
    }

    fn add_item(&mut self, item: Item, quantity: usize) {
        self.items.insert(item, quantity);
    }

    fn insert_coin(&mut self, coin: Coin) -> Result<&str, &str> {
        let result = match coin {
            Coin::TenCents => Ok("10 Cent inserted"),
            Coin::TwentyCents => Ok("20 Cent inserted"),
            Coin::FiftyCents => Ok("50 Cent inserted"),
            Coin::Euro1 => Ok("1 Euro inserted"),
            Coin::Euro2 => Ok("2 Euro inserted"),
            _ => Err("Invalid coin inserted"),
        };
        if result.is_ok() {
            self.coins += coin.to_cents();
        }
        result
    }

    fn get_item_price(item: &Item) -> u32 {
        match item {
            Item::Coke => 350,
            Chips => 200,
            Item::Coffee => 150
        }
    }

    fn buy(&mut self, item: Item) -> Result<u32, &str> {
        let item_price = VendingMachine::get_item_price(&item);
        let quantity_item = self.items.get_mut(&item).unwrap();
        if self.coins >= item_price && *quantity_item as u32 > 0 {
            let resto = self.coins - item_price;
            self.coins -= item_price;
            *quantity_item -= 1;
            Ok(resto)
        } else {
            Err("Credito non sufficente o credito non sufficente")
        }
    }
}
#[derive(Debug)]
struct Date(u8, u8, u16);
#[derive(Debug)]
struct Hour(u8, u8);
#[derive(Debug)]
struct BoxShipping {
    name: String,
    barecode: String,
    shipment_date: Date,
    shipment_hour: Hour,
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}/{:02}/{:04}", self.0, self.1, self.2)
    }
}

impl fmt::Display for Hour {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.0, self.1)
    }
}

impl fmt::Display for BoxShipping {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"name: {}", self.name)
    }
}

fn main() {
    //ES1
    println!("is it luhn? {}", is_it_luhn(&mut String::from("4539 3195 0343 6467")));

    //ES2
    /*
    Esempio 1: Enum
    Esempio 2: Enum
    Esempio 3: Struct
     */

    //ES3 non come lo vuole lui
    let mut hm: HashMap<&str, &str> = HashMap::new();
    hm.insert("123DJ123", "Andrea");
    hm.insert("HDGKLK23", "Luca");
    hm.insert("LOOOL123", "Marco");
    println!("Owner of HDGKLK23: {}", recognise_owner(&hm, "HDGKLK23").unwrap());
    println!("Owner of AAAAAA: {}", recognise_owner(&hm, "AAAAAAA").unwrap_or(String::from("Non trovato")));

    //ES4
    let mut hash_map: HashMap<Item, usize> = HashMap::new();
    hash_map.insert(Chips, 40);
    hash_map.insert(Coffee, 0);
    hash_map.insert(Coke, 2);
    let mut vending_machine = VendingMachine::new(hash_map);
    vending_machine.add_item(Chips, 20);
    println!("{}", vending_machine.insert_coin(Coin::Euro1).unwrap());
    println!("{}", vending_machine.insert_coin(Coin::Euro2).unwrap());
    //println!("{}",vending_machine.buy(Coke).unwrap());
    //println!("{}",vending_machine.buy(Coffee).unwrap());
    println!("{}", vending_machine.buy(Chips).unwrap());

    //ES5

    //ES6
    let mut bup = bup::Bup::new();
    bup.add_book("Il bologna", "123123123", 2024, "Gianni Morandi", "Bologna Calcio");
    bup.add_article("Zirkzee", "333222212", 2023, "39231aaa");
    bup.add_magazine("Piccolo Bologna", "2178yuidsj7", 2023, 4, 6);
    println!("Bup:\n{}",bup);
}

fn recognise_owner(hm: &HashMap<&str, &str>, plate: &str) -> Option<String> {
    match hm.get(plate) {
        Some(&v) => Some(String::from(v)),
        _ => None
    }
}

fn is_it_luhn(string: &mut String) -> bool {
    //controlli iniziali
    if string.len() <= 1 {
        return false;
    }
    for c in string.chars() {
        if !c.is_ascii_digit() && !c.is_ascii_whitespace() {
            return false;
        }
    }
    let t = string.trim().to_string();
    string.clear();
    string.push_str(t.as_str());

    let mut vec: Vec<String> = Vec::new();
    let mut n: String = String::new();
    for c in string.chars() {
        if c.is_ascii_whitespace() {
            vec.push(n.clone()); //funziona sempre
            n.clear();
        } else {
            n.push(c);
        }
    }
    if n.len() != 0 {
        vec.push(n.parse().expect("errore")); //funziona sempre
    }
    //println!("lista: {:?}", vec);
    let mut somma = 0;
    for numero in vec {
        let mut pari = true;
        if numero.len() % 2 != 0 { //se è pari allora prendo cifre: 0, 2, 4 altrimenti 1, 3, 5
            pari = false;
        }
        for i in 0..numero.len() {
            let digit = numero.chars().nth(i).unwrap().to_digit(10).unwrap();
            if i % 2 == 0 && pari || i % 2 != 0 && !pari {
                let mut t = digit * 2;
                if t > 9 {
                    t -= 9;
                }
                somma += t;
            } else {
                somma += digit;
            }
        }
    }
    //println!("Somma: {}", somma);
    somma % 10 == 0
}
