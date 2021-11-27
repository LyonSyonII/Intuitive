use die::{Die, die};
use pest::{iterators::Pair, iterators::Pairs, Parser as P};
use std::fs::File;
use std::io::*;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct Parser;

#[derive(Default)]
struct Global {
    variables: std::collections::HashMap<String, Rule>,
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

pub fn parse_string(inp: &str) -> String {
	let parse = Parser::parse(Rule::MAIN, &inp);
	let parse = match parse {
		Ok(p) => p,
		Err(e) => die!(e.to_string()),
	};

	let mut global = Global::default();
	let mut out = String::new();
	for expr in parse {
		out += &parse_expr(expr, &mut global);
	}
	out
}

fn parse_expr(expr: Pair<Rule>, global: &mut Global) -> String {
    match expr.as_rule() {
        Rule::Newline => "\n".into(),
        Rule::Comment => parse_comment(expr.as_str()),
        Rule::Def => parse_def(expr.into_inner(), global),
        Rule::Assign => parse_assig(expr.into_inner(), global),
        Rule::Print => parse_print(expr.into_inner(), global),
        _ => String::new(),
    }
}

fn parse_comment(comment: &str) -> String {
    format!("/* {} */", comment)
}

fn parse_def(mut pairs: Pairs<Rule>, global: &mut Global) -> String {
    let name = pairs.next().unwrap().as_str();
    let rhs = pairs.next().unwrap();
    let rule = rhs.as_rule();
    let rhs: String = match rule {
        Rule::Name | Rule::Int | Rule::Float | Rule::String => rhs.as_str().into(),
        Rule::Op => parse_op(rhs.into_inner(), &global).into(),
        _ => "".into(),
    };

    global.variables.insert(name.into(), rule);
    match rhs.is_empty() {
        true => format!("let mut {};", name),
        false => format!("let mut {} = {};", name, rhs),
    }
}

fn parse_assig(mut pairs: Pairs<Rule>, global: &mut Global) -> String {
    let name = pairs.next().unwrap().as_str();
    let rhs = pairs.next().unwrap();
    let mut rule = rhs.as_rule();
    let rhs: String = match rule {
        Rule::String => rhs.as_str().into(),
        Rule::Int => rhs.as_str().to_owned() + ".0",
        Rule::Float => rhs.as_str().replace(',', "."),
        Rule::Name => {
        	let ret = rhs.as_str().into();
        	rule = global.variables[ret];
        	ret.into()
        }
        Rule::Op => parse_op(rhs.into_inner(), &global).into(),
        _ => "".into(),
    };

    match global.variables.contains_key(name) && rule == global.variables[name] {
        true => format!("{} = {};", name, rhs),
        false => {
        	global.variables.insert(name.into(), rule);
            format!("let mut {} = {};", name, rhs)
        }
    }
}

fn parse_op(mut pairs: Pairs<Rule>, global: &Global) -> String {
    let pairs = pairs.next().unwrap();
    let sym = match pairs.as_rule() {
        Rule::Add => '+',
        Rule::Sub => '-',
        Rule::Mul => '*',
        Rule::Div => '/',
        _ => '\0',
    };

	let parse_side = |hs: Pair<Rule>| -> String {
    	match hs.as_rule() {
    		Rule::Op => parse_op(hs.into_inner(), &global),
    		Rule::Int /*if rrule == Rule::Float*/ => hs.as_str().to_owned() + ".0",
    		Rule::Float => hs.as_str().replace(',', "."),
    		_ => hs.as_str().into()
    	}
	};
    
    let mut pairs = pairs.into_inner();
    let lhs = parse_side(pairs.next().unwrap());
    let rhs = parse_side(pairs.next().unwrap());
	/*
    if lrule == Rule::Name && rrule == Rule::Name {
    	let lrule = global.variables[&lhs];
    	println!("Lhs rule: {:?}", lrule);
    	let rrule = global.variables[&rhs];
    	println!("Rhs rule: {:?}", rrule);
    	
    	if lrule != rrule {
    		match lrule {
    			Rule::Int => lhs = format!("({} as f32)", lhs),
    			Rule::Float => rhs = format!("({} as f32)", rhs),
    			_ => {},
    		}
    	}
    }
    */

    format!("{} {} {}", lhs, sym, rhs)
}

fn parse_print(pairs: Pairs<Rule>, global: &Global) -> String {
    let mut res = String::from("println!(\"");
    let mut rhs = String::new();
    for pair in pairs {
        res += "{}";
        rhs += &(", ".to_owned()
            + &match pair.as_rule() {
                Rule::Name | Rule::Int | Rule::String => pair.as_str().into(),
                Rule::Float => pair.as_str().replace(',', ".").into(),
                Rule::Op => parse_op(pair.into_inner(), &global),
                _ => "".into(),
            });
    }

    format!("{}\"{});", res, rhs)
}
