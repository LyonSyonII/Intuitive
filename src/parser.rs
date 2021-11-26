// TODO: Turn variables hashset into hashmap with rule and check when operating

use die::Die;
use pest::{iterators::Pair, iterators::Pairs, Parser as P};
use std::fs::File;
use std::io::*;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct Parser;

#[derive(Default)]
struct Global {
    variables: std::collections::HashSet<String>,
}

pub fn parse_file(file: &mut File) -> String {
    // Read file into string
    let mut inp = String::new();
    file.read_to_string(&mut inp).die("Unable to read the file");
    let parse = Parser::parse(Rule::MAIN, &inp).die("Error parsing the file");

    let mut out: String =
        "#[allow(non_snake_case)]\n#[allow(unused_variables)]\n#[allow(unused_mut)]\n#[allow(unused_assignments)]\nfn main() {"
            .into();

    let mut global = Global::default();

    // Read instructions
    for expr in parse {
        out += &parse_expr(expr, &mut global)
    }
    rustfmt_wrapper::rustfmt(out + "}").die("Could not format the input")
}
fn parse_expr(expr: Pair<Rule>, global: &mut Global) -> String {
    match expr.as_rule() {
        Rule::Newline => "\n".into(),
        Rule::Comment => parse_comment(expr.as_str()),
        Rule::Def => parse_def(expr.into_inner(), global),
        Rule::Assign => parse_assig(expr.into_inner(), global),
        Rule::Print => parse_print(expr.into_inner()),
        _ => String::new(),
    }
}

fn parse_comment(comment: &str) -> String {
    format!("/* {} */", comment)
}

fn parse_def(mut pairs: Pairs<Rule>, global: &mut Global) -> String {
    let name = pairs.next().unwrap().as_str();
    let rhs = pairs.next().unwrap();
    let rhs: String = match rhs.as_rule() {
        Rule::Name | Rule::Int | Rule::Float | Rule::String => rhs.as_str().into(),
        Rule::Op => parse_op(rhs.into_inner()).into(),
        _ => "".into(),
    };

    global.variables.insert(name.into());
    match rhs.is_empty() {
        true => format!("let mut {};", name),
        false => format!("let mut {} = {};", name, rhs),
    }
}

fn parse_assig(mut pairs: Pairs<Rule>, global: &mut Global) -> String {
    let name = pairs.next().unwrap().as_str();
    let rhs = pairs.next().unwrap();
    let rhs: String = match rhs.as_rule() {
        Rule::Name | Rule::Int | Rule::Float | Rule::String => rhs.as_str().into(),
        Rule::Op => parse_op(rhs.into_inner()).into(),
        _ => "".into(),
    };

    match global.variables.contains(name) {
        true => format!("{} = {};", name, rhs),
        false => {
            global.variables.insert(name.into());
            format!("let mut {} = {};", name, rhs)
        }
    }
}

fn parse_op(mut pairs: Pairs<Rule>) -> String {
    let pairs = pairs.next().unwrap();
    let sym = match pairs.as_rule() {
        Rule::Add => '+',
        Rule::Sub => '-',
        Rule::Mul => '*',
        Rule::Div => '/',
        _ => '\0',
    };
    
    let mut pairs = pairs.into_inner();
    let lhs = pairs.next().unwrap();
    let lrule = lhs.as_rule();
    let rhs = pairs.next().unwrap();
    
    let lhs_s = match lhs.as_rule() {
        Rule::Op => parse_op(lhs.into_inner()),
        Rule::Int if rhs.as_rule() == Rule::Float => lhs.as_str().to_owned() + ".0",
        Rule::Float => lhs.as_str().replace(',', "."),
        _ => lhs.as_str().into()
    };
    
    let rhs_s = match rhs.as_rule() {
        Rule::Op => parse_op(rhs.into_inner()),
        Rule::Int if lrule == Rule::Float => rhs.as_str().to_owned() + ".0",
        Rule::Float => rhs.as_str().replace(',', "."),
        _ => rhs.as_str().into()
    };

    format!("{} {} {}", lhs_s, sym, rhs_s)
}

fn parse_print(pairs: Pairs<Rule>) -> String {
    let mut res = String::from("println!(\"");
    let mut rhs = String::new();
    for pair in pairs {
        res += "{}";
        rhs += &(", ".to_owned()
            + &match pair.as_rule() {
                Rule::Name | Rule::Int | Rule::String => pair.as_str().into(),
                Rule::Float => pair.as_str().replace(',', ".").into(),
                Rule::Op => parse_op(pair.into_inner()),
                _ => "".into(),
            });
    }

    format!("{}\"{});", res, rhs)
}
