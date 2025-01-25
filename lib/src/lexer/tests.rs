// test basic lexing

mod lexemes {
    use crate::lexer::{literal::NumericValue, Lexeme, Lexer, Token};

    fn test_str_single_output(s: &str, value: &str) {
        let mut lexer = Lexer::new(s);
        let parsed = lexer.next();
        assert!(parsed.is_some());
        let parsed_value = match parsed.unwrap() {
            Lexeme::StringLiteral(p) => p,
            _ => panic!("String literal matched a different lexeme")
        };
        assert_eq!(parsed_value.value(), value);
        assert_eq!(parsed_value.raw(), s);
    }

    fn test_numeral_single_output(s: &str, value: NumericValue) {
        let mut lexer = Lexer::new(s);
        let parsed = lexer.next();
        assert!(parsed.is_some());
        let parsed_value = match parsed.unwrap() {
            Lexeme::NumericLiteral(p) => p,
            _ => panic!("Numeric literal matched a different lexeme")
        };
        assert_eq!(parsed_value.value(), value);
        assert_eq!(parsed_value.raw(), s);
    }

    #[test]
    fn string_literal_single_quote() {
        let sq: &str = "hey heres a string";
        let wrapped = format!("'{}'", sq);
        test_str_single_output(&wrapped, sq);
    }

    #[test]
    fn string_literal_double_quote() {
        let dq: &str = "hey heres another string";
        let wrapped = format!("\"{}\"", dq);
        test_str_single_output(&wrapped, dq);
    }

    #[test]
    fn string_literal_long() {
        let long: &str = "this is a long string with two equals";
        let wrapped = format!("[==[{}]==]", long);
        test_str_single_output(&wrapped, long);
    }

    #[test]
    fn basic_decimal() {
        let s = "115";
        let val = 115;
        let wrapped = NumericValue::Integer(val);
        test_numeral_single_output(s, wrapped);
    }

    #[test]
    fn float_decimal() {
        let s = "3.1415";
        let val = 3.1415;
        let wrapped = NumericValue::Float(val);
        test_numeral_single_output(s, wrapped);
    }

    #[test]
    fn exp_decimal() {
        let s = "0.31415e1";
        let val = 3.1415;
        let wrapped = NumericValue::Float(val);
        test_numeral_single_output(s, wrapped);
    }

    #[test]
    fn basic_hex() {
        // test fails, need to work on hex parsing
        let s = "0xA1";
        let val = 0xA1;
        let wrapped = NumericValue::Integer(val);
        test_numeral_single_output(s, wrapped);
    }
}