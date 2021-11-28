mod parser;

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_parse(inp: &str, out: &str) {
        assert_eq!(parser::parse_string(inp), out);
    }

    #[test]
    fn basic_parsing() {
        assert_parse("\n", "\n");
        assert_parse("\n\n", "\n\n");
        assert_parse(" ", "");
        assert_parse("   ", "");
        assert_parse("is to", "");
    }

    #[test]
    fn assign() {
        assert_parse("A is equal to 10.", "let mut A = 10.0;");
        assert_parse("A equal 1.", "let mut A = 1.0;");
        assert_parse("A = 999.", "let mut A = 999.0;");

        assert_parse("A is equal to 10,5.", "let mut A = 10.5;");
        assert_parse("A equal 15,1.", "let mut A = 15.1;");
        assert_parse("A = 0,56.", "let mut A = 0.56;");

        assert_parse("A is equal to \"Hello\".", "let mut A = \"Hello\";");
        assert_parse("A equal \"Hello\".", "let mut A = \"Hello\";");
        assert_parse("A = \"Hello\".", "let mut A = \"Hello\";");

        assert_parse("A = 10.\nB = A.", "let mut A = 10.0;\nlet mut B = A;");
    }

    fn gen_op_test(equals: &[&str], operations: &[&str], correct_eq: &str, correct_op: &str) {
        for eq in equals {
            for op in operations {
                for (x, y) in &[
                    ("5.0", "10.0"),
                    ("3,1416", "1,0"),
                    ("3.0", "25,125"),
                    ("A", "1.0"),
                    ("A", "2,5"),
                    ("A", "A"),
                ] {
                    let test = format!(
                        "A {} {} {} {}.",
                        eq,
                        x.replace(".0", ""),
                        op,
                        y.replace(".0", "")
                    );
                    let answer = format!(
                        "let mut A {} {} {} {};",
                        correct_eq,
                        x.replace(',', "."),
                        correct_op,
                        y.replace(',', ".")
                    );
                    println!("Test: {}", test);
                    println!("Answer: {}", &answer);
                    assert_parse(&test, &answer);
                }
            }
        }
    }

    #[test]
    fn add() {
        gen_op_test(
            &["is equal to", "equal", "es igual a", "="],
            &["plus", "mes", "+"],
            "=",
            "+",
        );
    }

    #[test]
    fn sub() {
        gen_op_test(
            &["is equal to", "equal", "es igual a", "="],
            &["minus", "menys", "-"],
            "=",
            "-",
        );
    }

    #[test]
    fn mul() {
        gen_op_test(
            &["is equal to", "equal", "es igual a", "="],
            &["times", "multiplied by", "per", "multiplicat per", "*"],
            "=",
            "*",
        );
    }

    #[test]
    fn div() {
        gen_op_test(
            &["is equal to", "equal", "es igual a", "="],
            &["divided by", "entre", "dividit entre"],
            "=",
            "/",
        );
    }
}
