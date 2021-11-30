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
        "#[allow(non_snake_case)]\n#[allow(dead_code)]\n#[allow(unused_variables)]\n#[allow(unused_mut)]\n#[allow(unused_assignments)]\nfn main() {"
            .into();

    let mut global = Global::new();
    for expr in parse {
        out += &parse_expr(expr, &mut global)
    }

    //out + "}"
    rustfmt_wrapper::rustfmt(out + "}" + include_str!("read_fn.txt"))
        .die("ERROR: Rustfmt could not format the input")
}

fn die(err: &str, line: u64, ctx: &str) -> ! {
    die!("\nERROR:   {} {}.\nContext: {}", err, line, ctx)
}

fn die_corr(err: &str, line: u64, corr: &str, ctx: &str) -> ! {
    die!(
        "\nERROR:   {} {}.\n         {}\nContext: {}",
        err,
        corr,
        line,
        ctx
    )
}

fn check_errors(expr: Pair<Rule>, global: &Global) -> ! {
    let _die = |err: &str| -> ! { die(err, global.line_num, expr.as_str()) };
    let _die_corr =
        |err: &str, corr: &str| -> ! { die_corr(err, global.line_num, corr, expr.as_str()) };
    match expr.as_rule() {
        Rule::EmptyStr => _die_corr("Empty string in line", "Strings cannot be empty"),
        Rule::NotDot => _die("Expected dot in line"),
        Rule::NotUpper => _die("Variable not starting with UPPERCASE letter in line"),
        Rule::NotVarRead => _die_corr("Incorrect read in line", "Only variables can be read"),
        Rule::ReadFmtStr => _die_corr("Printing more than one String in Read in line", "You can only print one message on Read, if you want to print an elaborate message, use a Print before."),
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
        let (ret, rule) = parse_rhs(pair, global);
        if rule == Rule::TypeInt || rule == Rule::TypeFloat || rule == Rule::TypeStr {
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
        (
            pair.as_str(),
            pairs.next().unwrap().as_str(),
        )
    } else {
        ("\"\"", pair.as_str())
    };

    if let Some(ty) = pairs.next() {
        let rule = ty.as_rule();
        let ty = match rule {
            Rule::TypeInt => "i64",
            Rule::TypeFloat => "f64",
            Rule::TypeStr => "String",
            _ => "",
        };

        global.variables.insert(name.into(), type_to_rule(rule));
        format!("let {} = read::<{}>({});", name, ty, message)
    } else {
        if global.variables.get(name).is_none() {
            die(
                "Trying to Read into non initialized variable in line",
                global.line_num,
                &global.line_str,
            )
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
            parse_op(pairs.next().unwrap().into_inner(), &global).0
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

    let (name, rhs) = if reverse {
        (rhs.as_str(), parse_rhs(lhs, &global).0)
    } else {
        (lhs.as_str(), parse_rhs(rhs, &global).0)
    };

    format!("{} {} {};", name, sym, rhs)
}

fn parse_list(mut pairs: Pairs<Rule>, global: &mut Global) -> String {
    let name = pairs.next().unwrap().as_str();
    let list_ty = pairs.next().unwrap().as_rule();
    let ty = match list_ty {
        Rule::TypeInt => Rule::Int,
        Rule::TypeFloat => Rule::Float,
        Rule::TypeStr => Rule::String,
        _ => Rule::Err,
    };

    let mut rhs = String::new();
    for elem in pairs {
        let (ret, mut rule) = parse_rhs(elem, global);
        if rule == Rule::FmtString {
            rule = Rule::String
        } else if rule == Rule::Int && ty == Rule::Float {
            rule = Rule::Float
        } else if rule == Rule::Float && ty == Rule::Int {
            rule = Rule::Int
        }

        if rule != ty {
            die_corr(
                &format!(
                    "Variable of type {:?} in a list of type {:?} in line",
                    rule, ty
                ),
                global.line_num,
                "Lists can only contain elements of the same type.\n",
                &global.line_str,
            )
        }

        rhs += &format!("{}, ", ret);
    }

    global.variables.insert(name.into(), list_ty);
    format!("let {} = Vec::from([{}]);", name, rhs)
}

fn parse_op(mut pairs: Pairs<Rule>, global: &Global) -> (String, Rule) {
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

    let parse_side = |hs: Pair<Rule>| -> (String, Rule) {
        /*
        match hs.as_rule() {
            Rule::Op | Rule::Cmp => parse_op(hs.into_inner(), &global),
            Rule::Int => (hs.as_str().to_owned() + ".0", Rule::Int),
            Rule::Float => (hs.as_str().replace(',', "."), Rule::Float),
            Rule::Name => {
                let hs = hs.as_str().to_owned();
                let rule = *global.variables.get(&hs).unwrap_or_else(|| {
                    die(
                        "Variable not initialized in line",
                        global.line_num,
                        hs.as_str(),
                    )
                });
                (hs, rule)
            }
            _ => (hs.as_str().into(), Rule::WHITESPACE),
        }
         */
        match hs.as_rule() {
            Rule::String | Rule::FmtString => die_corr("Operation with string in line", global.line_num, "Strings cannot be added, use formatting instead: e.g. Print \"The value of A is: \" A.", &global.line_str),
            _ => parse_rhs(hs, global)
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
                    &format!("Variable \"{}\" not initialized in line", ret),
                    global.line_num,
                    &global.line_str,
                )
            });
            ret.into()
        }
        Rule::Op | Rule::Cmp => {
            let ret = parse_op(rhs.into_inner(), &global);
            rule = ret.1;
            ret.0
        }
        _ => "".into(),
    };

    (rhs, rule)
}

fn parse_fmt_string(pairs: Pairs<Rule>, global: &Global) -> String {
    let mut lhs = String::from("format!(\"");
    let mut rhs = String::new();
    for pair in pairs {
        lhs += "{}";
        let expr = if pair.as_rule() == Rule::Op {
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
        Rule::TypeInt => Rule::Int,
        Rule::TypeFloat => Rule::Float,
        Rule::TypeStr => Rule::String,
        _ => Rule::Err,
    }
}
