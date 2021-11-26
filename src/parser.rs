use std::io::*;
use std::fs::File;
use die::Die;
use pest::{ Parser as P, iterators::Pair, iterators::Pairs };

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct Parser;

pub fn parse_file(file: &mut File) -> String {
    // Read file into string
    let mut inp = String::new();
    file.read_to_string(&mut inp).die("Unable to read the file");
    let parse = Parser::parse(Rule::MAIN, &inp).die("Error parsing the file");
    
    let mut out: String = "fn main() {".into();
    // Read instructions
    for expr in parse {
        out += &parse_expr(expr)
    }
    rustfmt_wrapper::rustfmt(out + "}").die("Could not format the input")
}

fn parse_expr(expr: Pair<Rule>) -> String {
    match expr.as_rule() {
        Rule::Assign => parse_assig(expr.into_inner()),
        Rule::Print => parse_print(expr.into_inner()),
        _ => { String::new() }
    }
}

fn parse_assig(mut pairs: Pairs<Rule>) -> String {
    let name = pairs.next().unwrap().as_str();
    let rhs = pairs.next().unwrap();
    let rhs: String = 
        match rhs.as_rule() {
            Rule::Name | Rule::Int | Rule::Float | Rule::String => rhs.as_str().into(),
            Rule::Op => parse_op(rhs.into_inner()).into(),
            _ => "".into(),
        };
    
    format!("let {} = {};", name, rhs)
}

fn parse_op(mut pairs: Pairs<Rule>) -> String {
    let pairs = pairs.next().unwrap();
    let sym = 
        match pairs.as_rule() {
            Rule::Add => '+',
            Rule::Sub => '-',
            Rule::Mul => '*',
            Rule::Div => '/',
            _ => '\0',
        };
    
    let mut pairs = pairs.into_inner();
    let lhs = pairs.next().unwrap();
    let rhs = pairs.next().unwrap();
    
    let lhs = 
        if lhs.as_rule() == Rule::Op {
            // Return underlying operation
            parse_op(lhs.into_inner())
        }
        else {
            // Return Value itself
            lhs.as_str().into()
        };
    
    let rhs = 
        if rhs.as_rule() == Rule::Op {
            // Return underlying operation
            parse_op(rhs.into_inner())
        }
        else {
            // Return Value itself
            rhs.as_str().into()
        };

    format!("{} {} {}", lhs, sym, rhs)
}

fn parse_print(mut pairs: Pairs<Rule>) -> String {
    
}
