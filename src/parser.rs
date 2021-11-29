// TODO: Add compile error for nested if
// TODO: Find way to express loops and allow to nest one if

#![allow(dead_code)]

use die::{die, Die};
use pest::{iterators::Pair, iterators::Pairs, Parser as P};
use std::fs::File;
use std::io::*;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct Parser;

#[derive(Default, Debug)]
struct Global {
    variables: std::collections::HashMap<String, Rule>,
    line_num: u64,
    line_str: String,
}

impl Global {
    fn new() -> Global {
        Global {
            variables: std::collections::HashMap::new(),
            line_num: 1,
            line_str: String::new(),
        }
    }
}

pub fn parse_string(inp: &str) -> String {
    let parse = Parser::parse(Rule::MAIN, &inp);
    let parse = match parse {
        Ok(p) => p,
        Err(e) => die!(e.to_string()),
    };

    let mut global = Global::new();
    let mut out = String::new();
    for expr in parse {
        out += &parse_expr(expr, &mut global);
    }
    out
}

pub fn parse_file(file: &mut File) -> String {
    let mut inp = String::new();
    file.read_to_string(&mut inp).die("Unable to read the file");
    let parse = Parser::parse(Rule::MAIN, &inp).die("Error parsing the file");

    let mut out: String =
        "#[allow(non_snake_case)]\n#[allow(unused_variables)]\n#[allow(unused_mut)]\n#[allow(unused_assignments)]\nfn main() {"
            .into();

    let mut global = Global::new();
    for expr in parse {
        out += &parse_expr(expr, &mut global)
    }

    //out + "}"
    rustfmt_wrapper::rustfmt(out + "}").die("ERROR: Rustfmt could not format the input")
}

fn die(err: &str, line: u64, corr: &str, ctx: &str) -> ! {
    die!("\nERROR:   {} {}.{}\nContext: {}", err, line, corr, ctx)
}

fn check_errors(expr: Pair<Rule>, global: &Global) -> ! {
    let _die = |err: &str| -> ! { die(err, global.line_num, "",expr.as_str()) };
    let _die_corr = |err: &str, corr: &str| -> !{ die(err, global.line_num, corr, expr.as_str())};
    match expr.as_rule() {
        Rule::NotDot => _die("Expected dot in line"),
        Rule::NotUpper => _die("Variable not starting with UPPERCASE letter in line"),
        _ => die!(),
    }
}

fn parse_expr(expr: Pair<Rule>, global: &mut Global) -> String {
    println!("{:?}", global);
    global.line_str = expr.as_str().into();
    
    match expr.as_rule() {
        Rule::Err => check_errors(expr.into_inner().next().unwrap(), global),
        Rule::Newline => {
            global.line_num += 1;
            "\n".into()
        }
        Rule::Comment => parse_comment(expr.as_str()),
        Rule::Def => parse_def(expr.into_inner(), global),
        Rule::Assign => parse_assig(expr.into_inner(), global),
        Rule::Print => parse_print(expr.into_inner(), global),
        Rule::If => parse_if(Rule::If, expr.into_inner(), global),
        Rule::Else => parse_if(Rule::Else, expr.into_inner(), global),
        Rule::ElseIf => parse_if(Rule::ElseIf, expr.into_inner(), global),
        _ => String::new(),
    }
}

fn parse_comment(comment: &str) -> String {
    format!("/* {} */", comment)
}

fn parse_def(mut pairs: Pairs<Rule>, global: &mut Global) -> String {
    let name = pairs.next().unwrap().as_str();
    let rhs = pairs.next().unwrap();
    let (rhs, rule) = parse_rhs(rhs, &global);

    global.variables.insert(name.into(), rule);
    match rhs.is_empty() {
        true => format!("let mut {};", name),
        false => format!("let mut {} = {};", name, rhs),
    }
}

fn parse_assig(mut pairs: Pairs<Rule>, global: &mut Global) -> String {
    let name = pairs.next().unwrap().as_str();
    let (rhs, rule) = parse_rhs(pairs.next().unwrap(), &global);

    let initialized = global.variables.contains_key(name)
        && (rule == Rule::Float || rule == global.variables[name]);
    match initialized {
        true => format!("{} = {};", name, rhs),
        false => {
            global.variables.insert(name.into(), rule);
            format!("let mut {} = {};", name, rhs)
        }
    }
}

fn parse_print(pairs: Pairs<Rule>, global: &Global) -> String {
    let mut lhs = String::from("println!(\"");
    let mut rhs = String::new();
    for pair in pairs {
        lhs += "{}";
        let pair = match pair.as_rule() {
            Rule::Name | Rule::Int | Rule::String => pair.as_str().into(),
            Rule::Float => pair.as_str().replace(',', "."),
            Rule::Op => parse_op(pair.into_inner(), &global).0,
            _ => "".into(),
        };
        rhs = format!("{}, {}", rhs, pair);
    }

    format!("{}\"{});", lhs, rhs)
}

fn parse_if(rule: Rule, mut pairs: Pairs<Rule>, global: &mut Global) -> String {
    let mut lhs = String::new();
    if rule != Rule::If {
        lhs += "else ";
    }

    if rule != Rule::Else {
        lhs += &format!(
            "if {}",
            parse_op(pairs.next().unwrap().into_inner(), &global).0
        );
    }

    lhs += "{";

    for pair in pairs {
        lhs += &parse_expr(pair, global);
    }

    lhs + "}"
}

fn parse_op(mut pairs: Pairs<Rule>, global: &Global) -> (String, Rule) {
    let pairs = pairs.next().unwrap();
    let sym = match pairs.as_rule() {
        Rule::Add => "+",
        Rule::Sub => "-",
        Rule::Mul => "*",
        Rule::Div => "/",
        Rule::Less => "<",
        Rule::LessEq => "<=",
        Rule::More => ">",
        Rule::MoreEq => ">=",
        Rule::EqCmp => "==",
        _ => "",
    };

    let parse_side = |hs: Pair<Rule>| -> (String, Rule) {
        match hs.as_rule() {
            Rule::Op => parse_op(hs.into_inner(), &global),
            Rule::Int => (hs.as_str().to_owned() + ".0", Rule::Int),
            Rule::Float => (hs.as_str().replace(',', "."), Rule::Float),
            Rule::Name => {
                let hs = hs.as_str().to_owned();
                let rule = *global.variables.get(&hs).unwrap_or_else(|| {
                    die(
                        "Variable not initialized in line",
                        global.line_num,
                        "",
                        hs.as_str(),
                    )
                });
                (hs, rule)
            }
            Rule::String | Rule::FmtString => die("Operation with string in line", global.line_num, "\n         Strings cannot be added, use formatting instead: e.g. Print \"The value of A is: \" A.", &global.line_str),
            _ => (hs.as_str().into(), Rule::WHITESPACE),
        }
    };

    let mut pairs = pairs.into_inner();
    let lhs = parse_side(pairs.next().unwrap());
    let rhs = parse_side(pairs.next().unwrap());
    let rule = if lhs.1 == rhs.1 { lhs.1 } else { Rule::Float };
    (format!("{} {} {}", lhs.0, sym, rhs.0), rule)
}

fn parse_rhs(rhs: Pair<Rule>, global: &Global) -> (String, Rule) {
    let mut rule = rhs.as_rule();
    let rhs = match rule {
        Rule::String => rhs.as_str().into(),
        Rule::FmtString => parse_fmt_string(rhs.into_inner(), global),
        Rule::Int => rhs.as_str().to_owned() + ".0",
        Rule::Float => rhs.as_str().replace(',', "."),
        Rule::Name => {
            let ret = rhs.as_str().into();
            rule = *global.variables.get(ret).unwrap_or_else(|| {
                die(
                    "Variable not initialized in line",
                    global.line_num,
                    "",
                    &global.line_str,
                )
            });
            ret.into()
        }
        Rule::Op => {
            let ret = parse_op(rhs.into_inner(), &global);
            rule = ret.1;
            ret.0
        }
        _ => "".into(),
    };

    (rhs, rule)
}

// TODO: Canviat pair.as_str per parse_op per trobar errors en la operacio

fn parse_fmt_string(pairs: Pairs<Rule>, global: &Global) -> String {
    let mut lhs = String::from("format!(\"");
    let mut rhs = String::new();
    for pair in pairs {
        lhs += "{}";
        //rhs += &format!(", {}", pair.as_str());
        let expr = if pair.as_rule() == Rule::Op {
        	parse_op(pair.into_inner(), global).0
        }
        else {
        	pair.as_str().into()
        };
        rhs += &format!(", {}", expr);
    }

    format!("{}\"{})", lhs, rhs)
}
