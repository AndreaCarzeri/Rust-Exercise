use std::collections::HashMap;

fn main() {
    //ES1
    println!("is it luhn? {}", is_it_luhn(&mut String::from("4539 3195 0343 6467")));

    //ES2
    /*
    Esempio 1: Enum
    Esempio 2: Enum
    Esempio 3: Struct
     */

    //ES3
    let mut hm: HashMap<&str, &str> = HashMap::new();
    hm.insert("123DJ123", "Andrea");
    hm.insert("HDGKLK23", "Luca");
    hm.insert("LOOOL123", "Marco");
    println!("Owner of HDGKLK23: {}", recognise_owner(&hm, "HDGKLK23").unwrap());
    println!("Owner of AAAAAA: {}", recognise_owner(&hm, "AAAAAAA").unwrap_or(String::from("Non trovato")));
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
