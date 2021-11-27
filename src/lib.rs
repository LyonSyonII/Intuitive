mod parser;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn basic_parsing() {
		assert_eq!(parser::parse_string("\n"	), "\n"		);
		assert_eq!(parser::parse_string("\n\n"	), "\n\n"	);
		assert_eq!(parser::parse_string(" "		), ""		);
		assert_eq!(parser::parse_string("   "	), ""		);
		assert_eq!(parser::parse_string("is to" ), ""		);
	}
	
	#[test]
	fn assign() {
		assert_eq!(parser::parse_string("A is equal to 10."), "let mut A = 10.0;");
		assert_eq!(parser::parse_string("A equal 1."), "let mut A = 1.0;");
		assert_eq!(parser::parse_string("A = 999."), "let mut A = 999.0;");
		
		assert_eq!(parser::parse_string("A is equal to 10,5."), "let mut A = 10.5;");
		assert_eq!(parser::parse_string("A equal 15,1."), "let mut A = 15.1;");
		assert_eq!(parser::parse_string("A = 0,56."), "let mut A = 0.56;");
		
		assert_eq!(parser::parse_string("A is equal to \"Hello\"."), "let mut A = \"Hello\";");
		assert_eq!(parser::parse_string("A equal \"Hello\"."), "let mut A = \"Hello\";");
		assert_eq!(parser::parse_string("A = \"Hello\"."), "let mut A = \"Hello\";");
		
		assert_eq!(parser::parse_string("A = 10.\nB = A."), "let mut A = 10.0;\nlet mut B = A;");
	}
}
