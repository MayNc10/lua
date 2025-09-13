/*
    Full Lua Grammar
    I'm just going to write out (in semi-correct EBNF) my understanding of Lua's grammar
    This will be a useful reference to keep around

    // Eliding chunk, maybe put it back
    
    block ::= {<stat>} [<laststat>] 

    stat ::= ';' |
            kw_do <block> kw_end |
            <varlist> '=' <explist> |
            kw_while <exp> kw_do <block> kw_end |
            kw_repeat <block> kw_until <exp> |
            kw_if <exp> kw_then <block> {kw_elseif <exp> kw_then <block>} [kw_else <block>] kw_end |
            kw_goto <Name> |
            <label> |
            kw_for <Name> '=' <exp> ',' <exp> [',' <exp>] kw_do <block> kw_end |
            kw_for <namelist> kw_in <explist> kw_do <block> kw_end |
            <functioncall> |
            kw_local <attnamelist> ['=' <explist>] |
            kw_function <funcname> <funcbody> |
            kw_local kw_function <Name> <funcbody>

    exp ::= <prefixexp> |
            kw_nil | kw_false | kw_true |
            <Numeral> |
            <LiteralString> |
            <functiondef> | // ???
            <tableconstructor> |
            '...' |
            <exp> <binop> <exp> |
            <unop> <exp> |

    prefixexp ::= <var> | <functioncall> | '(' <exp> ')'

    laststat ::=  kw_break |
                  kw_return [<explist>] [';']

    varlist ::= <var> {',' <var>}

    var ::= <Name> |
            <prefixexp> '[' <exp> ']' |
            <prefixexp> '.' <Name>

    explist ::= <exp> {',' <exp>}

    namelist ::= <Name> {',' <Name>}

    attnamelist ::= <Name> <attrib> {',' <Name> <attrib>}

    attrib ::= ['<' <Name> '>']

    label ::= '::' <Name> '::'

    tableconstructor ::= '{' [<fieldlist>] '}'

    fieldlist ::= <field> {<fieldsep> <field>} [<fieldsep>]

    field ::= '[' <exp> ']' '=' <exp> | <Name> '=' <exp> | <exp>

    fieldsep ::= ',' | ';'

    functioncall ::= <prefixexp> <args> |
                     <prefixexp> ':' <Name> <args>

    args ::= '(' [<explist>] ')' |
             <tableconstructor> |
             <LiteralString>
    
    functiondef ::= kw_function <funcbody>

    funcbody ::= '(' [<parlist>] ')' <block> kw_end

    funcname ::= <Name> {'.' <Name>} [':' <Name>]

    parlist ::= <namelist> [',' '...'] | '...'    

    Name ::= <identifier> //TODO just write out rules for these

    Numeral ::= TODO
    LiteralString ::= TODO 

    binop ::= TODO
    unop ::= TODO
*/

use crate::{ast::*, lexer::Lexer};

pub fn parse(mut source: &str) -> Option<Block> {
    // do other things?
    if &source[..2] == "#!" {
        // get rid of of shebang
        let nextl = source.find('\n').expect("source file with shebang should be non-empty");
        source = &source[nextl + 1..]; 
    }
    let mut lex = Lexer::new(source);
    let block = Block::parse(&mut lex);
    while let Some(lexeme) = lex.next() {
        println!("remaining lexeme: {:?}", lexeme);
    }
    block
}

