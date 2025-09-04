use std::{cmp::Ordering, collections::HashMap, fmt::{Debug, Display}, io};

use crate::{ast::{context::Ctx, function::{FunctionCall, MethodCall}}, lexer::{self, Lexeme, Lexer, identifier::{self, Identifier}, literal, operator, seperator}, value::Value};

pub struct TableExpression {}

#[derive(Clone, Debug)]
pub struct BinaryExpression {
    op: ExpOperation,
    lhs: Box<Expression>,
    rhs: Box<Expression>
}

#[derive(Clone, Debug)]
pub struct UnaryExpression {
    op: ExpOperation,
    arg: Box<Expression>
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

    pub fn is_arith_op(&self) -> bool {
        matches!(self, ExpOperation::Plus | ExpOperation::Minus | ExpOperation::Star | ExpOperation::Slash 
            | ExpOperation::Exp | ExpOperation::UnaryMinus)
    }
}

#[derive(Clone, Debug)]
pub enum Expression {
    NumericLiteral(literal::NumericLiteral),
    StringLiteral(literal::StringLiteral),
    Identifier(identifier::Identifier),
    FuncCall(FunctionCall),
    MethodCall(MethodCall),
    BinaryExp(BinaryExpression),
    UnaryExp(UnaryExpression)
    // TODO: TABLES!
}

impl Expression {
    pub fn eval(&self, ctx: &mut Ctx) -> Value {
        match self {
            Expression::NumericLiteral(nlit) => {
                // FIXME
                Value::Number(nlit.value()) 
            },
            Expression::StringLiteral(slit) => {
                Value::String(slit.value().to_string())
            },
            Expression::Identifier(ident) => {
                ctx.get_var(ident).unwrap_or(Value::Nil)
            },
            Expression::FuncCall(fcall) => {
                fcall.call(ctx)
            },
            Expression::MethodCall(mcall) => {
                mcall.call(ctx)
            },
            Expression::BinaryExp(b) => {
                if b.op.is_arith_op() {
                    let lhs_val = b.lhs.eval(ctx).as_number().expect("BinOp lhs wasnt number");
                    let rhs_val = b.rhs.eval(ctx).as_number().expect("BinOp rhs wasnt number");
                    Value::Number(match b.op {
                        ExpOperation::Plus =>lhs_val + rhs_val, 
                        ExpOperation::Minus => lhs_val - rhs_val,
                        ExpOperation::Star => lhs_val * rhs_val,
                        ExpOperation::Slash => lhs_val / rhs_val,
                        ExpOperation::Exp => lhs_val.powf(rhs_val),
                        _ => unreachable!()
                    })
                } else {
                    match b.op {
                        ExpOperation::Equals => {
                            let lhs_val = b.lhs.eval(ctx);
                            let rhs_val = b.rhs.eval(ctx);
                            Value::Boolean((lhs_val == rhs_val).into())
                        },
                        ExpOperation::NotEqual => {
                            let lhs_val = b.lhs.eval(ctx);
                            let rhs_val = b.rhs.eval(ctx);
                            Value::Boolean((lhs_val != rhs_val).into())
                        },
                        ExpOperation::LessThan => {
                            let lhs_val = b.lhs.eval(ctx).as_number().expect("BinOp lhs wasnt number");
                            let rhs_val = b.rhs.eval(ctx).as_number().expect("BinOp rhs wasnt number");
                            Value::Boolean((lhs_val < rhs_val).into())
                        },
                        ExpOperation::GreaterThan => {
                            let lhs_val = b.lhs.eval(ctx).as_number().expect("BinOp lhs wasnt number");
                            let rhs_val = b.rhs.eval(ctx).as_number().expect("BinOp rhs wasnt number");
                            Value::Boolean((lhs_val > rhs_val).into())
                        },
                        ExpOperation::LessEqual => {
                            let lhs_val = b.lhs.eval(ctx).as_number().expect("BinOp lhs wasnt number");
                            let rhs_val = b.rhs.eval(ctx).as_number().expect("BinOp rhs wasnt number");
                            Value::Boolean((lhs_val <= rhs_val).into())
                        },
                        ExpOperation::GreaterEqual => {
                            let lhs_val = b.lhs.eval(ctx).as_number().expect("BinOp lhs wasnt number");
                            let rhs_val = b.rhs.eval(ctx).as_number().expect("BinOp rhs wasnt number");
                            Value::Boolean((lhs_val >= rhs_val).into())
                        },
                        // short circuits - should probably have a different branch
                        ExpOperation::Or => {
                            let lhs_val = b.lhs.eval(ctx).as_bool();
                            Value::Boolean((lhs_val || b.rhs.eval(ctx).as_bool()).into())
                        },
                        ExpOperation::And => {
                            let lhs_val = b.lhs.eval(ctx).as_bool();
                            Value::Boolean((lhs_val && b.rhs.eval(ctx).as_bool()).into())
                        }
                        _ => panic!("Binop {:?} not yet implemented!", b.op)
                    }
                }
            }     
            _ => panic!("Expression kind {:?} not yet implemented!", self)
        }
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn parse_expression(lex: &mut Lexer) -> Option<Expression> {
    // UGHHH I HATE SHUnTING YARD
    let mut operands = Vec::new();
    let mut operations = Vec::new();

    let mut last_was_arg = false;
    let mut opened_parens = 0;

    while let Some(tok) = lex.clone().peekable().peek() {
        //eprintln!("tok: {:?}\n op stack: {:?}\n arg_stack: {:?}\n\n", tok, operations, operands);
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
                        operations.push(ExpOperation::LessThan);
                    },
                    lexer::AngleBrackets::Close => {
                        assert!(last_was_arg);
                        last_was_arg = false;
                        operations.push(ExpOperation::GreaterThan);
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
                            break;
                        }
                        lex.next();
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
                let mut dup_lex = lex.clone();
                last_was_arg = true;
                if let Some(funccall) = FunctionCall::parse(&mut dup_lex) {
                    operands.push(Expression::FuncCall(funccall));
                    *lex = dup_lex;
                }
                else if let Some(mcall) = MethodCall::parse({ dup_lex = lex.clone(); &mut dup_lex }) {
                    operands.push(Expression::MethodCall(mcall));
                    *lex = dup_lex;
                }
                else {
                    lex.next();
                    operands.push(Expression::Identifier(ident.clone()));
                }        
            },
            Lexeme::NumericLiteral(nlit) => {
                lex.next();
                assert!(!last_was_arg);
                last_was_arg = true;
                operands.push(Expression::NumericLiteral(nlit.clone()));
            },
            Lexeme::StringLiteral(slit) => {
                lex.next();
                assert!(!last_was_arg);
                last_was_arg = true;
                operands.push(Expression::StringLiteral(slit.clone()));
            },
            _ => break
        }

        //eprintln!("matched tok {:?}", tok);
        while !last_was_arg && operations.len() > 0 {
            //eprintln!("in shunting yard processing");
            //eprintln!("top of op stack is {:?}", operations.last());
            if operations.len() > 1 {
                let current = operations.pop().unwrap();
                let previous = operations.pop().unwrap();
                if previous == ExpOperation::CloseParen {
                    // find matching open paren
                    // this code for wrapping a phrase is the same that is called at the end of parsing
                    // therefore, make that subroutine
                    //eprintln!("parsing close paren, current token is {:?}, stack is {:?}", current, operations);
                    
                    let mut start_idx = operations.len() - 1;
                    while operations[start_idx] != ExpOperation::OpenParen {
                        start_idx -= 1;
                    }
                    shunting_yard(&operations[start_idx + 1..], &mut operands);
                    operations.truncate(start_idx);
                    operations.push(current);
                }
                else if ExpOperation::precedence(previous, current) == Ordering::Greater {
                    // prev arg binds to last two operands
                    if previous != ExpOperation::OpenParen {
                        let rhs = Box::new(operands.pop().unwrap());
                        let lhs = Box::new(operands.pop().unwrap());
                        let new_operand = Expression::BinaryExp(
                            BinaryExpression { op: previous, lhs, rhs }  
                        );
                        operands.push(new_operand);
                        operations.push(current);
                    } else {
                        operations.push(previous); 
                        operations.push(current);
                        break;
                    }
                } else if ExpOperation::precedence(previous, current) == Ordering::Equal {
                    // we've already handled paren cases
                    // if op isnt exp, we can eval prev
                    if previous != ExpOperation::Exp && previous != ExpOperation::Concat {
                        // code duplication
                        // TODO: MERGE PATHS
                        let rhs = Box::new(operands.pop().unwrap());
                        let lhs = Box::new(operands.pop().unwrap());
                        let new_operand = Expression::BinaryExp(
                            BinaryExpression { op: previous, lhs, rhs }  
                        );
                        operands.push(new_operand);
                        operations.push(current);
                    } else { todo!() }
                }
                else { operations.push(previous); operations.push(current); break; }
            } else if *operations.last().unwrap() ==  ExpOperation::UnaryMinus {
                /* 
                operations.pop();
                let arg = Box::new(operands.pop().unwrap());
                let new_arg = Expression::UnaryExp(
                    UnaryExpression { op: ExpOperation::UnaryMinus, arg }
                );
                operands.push(new_arg);
                */
                todo!();
            }
            else { //eprintln!("breaking"); 
                break 
            }
        }
    } 

    while let Some(op) = operations.pop() {
        if op == ExpOperation::UnaryMinus {
            // duplicate code
            let arg = Box::new(operands.pop().unwrap());
            let new_arg = Expression::UnaryExp(
                UnaryExpression { op: ExpOperation::UnaryMinus, arg }
            );
            operands.push(new_arg);
        }
        else {
            // also code dup
            let rhs = Box::new(operands.pop().unwrap());
            let lhs = Box::new(operands.pop().unwrap());
            let new_operand = Expression::BinaryExp(
                BinaryExpression { op: op, lhs, rhs }  
            );
            operands.push(new_operand);
        }
    }

    //println!("Operands: {:?}", operands);
    //println!("Operations: {:?}", operations);

    operands.pop()
}

fn shunting_yard(ops: &[ExpOperation], args: &mut Vec<Expression>) {
    // invariant held by the algorithm is that operations are always sorted lowest associativity to highest
    // when we get an op and see that the top of the stack has higher precedence we pack that op into an expression
    // therefore, to finish the rest of these args, just go one by one top to bottom
    for op in ops.iter().rev() {
        if *op != ExpOperation::UnaryMinus {
            if args.len() < 2 { break; }
            let lhs = Box::new(args.pop().unwrap());
            let rhs = Box::new(args.pop().unwrap());
            let e = BinaryExpression { op: *op, lhs, rhs };
            args.push(Expression::BinaryExp(e));
        } else { todo!() }
    }
}

