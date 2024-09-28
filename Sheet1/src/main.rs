use std::collections::HashMap;
use std::ops::{Add, Index};
use std::io;
use std::ptr::hash;

const VELOCITÀ_LUCE: f32 = 299_792_458.0; // in m/s

fn main() {
    let stringa1 = "Hello, world!";
    println!("Stringa invertita: {}", string_reverse(stringa1));

    //es 2
    let v1: i32 = 10;
    let v2: i32 = 5;
    println!("Il maggiore tra {} e {} è: {}", v1, v2, bigger(v1, v2));

    //es3
    let v1: i32 = 10;
    let v2: f32 = 3.5;
    let v3: f64 = 9.35;
    println!("La moltiplicazione tra {}, {}, {} è: {}", v1, v2, v3, multiply(v1, v2, v3));

    //es4
    let v1: f32 = 80.0;
    println!("e = mc^2 {}", e_equals_mc_squared(v1));

    //es5
    let mut vettore: Vec<i32> = Vec::new();
    vettore.push(5);
    vettore.push(1);
    vettore.push(3);
    vettore.push(9);
    vettore.push(2);
    let valori = max_min(&vettore);
    println!("Vettore iniziale: {:?}", vettore);
    match valori {
        Some((max, min)) => println!("Max: {}, Min: {}", max, min),
        None => println!("Il vettore è vuoto."),
    }

    //es6
    let stringa = String::from("ciao sono delle lettere vediamo cosa succede");
    println!("Stringa convertita: {}", lord_farquaad(&stringa));

    //es7
    let mut forniture: HashMap<String, f32> = HashMap::new();
    forniture.insert(String::from("Lavapiatti"), 80.0);
    forniture.insert(String::from("Lavastoviglie"), 100.0);
    forniture.insert(String::from("Microonde"), 35.0);
    let mut input = String::new();
    // Mostra un messaggio all'utente
    println!("Inserisci il nome della forniture:");
    // Leggi l'input da tastiera e salva nella variabile `input`
    io::stdin()
        .read_line(&mut input) // read_line salva l'input nella variabile
        .expect("Errore nella lettura dell'input");
    //println!("il prezzo della forniture inserita e': {}", get_forniture_price(&forniture, &input));

    //ES8
    let stringa = String::from("Stringa di Base");
    let new_string = append(stringa.clone());
    println!("Stringa iniziale: {} e Stringa finale: {}", stringa, new_string);
}

fn append(stringa: String) -> String {
    let new_string = stringa.clone();
    new_string.add("foobar") //non concatena ma fa una "somma" e quindi ritorna una stringa
}

fn get_forniture_price(hash_map: &HashMap<String, f32>, input: &String) -> f32 {

    // Rimuovi spazi bianchi e newline dall'input
    let input = input.trim();

    let risultato = hash_map.get(input);

    match risultato {
        Some(&prezzo) => prezzo,
        None => -1.0
    }
}

fn lord_farquaad(s: &String) -> String {
    let mut risultato = String::new(); // Nuova stringa in cui costruire il risultato

    for lettera in s.chars() {
        if lettera == 'e' {
            risultato.push_str("💥"); // Sostituisci 'e' con l'emoji
        } else {
            risultato.push(lettera); // Mantieni il resto dei caratteri
        }
    }

    risultato
}


fn max_min(v: &Vec<i32>) -> Option<(i32, i32)> {
    // Gestire il caso di un vettore vuoto
    if v.is_empty() {
        return None;
    }

    let mut min = v[0]; // Inizializza min con il primo elemento
    let mut max = v[0]; // Inizializza max con il primo elemento

    for &numero in v { // Usando &numero per dereferenziare direttamente
        if numero < min {
            min = numero;
        }
        if numero > max {
            max = numero;
        }
    }

    Some((max, min)) // Restituisce i risultati come un'Option
}


fn e_equals_mc_squared(mass: f32) -> f32 {
    VELOCITÀ_LUCE.powf(2.0) * mass
}

fn string_reverse(s: &str) -> String { //ES 1
    let mut risultato = String::new();  // Creiamo una stringa vuota per contenere il risultato

    // Iteriamo sui caratteri della stringa, partendo dall'ultimo
    for c in s.chars().rev() {
        risultato.push(c);  // Aggiungiamo ciascun carattere invertito alla nuova stringa
    }

    risultato  // Restituiamo la stringa invertita
}

fn bigger(v1: i32, v2: i32) -> i32 {
    if v1 > v2 { v1 } else { v2 }
}

fn multiply(v1: i32, v2: f32, v3: f64) -> f64 {
    (v1 as f64) * (v2 as f64) * v3
}




