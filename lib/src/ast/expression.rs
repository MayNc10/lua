use std::cmp::Ordering;

use crate::lexer::{self, identifier, literal, operator, seperator, Lexeme, Lexer};

pub trait Expression {}
pub struct Literal {}
impl Expression for Literal {

}
pub struct TableExpression {}

#[derive(Clone, Debug)]
pub struct BinaryExpression {
    op: ExpOperation,
    lhs: Box<ExpOperand>,
    rhs: Box<ExpOperand>
}
impl Expression for BinaryExpression {

}

#[derive(Clone, Debug)]
pub struct UnaryExpression {
    op: ExpOperation,
    arg: Box<ExpOperand>
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ExpOperation {
    And,
    Or,
    LessThan,
    GreaterThan,
    LessEqual,
    GreaterEqual,
    NotEqual,
    Equals,
    Concat,
    Plus,
    Minus,
    Star,
    Slash,
    Not,
    Exp,
    OpenParen,
    CloseParen,
    UnaryMinus,
}

impl ExpOperation {
    pub fn precedence(first: ExpOperation, second: ExpOperation) -> Ordering {
        first.rank().cmp(&second.rank())
    }
    fn rank(&self) -> u8 {
        match self {
            ExpOperation::And | ExpOperation::Or => 0,
            ExpOperation::LessThan | ExpOperation::GreaterThan 
                | ExpOperation::LessEqual | ExpOperation::GreaterEqual 
                | ExpOperation::Equals | ExpOperation::NotEqual => 1,
            ExpOperation::Concat => 2,
            ExpOperation::Plus | ExpOperation::Minus => 3,
            ExpOperation::Star | ExpOperation::Slash => 4,
            ExpOperation::Not | ExpOperation::UnaryMinus => 5,
            ExpOperation::Exp => 6,
            ExpOperation::OpenParen | ExpOperation::CloseParen => 7,
        }
    }
}

#[derive(Clone, Debug)]
enum ExpOperand {
    NumericLiteral(literal::NumericLiteral),
    StringLiteral(literal::StringLiteral),
    Identifier(identifier::Identifier),
    FuncCall(identifier::Identifier),
    MethodCall(identifier::Identifier, identifier::Identifier),
    BinaryExp(BinaryExpression),
    UnaryExp(UnaryExpression)
}

// rework eventually
impl Expression for ExpOperand {}

pub fn parse_expression(lex: &mut Lexer) -> Option<Box<dyn Expression>> {
    // UGHHH I HATE SHUTNING YARD
    let mut operands = Vec::new();
    let mut operations = Vec::new();

    let mut last_was_arg = false;
    let mut opened_parens = 0;

    while let Some(tok) = lex.clone().peekable().peek() {
        match tok {
            Lexeme::Operator(op) => {
                lex.next();
                match op {
                    operator::Operator::LogicalAnd => {
                        assert!(last_was_arg);
                        last_was_arg = false;
                        operations.push(ExpOperation::And);
                    },
                    operator::Operator::LogicalOr => {
                        assert!(last_was_arg);
                        last_was_arg = false;
                        operations.push(ExpOperation::Or);
                    },
                    operator::Operator::LessEqual => {
                        assert!(last_was_arg);
                        last_was_arg = false;
                        operations.push(ExpOperation::LessEqual);
                    },
                    operator::Operator::GreaterEqual => {
                        assert!(last_was_arg);
                        last_was_arg = false;
                        operations.push(ExpOperation::GreaterEqual);
                    },
                    operator::Operator::Equal => {
                        assert!(last_was_arg);
                        last_was_arg = false;
                        operations.push(ExpOperation::Equals);
                    }
                    operator::Operator::NotEqual => {
                        assert!(last_was_arg);
                        last_was_arg = false;
                        operations.push(ExpOperation::NotEqual);
                    } 
                    operator::Operator::Concat => {
                        assert!(last_was_arg);
                        last_was_arg = false;
                        operations.push(ExpOperation::Concat);
                    },
                    operator::Operator::Plus => {
                        assert!(last_was_arg);
                        last_was_arg = false;
                        operations.push(ExpOperation::Plus);
                    },
                    operator::Operator::Minus => {
                        operations.push(
                            if last_was_arg {
                                ExpOperation::Minus
                            } else { ExpOperation::UnaryMinus }
                        );
                        last_was_arg = false;
                    },
                    operator::Operator::Star => {
                        assert!(last_was_arg);
                        last_was_arg = false;
                        operations.push(ExpOperation::Star);
                    },
                    operator::Operator::Slash => {
                        assert!(last_was_arg);
                        last_was_arg = false;
                        operations.push(ExpOperation::Slash);
                    },
                    operator::Operator::LogicalNot => {
                        assert!(!last_was_arg);
                        last_was_arg = false;
                        operations.push(ExpOperation::Not);
                    },
                    operator::Operator::Caret => {
                        assert!(last_was_arg);
                        last_was_arg = false;
                        operations.push(ExpOperation::Exp);
                    }
                    _ => { todo!() }
                }
            },
            Lexeme::AngleBrackets(bkt) => {
                lex.next();
                match bkt {
                    lexer::AngleBrackets::Open => {
                        assert!(last_was_arg);
                        last_was_arg = false;
                        operations.push(ExpOperation::GreaterThan);
                    },
                    lexer::AngleBrackets::Close => {
                        assert!(last_was_arg);
                        last_was_arg = false;
                        operations.push(ExpOperation::LessThan);
                    }
                }
            },
            Lexeme::Seperator(sep) => {
                match sep {
                    seperator::Seperator::OpenParen => {
                        lex.next();
                        opened_parens += 1;
                        assert!(!last_was_arg);
                        last_was_arg = false;
                        operations.push(ExpOperation::OpenParen);
                    },
                    seperator::Seperator::CloseParen => {
                        
                        opened_parens -= 1;
                        if opened_parens < 0 {
                            eprintln!("might have miscounted parens!");
                            break;
                        }
                        lex.next();
                        assert!(!last_was_arg);
                        last_was_arg = true;
                        operations.push(ExpOperation::CloseParen);
                    },
                    seperator::Seperator::OpenBracket | seperator::Seperator::CloseBracket => {
                        todo!()
                    }
                    _ => break
                }
            },
            Lexeme::Identifier(ident) => {
                if last_was_arg {
                    break;
                }
                lex.next();
                last_was_arg = true;
                if let Some(Lexeme::Seperator(seperator::Seperator::OpenParen)) = lex.clone().peekable().peek() {
                    // assert that its a correct function call
                    println!("resolving function call");
                    opened_parens += 1;
                    lex.next();
                    // parse expressions
                    while lex.clone().peekable().peek() != Some(&Lexeme::Seperator(seperator::Seperator::CloseParen)) {
                        let _ = parse_expression(lex);
                        if lex.clone().peekable().peek() == Some(&Lexeme::Seperator(seperator::Seperator::Comma)) {
                            lex.next();
                        }
                    }
                   
                    opened_parens -= 1;
                    lex.next();
                    operands.push(ExpOperand::FuncCall(ident.clone()));
                }
                else if let Some(Lexeme::Seperator(seperator::Seperator::Dot)) = lex.clone().peekable().peek()
                {
                    // terrible way to peek!!!
                    lex.next();
                    match lex.next() {
                        Some(Lexeme::Identifier(method)) => {
                            lex.next(); // should be oparen
                            opened_parens += 1;
                            // parse args
                            // THIS IS DUPLICATED CODE FROM EXPRESSION IDENT PARSING
                            while lex.clone().peekable().peek() != Some(&Lexeme::Seperator(seperator::Seperator::CloseParen)) {
                                let _ = parse_expression(lex);
                                if lex.clone().peekable().peek() == Some(&Lexeme::Seperator(seperator::Seperator::Comma)) {
                                    lex.next();
                                }
                            }
                            lex.next(); 
                            operands.push(ExpOperand::MethodCall(ident.clone(), method));
                            println!("parsed method as expression");   
                        }
                        _ => panic!("???")
                    }
                }
                else {
                    operands.push(ExpOperand::Identifier(ident.clone()));
                }
            },
            Lexeme::NumericLiteral(nlit) => {
                lex.next();
                assert!(!last_was_arg);
                last_was_arg = true;
                operands.push(ExpOperand::NumericLiteral(nlit.clone()));
            },
            Lexeme::StringLiteral(slit) => {
                lex.next();
                assert!(!last_was_arg);
                last_was_arg = true;
                operands.push(ExpOperand::StringLiteral(slit.clone()));
            },
            _ => break
        }

        while last_was_arg && operations.len() > 0 {
            if operations.len() > 1 {
                let current = operations.pop().unwrap();
                let previous = operations.pop().unwrap();
                if previous == ExpOperation::CloseParen {
                    // find matching open paren
                    // this code for wrapping a phrase is the same that is called at the end of parsing
                    // therefore, make that subroutine
                }
                if ExpOperation::precedence(previous, current) == Ordering::Greater {
                    // prev arg binds to last two operands
                    if previous != ExpOperation::OpenParen {
                        let rhs = Box::new(operands.pop().unwrap());
                        let lhs = Box::new(operands.pop().unwrap());
                        let new_operand = ExpOperand::BinaryExp(
                            BinaryExpression { op: previous, lhs, rhs }  
                        );
                        operands.push(new_operand);
                        operations.push(current);
                    }
                } else if ExpOperation::precedence(previous, current) == Ordering::Equal {
                    // we've already handled paren cases
                    // if op isnt exp, we can eval prev
                    if previous != ExpOperation::Exp {
                        // code duplication
                        // TODO: MERGE PATHS
                        let rhs = Box::new(operands.pop().unwrap());
                        let lhs = Box::new(operands.pop().unwrap());
                        let new_operand = ExpOperand::BinaryExp(
                            BinaryExpression { op: previous, lhs, rhs }  
                        );
                        operands.push(new_operand);
                        operations.push(current);
                    } else { break }
                }
                else { break }
            } else if *operations.last().unwrap() ==  ExpOperation::UnaryMinus {
                operations.pop();
                let arg = Box::new(operands.pop().unwrap());
                let new_arg = ExpOperand::UnaryExp(
                    UnaryExpression { op: ExpOperation::UnaryMinus, arg }
                );
                operands.push(new_arg);
            }
            else { break }
        }
    } 

    while let Some(op) = operations.pop() {
        if op == ExpOperation::UnaryMinus {
            // duplicate code
            let arg = Box::new(operands.pop().unwrap());
            let new_arg = ExpOperand::UnaryExp(
                UnaryExpression { op: ExpOperation::UnaryMinus, arg }
            );
            operands.push(new_arg);
        }
        else {
            // also code dup
            let rhs = Box::new(operands.pop().unwrap());
            let lhs = Box::new(operands.pop().unwrap());
            let new_operand = ExpOperand::BinaryExp(
                BinaryExpression { op: op, lhs, rhs }  
            );
            operands.push(new_operand);
        }
    }

    //println!("Operands: {:?}", operands);
    //println!("Operations: {:?}", operations);

    operands.pop().map(|exp| Box::new(exp) as Box<dyn Expression>)
}