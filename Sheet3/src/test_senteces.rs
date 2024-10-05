use std::cmp::min;
use std::collections::HashMap;
use crate::sentence::Sentence;

pub fn magic_sentence(hm: &HashMap<i32, Sentence>, i: i32, j: i32) -> Result<Sentence, &str> {
    let s1 = hm.get(&i);
    let s2 = hm.get(&j);

    if s1.is_none() {
        return Err("Sentence on i not found");
    }

    if s2.is_none() {
        return Err("Sentence on j not found");
    }

    //scorro le due sentence e vedo le parole uguali
    let mut s_return = Sentence::new_default();
    let mut cont = 0;
    let min_len = min(s1.unwrap().words.len(), s2.unwrap().words.iter().len());
    while cont < min_len {
        if s1.unwrap().words.get(cont) == s2.unwrap().words.get(cont) {
            s_return.words.push(s1.unwrap().words.get(cont).unwrap().clone())
        }
        cont += 1;
    }

    if s_return.words.len() == 0 {
        return Err("No equal words finded");
    }
    Ok(s_return)
}