use std::collections::HashMap;
use std::ops::Index;
use crate::Expression::Number;

enum I32String {
    Int(i32),
    Text(String),
}

enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

enum Expression {
    Number(i32),
    Operation {
        left: Box<Expression>,
        operation: Operation,
        right: Box<Expression>,
    },
}

fn main() {
    //ES1
    let mut array = [1, 2, 3, 4, 5];
    println!("Array prima: {:?}", array);
    modify_odd(&mut array[1..4]);
    println!("Array dopo (slice da 1 a 3 compreso: {:?}", array);
    modify_odd_2();

    //ES2
    let hm = count_character(String::from("questo è un testo di prova per vedere se funziona"));
    println!("HashMap {:?}", hm);

    //ES3
    let array = [1, 2, 3, 4, 5, 6];
    println!("Valore restituito: {:?}", split_at_value(&array, 9));

    //ES4
    let mut vec1 = vec![1, 2, 3, 4, 5, 6];
    let vec2 = vec![2, 7];
    sub_slice(&vec1, &vec2);

    //ES5
    println!("Max del vett: {:?} è: {}", vec1, max(&vec1));
    println!("Il vettore è ordinato in modo crescente? {}", is_sorted(&vec1));
    swap(&mut vec1);
    println!("Swappo primo ed ultimo {:?}", vec1);
    let mut vec_str = vec![String::from("Ciao"), String::from("sono"), String::from("luca")];
    insert_if_longer(&mut vec_str, String::from("giuratoooooo"));
    println!("Nuovo vec:  {:?}", vec_str);

    //ES6
    println!("Build_Vector: {:?}", build_vector(vec1.iter()));

    //ES7
    let mut vec_pancake = vec![3, 3, 3, 2, 2, 1, 2, 2, 3, 5, 7, 8, ];
    println!("Prima pancake {:?}", vec_pancake);
    pancake_sort(&mut vec_pancake);
    println!("Dopo pancake {:?}", vec_pancake);

    //ES8
    let v1 = [1, 3, 5, 6, 9];
    let v2 = [2, 3, 4, 7, 8, 15, 34];
    println!("Merged: {:?}", merge(&v1, &v2));

    //ES9
    let mut vector_mixed: Vec<I32String> = Vec::new();
    vector_mixed.push(I32String::Int(50));
    vector_mixed.push(I32String::Text(String::from("ciao")));

    //ES10
    let expression1: Expression = Expression::Operation { left: Box::new(Number(10)), operation: Operation::Add, right: Box::new(Number(10)) };
    let expression2: Expression = Expression::Operation { left: Box::new(Number(3)), operation: Operation::Mul, right: Box::new(Number(10)) };
    let expression3: Expression = Expression::Operation { left: Box::new(expression2), operation: Operation::Sub, right: Box::new(expression1) }; //30 - 20
    println!("Il risultato dell'operazione e': {:?}", evaluete_expression(&expression3));
}

fn evaluete_expression(expression: &Expression) -> Result<i32, &str> {
    let mut result: Result<i32, &str> = Ok(0);
    match expression {
        Number(i) => result = Ok(*i),
        Expression::Operation { left, operation, right } => {
            let value_left = evaluete_expression(left)?;
            let value_right = evaluete_expression(right)?;
            match operation {
                Operation::Add => {
                    let ris_checked = value_left.checked_add(value_right); //ritorna option da controllare se è andato tutto a buon fine
                    match ris_checked {
                        None => result = Err("overflow"),
                        Some(v) => result = Ok(v)
                    }
                }
                Operation::Sub => {
                    let ris_checked = value_left.checked_sub(value_right); //ritorna option da controllare se è andato tutto a buon fine
                    match ris_checked {
                        None => result = Err("overflow"),
                        Some(v) => result = Ok(v)
                    }
                }
                Operation::Mul => {
                    let ris_checked = value_left.checked_mul(value_right); //ritorna option da controllare se è andato tutto a buon fine
                    match ris_checked {
                        None => result = Err("overflow"),
                        Some(v) => result = Ok(v)
                    }
                }
                Operation::Div => {
                    let ris_checked = value_left.checked_div(value_right); //ritorna option da controllare se è andato tutto a buon fine
                    match ris_checked {
                        None => result = Err("division by zero"),
                        Some(v) => result = Ok(v)
                    }
                }
            }
        }
    }
    result
}

fn merge(v1: &[i32], v2: &[i32]) -> Vec<i32> {
    let mut vec = Vec::from(v1);
    let mut index2 = 0;
    let mut max_index = vec.len();
    let mut i = 0;

    while i < max_index {
        println!("Confronto {} con {}", vec[i], v2[index2]);
        if vec[i] >= v2[index2] {
            vec.insert(i, v2[index2]);
            index2 += 1;
            max_index += 1;
        }
        if index2 == v2.len() {
            break;
        }
        i += 1;
    }
    //aggiungere i rimanenti
    for i in index2..v2.len() {
        vec.push(v2[i]);
    }
    vec
}

fn flip_pancake(vec: &mut Vec<i32>, mut k: usize) {
    let mut left: usize = 0;
    while left < k {
        let t = vec[left];
        vec[left] = vec[k];
        vec[k] = t;
        k -= 1;
        left += 1;
    }
}

fn max_index_pancake(vec: &mut Vec<i32>, mut k: usize) -> usize {
    let mut index: usize = 0;
    for i in 0..k {
        if vec[i] > vec[index] {
            index = i;
        }
    }
    index
}
fn pancake_sort(vec: &mut Vec<i32>) {
    let mut n = vec.len();
    while n > 1 {
        let maxidx = max_index_pancake(vec, n);
        if maxidx != n - 1 {
            if maxidx != 0 {
                flip_pancake(vec, maxidx);
            }
            flip_pancake(vec, n - 1);
        }
        n -= 1;
    }
}

fn build_vector(iter: std::slice::Iter<i32>) -> Vec<&i32> {
    let mut vec = Vec::new();
    for num in iter {
        vec.push(num)
    }
    vec
}

fn insert_if_longer(vec: &mut Vec<String>, str: String) {
    if str.len() > 10 {
        vec.push(str)
    }
}

fn is_sorted(vec: &Vec<i32>) -> bool {
    let mut sorted = true;
    let mut prec = vec[0];
    for i in vec { //salto il primo
        if *i < prec {
            return false;
        } else {
            prec = *i;
        }
    }
    sorted
}

fn max(vec: &[i32]) -> i32 {
    let mut max = vec[0];
    for n in vec {
        if *n > max {
            max = *n;
        }
    }
    max
}

fn swap(vec: &mut Vec<i32>) {
    let t = vec[0];
    let last_index = vec.len() - 1;
    vec[0] = vec[last_index];
    vec[last_index] = t;
}

fn sub_slice(vec1: &Vec<i32>, vec2: &Vec<i32>) {
    let mut index2 = 0;
    let mut start_print = 0;
    let mut contatore = 0;
    let mut end_print = 0;
    for element in vec1.iter() {
        contatore += 1;
        if index2 < vec2.len() && *element == vec2[index2] {
            index2 += 1;
            if start_print == 0 {
                start_print = index2;
            }
            end_print = contatore;
        }
    }
    if index2 == vec2.len() {
        println!("{:?}", &vec1[start_print..end_print]);
    } else {
        println!("Not found");
    }
}

fn split_at_value(slice: &[i32], value: i32) -> Option<(&[i32], &[i32])> {
    if !slice.contains(&value) {
        None
    } else {
        let mut index = Option::<usize>::None;
        for (i, element) in slice.iter().enumerate() {
            if *element == value {
                index = Option::Some(i);
                break;
            }
        }
        let index = index?;
        Some((&slice[..index], &slice[index..]))
    }
}

fn count_character(str: String) -> HashMap<char, u32> {
    let mut hm: HashMap<char, u32> = HashMap::new();
    for char in str.chars() {
        if let Some(val) = hm.get_mut(&char) { //se esiste un valore lo aumenta altimenti lo crea
            *val += 1;
        } else {
            hm.insert(char, 1);
        }
    }
    hm
}

fn modify_odd_2() {
    let mut vec: Vec<i32> = Vec::new();
    let mut cont = 0;
    while cont < 100 {
        vec.push(cont);
        cont += 1;
    }
    println!("Vector: {:?}", vec);
    modify_odd(&mut vec);
    println!("Vector: {:?}", vec);
}

fn modify_odd(slice: &mut [i32]) {
    for n in slice {
        if *n % 2 != 0 {
            *n = 0;
        }
    }
}
