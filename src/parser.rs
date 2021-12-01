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
    errors: String,
}

impl Global {
    fn new() -> Global {
        Global {
            variables: std::collections::HashMap::new(),
            line_num: 1,
            line_str: String::new(),
            errors: String::new(),
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
        "#![allow(non_snake_case)]\n#![allow(dead_code)]\n#![allow(unused_variables)]\n#![allow(unused_mut)]\n#![allow(unused_assignments)]\nfn main() {"
            .into();

    let mut global = Global::new();
    for expr in parse {
        out += &parse_expr(expr, &mut global)
    }
    
    for var in global.variables {
        if var.1 == Rule::NotInit {
            global.errors += &format!("\nERROR: Unused variable \"{}\".\n       All variables must be used, remove the ones you don't want.", var.0)
        }
    }
    
    if !global.errors.is_empty() {
        die!("{}", global.errors);
    }

    //out + "}"
    rustfmt_wrapper::rustfmt(out + "}" + include_str!("read_fn.txt"))
        .die("ERROR: Rustfmt could not format the input")
}

fn die(err: &str, global: &mut Global) -> String {
    global.errors += &format!("\nERROR:   {} {}.\nContext: {}.\n", err, global.line_num, global.line_str);
    String::new()
}

fn die_corr(err: &str, corr: &str, global: &mut Global) -> String {
    global.errors += &format!(
        "\nERROR:   {} {}.\n         {}\nContext: {}.\n",
        err,
        global.line_num,
        corr,
        global.line_str
    );

    String::new()
}

fn check_errors(expr: Pair<Rule>, global: &mut Global) -> String {
    let mut _die = |err: &str| -> String { die(err, global) };
    let mut _die_corr =
        |err: &str, corr: &str| -> String { die_corr(err, corr, global) };

    match expr.as_rule() {
        Rule::EmptyStr => die_corr("Empty string in line", "Strings cannot be empty", global),
        Rule::NotDot => die("Expected dot in line", global),
        Rule::NotUpper => die_corr(&format!("Variable with name \"{}\" does not start with UPPERCASE letter in line", expr.as_str()), "Variables must start with an UPPERCASE letter.", global),
        
        Rule::NotVarRead => die_corr("Incorrect read in line", "Only variables can be read", global),
        Rule::ReadFmtStr => die_corr("Printing more than one String with Read in line", "You can only print one message with Read, if you want to print an elaborate message, use a Print before.", global),

        _ => die("Unexpected error, please post an issue to https://github.com/LyonSyonII/Intuitive with your code file.", global),
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
        Rule::Read => parse_read(expr.into_inner(), global),
        Rule::If => parse_if(Rule::If, expr.into_inner(), global),
        Rule::Else => parse_if(Rule::Else, expr.into_inner(), global),
        Rule::ElseIf => parse_if(Rule::ElseIf, expr.into_inner(), global),
        Rule::AddEq => parse_op_eq("+=", true, expr.into_inner(), global),
        Rule::SubEq => parse_op_eq("-=", true, expr.into_inner(), global),
        Rule::MulEq => parse_op_eq("*=", false, expr.into_inner(), global),
        Rule::DivEq => parse_op_eq("/=", false, expr.into_inner(), global),
        Rule::List => parse_list(expr.into_inner(), global),
        _ => String::new(),
    }
}

fn parse_comment(comment: &str) -> String {
    format!("/* {} */", comment)
}

fn parse_def(mut pairs: Pairs<Rule>, global: &mut Global) -> String {
    let name = pairs.next().unwrap().as_str();
    if let Some(rhs) = pairs.next() {
        let rule = rhs.as_rule();
        if is_type(rhs.as_rule()) {
            global.variables.insert(name.into(), type_to_rule(rule));
            format!("let mut {}: {};", name, type_to_str(rule))
        }
        else {
            let (rhs, rule) = parse_rhs(rhs, global);
            global.variables.insert(name.into(), rule);
            format!("let mut {} = {};", name, rhs)
        }
        
    } else {
        global.variables.insert(name.into(), Rule::NotInit);
        format!("let mut {};", name)
    }
}

fn parse_assig(mut pairs: Pairs<Rule>, global: &mut Global) -> String {
    let name = pairs.next().unwrap().as_str();
    let (rhs, rule) = parse_rhs(pairs.next().unwrap(), global);
    
    // Canged rule == *ty by is_same_type()
    let decl = if let Some(ty) = global.variables.get(name) {
        is_same_type(rule, *ty) || rule == Rule::NotInit
    } else {
        false
    };
    
    if decl {
        format!("{} = {};", name, rhs)
    } else {
        global.variables.insert(name.into(), rule);
        format!("let mut {} = {};", name, rhs)
    }
}

fn parse_print(pairs: Pairs<Rule>, global: &mut Global) -> String {
    let mut lhs = String::from("println!(\"");
    let mut rhs = String::new();
    for pair in pairs {
        let (ret, rule) = parse_rhs(pair, global);
        if is_type(rule) {
            lhs += "{:?}";
        } else {
            lhs += "{}";
        }

        rhs = format!("{}, {}", rhs, ret);
    }

    format!("{}\"{});", lhs, rhs)
}

fn parse_read(mut pairs: Pairs<Rule>, global: &mut Global) -> String {
    let pair = pairs.next().unwrap();
    let (message, name) = if pair.as_rule() == Rule::String {
        (pair.as_str(), pairs.next().unwrap().as_str())
    } else {
        ("\"\"", pair.as_str())
    };

    if let Some(ty) = pairs.next() {
        let rule = ty.as_rule();

        global.variables.insert(name.into(), type_to_rule(rule));
        format!("let {} = read::<{}>({});", name, type_to_str(rule), message)
    } else {
        if global.variables.get(name).is_none() {
            die(
                "Trying to Read into non declared variable in line",
                global
            );
        }

        format!("{} = read({});", name, message)
    }
}

fn parse_if(rule: Rule, mut pairs: Pairs<Rule>, global: &mut Global) -> String {
    let mut lhs = String::new();
    if rule != Rule::If {
        lhs += "else ";
    }

    if rule != Rule::Else {
        lhs += &format!(
            "if {}",
            parse_op(pairs.next().unwrap().into_inner(), global).0
        );
    }

    lhs += "{";

    for pair in pairs {
        lhs += &parse_expr(pair, global);
    }

    lhs + "}"
}

fn parse_op_eq(sym: &str, reverse: bool, mut pairs: Pairs<Rule>, global: &mut Global) -> String {
    let lhs = pairs.next().unwrap();
    let rhs = pairs.next().unwrap();
    // Reverse is +, -, List
    let (name, rhs) = if reverse {
        let add_list = parse_add_list(rhs.as_str(), lhs.clone(), global);
        if add_list != String::new() {
            return add_list;
        }

        (rhs.as_str(), parse_rhs(lhs, global).0)
    } else {
        (lhs.as_str(), parse_rhs(rhs, global).0)
    };

    format!("{} {} {};", name, sym, rhs)
}

fn parse_list(mut pairs: Pairs<Rule>, global: &mut Global) -> String {
    let name = pairs.next().unwrap().as_str();
    let list_ty = pairs.next().unwrap().as_rule();
    let ty = type_to_rule(list_ty);

    let mut rhs = String::new();
    for elem in pairs {
        let (ret, mut rule) = parse_rhs(elem, global);
        if rule == Rule::FmtString {
            rule = Rule::String
        }

        if !is_same_type(rule, ty) {
            die_corr(
                &format!(
                    "Variable of type {:?} in a list of type {:?} in line",
                    rule, ty
                ),
                "Lists can only contain elements of the same type.",
                global
            );
        }

        rhs += &format!("{}, ", ret);
    }

    global.variables.insert(name.into(), list_ty);
    format!("let mut {} = Vec::from([{}]);", name, rhs)
}

fn parse_add_list(name: &str, lhs: Pair<Rule>, global: &mut Global) -> String {
        if let Some(list) = global.variables.clone().get(name) {
            if is_type(*list) {
                let (mut lhs, ty) = parse_rhs(lhs, global);
                let list = type_to_rule(*list);
                if !is_same_type(ty, list) {
                    let err = format!("Trying to add element of type {:?} to List of type {:?} in line", ty, list);
                    die_corr(&err, "Lists can only contain elements of the same type.", global);
                }
                if ty == Rule::String { lhs += ".into()" }
                format!("{}.push({});", name, lhs) 
            }
            else { String::new() }
        }
        else {
            String::new()
        }
}

fn parse_op(mut pairs: Pairs<Rule>, global: &mut Global) -> (String, Rule) {
    let pairs = pairs.next().unwrap();
    let sym = match pairs.as_rule() {
        Rule::Add => "+",
        Rule::Sub => "-",
        Rule::Mul => "*",
        Rule::Div => "/",
        Rule::Lower => "<",
        Rule::LowEq => "<=",
        Rule::Greater => ">",
        Rule::GreatEq => ">=",
        Rule::EqCmp => "==",
        Rule::NotEq => "!=",
        Rule::And => "&&",
        Rule::Or => "||",
        _ => "",
    };

    let mut parse_side = |hs: Pair<Rule>| -> (String, Rule) {
        match hs.as_rule() {
            Rule::String | Rule::FmtString => (
                die_corr(
                    "Operation with string in line",  
                "Strings cannot be added, use formatting instead: e.g. Print \"The value of A is: \" A.", global
            ), Rule::Err),
            _ => parse_rhs(hs, global)
        }
    };

    let mut pairs = pairs.into_inner();
    let lhs = parse_side(pairs.next().unwrap());
    let rhs = parse_side(pairs.next().unwrap());
    let rule = if lhs.1 == rhs.1 { lhs.1 } else { Rule::Float };
    (format!("{} {} {}", lhs.0, sym, rhs.0), rule)
}

fn parse_rhs(rhs: Pair<Rule>, global: &mut Global) -> (String, Rule) {
    let mut rule = rhs.as_rule();
    let rhs = match rule {
        Rule::String => rhs.as_str().into(),
        Rule::FmtString => parse_fmt_string(rhs.into_inner(), global),
        Rule::Int => rhs.as_str().to_owned() + ".0",
        Rule::Float => rhs.as_str().replace(',', "."),
        Rule::Name => {
            let ret = rhs.as_str().into();
            rule = *global.variables.clone().get(ret).unwrap_or_else(|| {
                die(
                    &format!("Variable \"{}\" not initialized in line", ret),
                    global
                );
                &Rule::Err
            });
            ret.into()
        }
        Rule::Op | Rule::Cmp => {
            let ret = parse_op(rhs.into_inner(), global);
            rule = ret.1;
            ret.0
        }
        _ => check_errors(rhs, global),
    };

    (rhs, rule)
}

fn parse_fmt_string(pairs: Pairs<Rule>, global: &mut Global) -> String {
    let mut lhs = String::from("format!(\"");
    let mut rhs = String::new();
    for pair in pairs {
        let rule = pair.as_rule();
        if rule == Rule::Name { 
            if let Some(var) = global.variables.get(pair.as_str()) {
                if is_type(*var) {
                    lhs += "{:?}" 
                }
            }
        }
        else { lhs += "{}" }
    
        let expr = if rule == Rule::Op {
            parse_op(pair.into_inner(), global).0
        } else {
            pair.as_str().into()
        };
        rhs += &format!(", {}", expr);
    }
    
    format!("{}\"{})", lhs, rhs)
}

fn type_to_rule(ty: Rule) -> Rule {
    match ty {
        //Rule::TypeInt => Rule::Int,
        //Rule::TypeFloat => Rule::Float,
        Rule::TypeInt | Rule::TypeFloat => Rule::Float,
        Rule::TypeStr => Rule::String,
        _ => Rule::Err,
    }
}

fn type_to_str(ty: Rule) -> &'static str {
    match ty {
        //Rule::TypeInt => "i64",
        //Rule::TypeFloat => "f64",
        Rule::TypeInt | Rule::TypeFloat => "f64",
        Rule::TypeStr => "String",
        _ => "",
    }
}

fn is_same_type(rule1: Rule, rule2: Rule) -> bool {
    match rule1 {
        Rule::Int | Rule::Float => rule2 == Rule::Float || rule2 == Rule::Int,
        Rule::String | Rule::FmtString => rule2 == Rule::String || rule2 == Rule::FmtString,
        Rule::Err | Rule::NotUpper => true,
        _ => false,
    }
}

fn is_int(rule1: Rule, rule2: Rule) -> bool {
    rule1 == Rule::Int && rule2 == Rule::Float || rule1 == Rule::Float && rule2 == Rule::Int
}

fn is_type(rule: Rule) -> bool {
    match rule {
        Rule::TypeInt | Rule::TypeFloat | Rule::TypeStr | Rule::Err | Rule::NotUpper => true,
        _ => false,
    }
}
