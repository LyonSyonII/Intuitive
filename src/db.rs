use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use itertools::chain;
use phf::{phf_set};

pub struct DB {
    pub ignore: HashSet<&'static str>,
    pub print: HashSet<&'static str>,
    pub r#true: HashSet<&'static str>,
    pub r#false: HashSet<&'static str>,
    pub keyword: HashSet<&'static str>,
}

impl DB {
    pub fn new(&self) -> Self {
        DB {
             ignore: HashSet::from(["a"]),
              print: HashSet::from(["print", "imprimir", "imprimeix"]),
               r#true: HashSet::from(["true", "cert", "veritat"]),
              r#false: HashSet::from(["false", "fals", "mentida"]),
            keyword: HashSet::from_iter(chain!(self.print, self.r#true, self.r#false)),
        }
    }

    pub fn is_literal(&self, string: &str) -> bool {
        string.starts_with('"')
            || string.starts_with('\'')
            || string.parse::<i64>().is_ok()
            || self.r#true.contains(string)
            || self.r#false.contains(string)
    }
}



/*
const fn clone_sets<'a>(sets: &[HashSet<&'a str>]) -> HashSet<&'a str> {
    let mut res : HashSet<&str> = HashSet::new();
    
    for set in sets {
        res.extend(set);
    }
    res
} */