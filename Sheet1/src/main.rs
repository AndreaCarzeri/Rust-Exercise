
const VELOCITÀ_LUCE: f32 = 299_792_458.0; // in m/s

fn main() {
    let stringa1 = "Hello, world!";
    println!("Stringa invertita: {}",string_reverse(stringa1));

    //es 2
    let v1: i32 = 10;
    let v2: i32 = 5;
    println!("Il maggiore tra {} e {} è: {}",v1,v2,bigger(v1,v2));

    //es3
    let v1: i32 = 10;
    let v2: f32 = 3.5;
    let v3: f64 = 9.35;
    println!("La moltiplicazione tra {}, {}, {} è: {}",v1,v2,v3,multiply(v1,v2,v3));

    //es4
    let v1: f32 = 80.0;
    println!("e = mc^2 {}",e_equals_mc_squared(v1));


}

fn e_equals_mc_squared(mass: f32) -> f32 {
    VELOCITÀ_LUCE.powf(2.0)*mass
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
    if v1 > v2 { v1 }
    else { v2 }
}

fn multiply (v1: i32, v2: f32, v3: f64) -> f64{
    (v1 as f64) * (v2 as f64) * v3
}




