use std::collections::{HashMap, HashSet};

// TODO: Just copy strings
pub const IGNORE: HashSet<&str> = HashSet::from([""]);
pub const PRINT: HashSet<&str> = HashSet::from(["print", "imprimir", "imprimeix"]);
pub const TRUE: HashSet<&str> = HashSet::from(["true", "cert", "veritat"]);
pub const FALSE: HashSet<&str> = HashSet::from(["false", "fals", "mentida"]);
pub const KEYWORD: fn() -> HashSet<&&'static str> = || {
    let mut res: HashSet<&&str> = HashSet::new();
    let keywords = vec![&IGNORE, &PRINT, &TRUE, &FALSE];
    
    for set in keywords {
        for keyword in set {
            res.insert(keyword);
        }
    }
    
    println!("{:?}", res);
    res
};

pub fn IS_LITERAL(string: &str) -> bool {
           string.starts_with('"') 
        || string.starts_with('\'') 
        || string.parse::<i64>().is_ok() 
        || TRUE.contains(string) 
        || FALSE.contains(string)
}