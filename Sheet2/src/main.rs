use std::collections::HashMap;
use std::ops::Index;

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
    let vec1 = vec![1,2,3,4,5,6];
    let vec2 = vec![2,7];
    sub_slice(&vec1,&vec2);

    //ES5

}

fn sub_slice(vec1: &Vec<i32>, vec2: &Vec<i32>) {
    let mut index2 = 0;
    let mut start_print =0;
    let mut contatore =0;
    let mut end_print=0;
    for element in vec1.iter() {
        contatore+=1;
        if index2 < vec2.len() && *element == vec2[index2] {
            index2+=1;
            if start_print == 0 {
                start_print=index2;
            }
            end_print=contatore;
        }
    }
    if index2 == vec2.len() {
        println!("{:?}", &vec1[start_print..end_print]);
    }else {
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
