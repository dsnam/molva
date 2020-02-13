use crate::lexer::{Token, Token::*, TokenInfo};

#[derive(Debug)]
pub enum Expr {
    Invoke {
        fn_name: String,
        args: Vec<Expr>,
    },

    Boolean {
        operator: String,
        left: Box<Expr>,
        right: Box<Expr>,
    },

    Conditional {
        condition: Box<Expr>,
        consequent: Box<Expr>,
        alternative: Box<Expr>,
    },

    Float(f64),
    StringLiteral(String),
    Int(i32),
    BoolTrue,
    BoolFalse,
    Identifier(String),
}

pub enum Statement {
    VarDec {
        mutable: bool,
        name: String,
        type_name: Option<String>,
    },

    VarAssign {
        name: String,
        value: Box<Expr>,
    },

    For {
        var: String,
        start: Box<Expr>,
        end: Box<Expr>,
        step: Option<Box<Expr>>,
        body: Box<Expr>,
    },

    While {
        condition: Box<Expr>,
        body: Box<Expr>,
    },
}

#[derive(Debug)]
pub struct TypedIdentifier {
    name: String,
    type_name: String,
}

#[derive(Debug)]
pub struct Prototype {
    pub name: String,
    pub args: Vec<TypedIdentifier>,
    pub return_type: Option<String>,
    pub is_operator: bool,
}

#[derive(Debug)]
pub struct Function {
    pub prototype: Prototype,
    pub body: Expr,
}

#[derive(Debug)]
pub struct ParserError {
    pub error: String,
    pub line: usize,
    pub col: usize,
}

impl ParserError {
    pub fn new(error_msg: String, line: usize, col: usize) -> Self {
        Self {
            error: error_msg,
            line,
            col,
        }
    }
}

pub struct Parser {
    tokens: Vec<TokenInfo>,
    pos: usize,
}

impl Parser {
    pub fn new(input: Vec<TokenInfo>) -> Self {
        Self {
            tokens: input,
            pos: 0,
        }
    }

    fn current(&self) -> Result<TokenInfo, ParserError> {
        if self.pos >= self.tokens.len() {
            Err(ParserError::new(String::from("Unexpected EOF"), 0, 0))
        } else {
            Ok(self.tokens[self.pos].clone())
        }
    }

    fn curr(&self) -> Token {
        self.tokens[self.pos].clone().token
    }

    fn at_end(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn advance(&mut self) -> Result<(), ParserError> {
        let next_pos = self.pos + 1;
        if next_pos < self.tokens.len() {
            self.pos = next_pos;
            Ok(())
        } else {
            Err(ParserError::new(String::from("Unexpected EOF"), 0, 0))
        }
    }

    fn next(&self) -> Token {
        if self.at_end() {
            return EOF;
        }
        self.tokens[self.pos + 1].clone().token
    }

    fn wrap_error(error_msg: String, line: usize, col: usize) -> ParserError {
        ParserError::new(error_msg, line, col)
    }

    pub fn parse(&mut self) -> Result<Function, ParserError> {
        let result = match self.current()?.token {
            Fn => self.parse_function(),
            Op => self.parse_operator_def(),
            _ => Err(ParserError::new(
                String::from("only function and operator definitions are allowed at the top level"),
                self.current()?.line,
                self.current()?.col,
            )),
        };
        match result {
            Ok(result) => {
                if !self.at_end() {
                    Err(Self::wrap_error(String::from("borked"), 0, 0))
                } else {
                    Ok(result)
                }
            }
            err => err,
        }
    }

    // just a short wrapper for when we want to consume a token but do nothing with it other than
    // verify it's there. mostly for structural elements like brackets
    fn expect(&mut self, token: Token) -> Result<(), ParserError> {
        match self.curr() {
            token => {
                self.advance();
                Ok(())
            }
            _ => Err(Self::wrap_error(
                format!(
                    "Expected {:?} at line {} col {}",
                    self.current()?.token,
                    self.current()?.line,
                    self.current()?.col
                ),
                self.current()?.line,
                self.current()?.col,
            )),
        }
    }

    fn parse_function(&mut self) -> Result<Function, ParserError> {
        let prototype = self.parse_prototype()?;

        self.expect(LeftCurlBracket);

        let body = self.parse_expr()?;

        self.expect(RightCurlBracket);

        Ok(Function { prototype, body })
    }

    fn parse_prototype(&mut self) -> Result<Prototype, ParserError> {
        self.expect(Fn);

        let fn_name = self.parse_identifier()?;

        self.expect(LeftParen);

        let mut args = Vec::new();
        if self.next() != RightParen {
            loop {
                args.push(self.parse_typed_identifier()?);
                match self.curr() {
                    RightParen => {
                        self.advance();
                        break;
                    }
                    Comma => {
                        self.advance();
                    }
                    _ => {
                        return Err(Self::wrap_error(
                            format!(
                                "Expected ',' or ')' in function definition at line {} col {}",
                                self.current()?.line,
                                self.current()?.col
                            ),
                            self.current()?.line,
                            self.current()?.col,
                        ))
                    }
                }
            }
        }
        let return_type: Option<String>;
        if self.curr() == Minus {
            println!("hi");
            self.expect(Minus);
            self.expect(RightAngBracket);
            return_type = Option::Some(self.parse_identifier()?);
        } else {
            return_type = Option::None;
        }

        Ok(Prototype {
            name: fn_name,
            args,
            return_type,
            is_operator: false,
        })
    }

    fn parse_operator_def(&mut self) -> Result<Function, ParserError> {
        unimplemented!();
    }

    fn parse_expr(&mut self) -> Result<Expr, ParserError> {
        unimplemented!();
    }

    fn parse_identifier(&mut self) -> Result<String, ParserError> {
        match self.current()?.token {
            Identifier(s) => {
                self.advance();
                Ok(s)
            }
            _ => Err(Self::wrap_error(
                String::from("Expected identifier"),
                self.current()?.line,
                self.current()?.col,
            )),
        }
    }

    fn parse_typed_identifier(&mut self) -> Result<TypedIdentifier, ParserError> {
        let name = self.parse_identifier()?;
        self.expect(Colon);
        let type_name = self.parse_identifier()?;
        Ok(TypedIdentifier { name, type_name })
    }

    fn parse_literal(&mut self) -> Result<Expr, ParserError> {
        match self.current()?.token {
            Int(i) => {
                self.advance();
                Ok(Expr::Int(i))
            }
            Float(f) => {
                self.advance();
                Ok(Expr::Float(f))
            }
            StringLiteral(s) => {
                self.advance();
                Ok(Expr::StringLiteral(s))
            }
            BoolTrue => {
                self.advance();
                Ok(Expr::BoolTrue)
            }
            BoolFalse => {
                self.advance();
                Ok(Expr::BoolFalse)
            }
            _ => Err(Self::wrap_error(
                String::from("Expected number literal"),
                self.current()?.line,
                self.current()?.col,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::*;

    fn wrap_tok(token: Token) -> TokenInfo {
        TokenInfo {
            token,
            line: 0,
            col: 0,
        }
    }

    #[test]
    fn test_parse_fn_proto() {
        let tokens = vec![
            Fn,
            Identifier(String::from("test")),
            LeftParen,
            Identifier(String::from("param1")),
            Colon,
            Identifier(String::from("Type1")),
            Comma,
            Identifier(String::from("param2")),
            Colon,
            Identifier(String::from("Type2")),
            RightParen,
            Minus,
            RightAngBracket,
            Identifier(String::from("OutputType")),
        ]
        .iter()
        .map(|t| wrap_tok(t.clone()))
        .collect::<Vec<_>>();
        let mut parser = Parser::new(tokens);
        let proto = parser.parse_prototype().unwrap();
        assert_eq!(proto.name, "test");
        assert_eq!(proto.args.len(), 2);
        assert_eq!(proto.args.get(0).unwrap().name, "param1");
        assert_eq!(proto.args.get(0).unwrap().type_name, "Type1");
        assert_eq!(proto.return_type.unwrap(), "OutputType");
    }
}