use serde_json::{Number, Value};
use std::iter::Enumerate;
use std::iter::Peekable;
use std::str::Chars;

use super::operand::{Operand, Operator};

struct TokenRange {
    tracking: bool,
    started_at: usize,
    ended_at: usize,
}

impl TokenRange {
    pub fn new() -> Self {
        return TokenRange {
            tracking: false,
            started_at: 0,
            ended_at: 0,
        };
    }

    pub fn set_start(&mut self, i: usize) {
        if self.tracking == false {
            self.tracking = true;
            self.started_at = i;
        }
    }

    pub fn set_end(&mut self, i: usize) {
        self.ended_at = i;
    }
}

type Program = Vec<Operand>;

pub struct Tokenizer<'a> {
    expression: &'a str,
    i: Peekable<Enumerate<Chars<'a>>>,
    operands: Program,
}

impl<'a> Tokenizer<'a> {
    pub fn new(expression: &'a str) -> Self {
        return Tokenizer {
            expression,
            i: expression.chars().enumerate().peekable(),
            operands: Vec::with_capacity(expression.len()),
        };
    }

    pub fn parse(&mut self) -> Result<(), String> {
        loop {
            self.consume_spaces();

            if self.i.peek() == None {
                break;
            }
            let n = self.next_operand()?;
            self.operands.push(n);
        }

        Ok(())
    }

    fn consume_spaces(&mut self) {
        loop {
            match self.i.peek() {
                Some(&(_, '\t')) | Some(&(_, ' ')) | Some(&(_, '\n')) => {
                    self.i.next();
                }
                _ => break,
            }
        }
    }

    fn next_operand(&mut self) -> Result<Operand, String> {
        while let Some(&(_index, c)) = self.i.peek() {
            if c == '(' {
                self.i.next();
                return Ok(Operand::OpenParen);
            } else if c == ')' {
                self.i.next();
                return Ok(Operand::CloseParen);
            } else if c == '"' {
                return self.consume_string();
            } else if check_if_operand(&c) {
                return self.consume_variable();
            } else if check_if_operator(&c) {
                return self.consume_operator();
            } else if check_if_digit(&c) {
                return self.consume_number();
            } else {
                return Err(format!("unknown symbol at index {}, {:?}", _index, c));
            }
        }

        Err("Reached end - unprocessed statements found".to_string())
    }
    pub fn to_postfix(self) -> Result<Vec<Operand>, String> {
        let mut stack: Vec<Operand> = Vec::with_capacity(50);
        let mut postfix: Vec<Operand> = Vec::with_capacity(self.operands.len());

        for o in self.operands {
            match o {
                Operand::Primitive(_) | Operand::Variable(_) => {
                    postfix.push(o);
                }
                Operand::OpenParen => {
                    stack.push(o);
                }
                Operand::CloseParen => {
                    let mut found = false;
                    while let Some(s_item) = stack.pop() {
                        match s_item {
                            Operand::OpenParen => {
                                found = true;
                                break;
                            }
                            _ => {
                                postfix.push(s_item);
                            }
                        }
                    }

                    if found == false {
                        return Err("no matching opening paren".to_string());
                    }
                }
                Operand::OperatorToken(ref t) => {
                    if stack.len() == 0 {
                        stack.push(o);
                    } else {
                        loop {
                            if let Some(Operand::OpenParen) = stack.last() {
                                stack.push(o);
                                break;
                            } else if let Some(Operand::OperatorToken(so)) = stack.last() {
                                if precedence(so) >= precedence(&t) {
                                    if let Some(poped_stack_item) = stack.pop() {
                                        postfix.push(poped_stack_item);
                                    } else {
                                        return Err("stack underflow".to_string());
                                    }
                                } else {
                                    stack.push(o);
                                    break;
                                }
                            } else {
                                stack.push(o);
                                break;
                            }
                        }
                    }
                }
            }
        }

        while let Some(s_item) = stack.pop() {
            if s_item == Operand::OpenParen {
                return Err("no matching closing paren".to_string());
            }

            postfix.push(s_item);
        }

        return Ok(postfix);
    }

    fn consume_string(&mut self) -> Result<Operand, String> {
        self.i.next();
        let mut is_closed = false;
        let mut range = TokenRange::new();

        while let Some((_index, c)) = self.i.next() {
            if c == '"' {
                is_closed = true;
                range.set_end(_index);
                break;
            } else {
                range.set_start(_index);
            }
        }

        if is_closed {
            let str_literal = &self.expression[range.started_at..range.ended_at];

            return Ok(Operand::Primitive(Value::String(str_literal.to_string())));
        } else {
            return Err(format!("no closing \" at {}", range.started_at));
        }
    }

    fn consume_operator(&mut self) -> Result<Operand, String> {
        let mut range = TokenRange::new();

        while let Some(&(_index, c)) = self.i.peek() {
            if check_if_operator(&c) {
                range.set_start(_index);
                self.i.next();
            } else {
                range.set_end(_index);
                break;
            }

            range.set_end(_index + 1);
        }

        let operator = &self.expression[range.started_at..range.ended_at];

        let o = match operator {
            "+" => Some(Operator::Plus),
            "-" => Some(Operator::Substract),
            "*" => Some(Operator::Multiply),
            "/" => Some(Operator::Division),
            "=" => Some(Operator::E),
            "!=" => Some(Operator::NE),
            "<" => Some(Operator::L),
            "<=" => Some(Operator::LE),
            ">" => Some(Operator::G),
            ">=" => Some(Operator::GE),
            _ => None,
        };

        if let Some(token) = o {
            return Ok(Operand::OperatorToken(token));
        } else {
            return Err(format!(
                "unsupported operator \"{:?}\" at {} ",
                o, range.started_at
            ));
        }
    }

    fn consume_variable(&mut self) -> Result<Operand, String> {
        let mut range = TokenRange::new();

        while let Some(&(_index, c)) = self.i.peek() {
            if check_if_operand(&c) {
                range.set_start(_index);
                self.i.next();
            } else {
                break;
            }

            range.set_end(_index);
        }

        let variable = &self.expression[range.started_at..range.ended_at + 1];

        let reserved_bool_keywords = ["true", "false"];

        if reserved_bool_keywords.contains(&variable) {
            return Ok(Operand::Primitive(Value::Bool(variable == "true")));
        }

        return Ok(Operand::Variable(variable.to_string()));
    }

    fn consume_number(&mut self) -> Result<Operand, String> {
        let mut range = TokenRange::new();
        let mut has_dot = false;

        while let Some(&(_index, c)) = self.i.peek() {
            if range.tracking == false && check_if_digit(&c) {
                range.set_start(_index);
                self.i.next();
            } else if &c == &'.' {
                if has_dot == false {
                    has_dot = true;
                    self.i.next();
                } else {
                    break;
                    // return Err(format!("number: multiple '.' at {}", _index));
                }
            } else if check_if_digit(&c) {
                range.set_start(_index);
                self.i.next();
            } else {
                break;
            }

            range.set_end(_index);
        }

        let number = &self.expression[range.started_at..range.ended_at + 1];

        if number.len() == 0 {
            return Err(format!("empty number at {}", range.started_at));
        }

        let res_number = number.parse::<f64>().unwrap();
        let res_operand = Operand::Primitive(Value::Number(Number::from_f64(res_number).unwrap()));
        return Ok(res_operand);
    }

    pub fn insert_start(&mut self, o: Operand) {
        self.operands.insert(0, o);
    }

    pub fn starts_with_operand(&self) -> bool {
        if let Some(Operand::OperatorToken(_)) = self.operands.get(0) {
            return true;
        }

        false
    }
}

fn check_if_operand(c: &char) -> bool {
    (c >= &'a' && c <= &'z') || (c >= &'A' && c <= &'Z')
}

fn check_if_operator(c: &char) -> bool {
    ['+', '-', '/', '*', '<', '=', '>', '!'].contains(c)
}

fn check_if_digit(c: &char) -> bool {
    c >= &'0' && c <= &'9'
}

/*
            3
          ____
        __|__|___
        |2 level|      * /
    ____|_______|____
    |    1 Level    |  + -
 ______________________
|  0 other ... =, >, <  |

*/

fn precedence(c: &Operator) -> i32 {
    match c {
        Operator::Plus | Operator::Substract => 1,
        Operator::Division | Operator::Multiply => 2,
        _ => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_expression() -> Result<(), String> {
        let formula = "100.00<=
 ((aA+(b*c))-d*2 )";

        let mut parser = Tokenizer::new(&formula);
        let res = parser.parse();
        assert!(res.is_ok());
        assert!(parser.operands.len() == 17);

        let postfix = parser.to_postfix()?;
        assert!(postfix.len() == 11);
        assert_eq!(
            postfix,
            vec![
                Operand::Primitive(Value::Number(Number::from_f64(100.0).unwrap())),
                Operand::Variable("aA".to_string()),
                Operand::Variable("b".to_string()),
                Operand::Variable("c".to_string()),
                Operand::OperatorToken(Operator::Multiply),
                Operand::OperatorToken(Operator::Plus),
                Operand::Variable("d".to_string()),
                Operand::Primitive(Value::Number(Number::from_f64(2.0).unwrap())),
                Operand::OperatorToken(Operator::Multiply),
                Operand::OperatorToken(Operator::Substract),
                Operand::OperatorToken(Operator::LE),
            ]
        );
        Ok(())
    }
    #[test]
    fn parses_no_paren_expression() -> Result<(), String> {
        let formula = "aA+b *c-d*2";

        let mut parser = Tokenizer::new(&formula);
        let res = parser.parse();
        assert!(res.is_ok());
        assert!(parser.operands.len() == 9);

        let postfix = parser.to_postfix()?;
        assert!(postfix.len() == 9);
        assert_eq!(
            postfix,
            vec![
                Operand::Variable("aA".to_string()),
                Operand::Variable("b".to_string()),
                Operand::Variable("c".to_string()),
                Operand::OperatorToken(Operator::Multiply),
                Operand::OperatorToken(Operator::Plus),
                Operand::Variable("d".to_string()),
                Operand::Primitive(Value::Number(Number::from_f64(2.0).unwrap())),
                Operand::OperatorToken(Operator::Multiply),
                Operand::OperatorToken(Operator::Substract),
            ]
        );
        Ok(())
    }

    #[test]
    fn fails_expression_unknown_symbol() {
        let formula = "100.00<)^";
        let mut parser = Tokenizer::new(&formula);
        let res = parser.parse();
        assert!(!res.is_ok());
    }

    #[test]
    fn fails_expression_no_open_paren() -> Result<(), String> {
        let formula = "100.00)";
        let mut parser = Tokenizer::new(&formula);
        parser.parse()?;
        let postfix = parser.to_postfix();
        assert!(!postfix.is_ok());
        Ok(())
    }

    #[test]
    fn fails_expression_no_closing_paren() -> Result<(), String> {
        let formula = "100.00(";
        let mut parser = Tokenizer::new(&formula);
        parser.parse()?;

        let postfix = parser.to_postfix();
        assert!(!postfix.is_ok());
        Ok(())
    }

    #[test]
    fn succeeds_single_string_literal_element() -> Result<(), String> {
        let formula = "\"hello\"";
        let mut parser = Tokenizer::new(&formula);
        parser.parse()?;

        let postfix = parser.to_postfix();
        assert!(postfix.is_ok());

        assert_eq!(
            postfix?,
            vec![Operand::Primitive(Value::String("hello".to_string()))]
        );
        Ok(())
    }

    #[test]
    fn fail_string_missing_quote() -> Result<(), String> {
        let formula = "\"miss you";
        let mut parser = Tokenizer::new(&formula);
        let failed_string_parse = parser.parse();

        assert!(!failed_string_parse.is_ok());
        Ok(())
    }

    #[test]
    fn fail_number_with_many_dots() -> Result<(), String> {
        let formula = "100.00.0";
        let mut parser = Tokenizer::new(&formula);
        let failed_number = parser.parse();

        assert!(!failed_number.is_ok());
        Ok(())
    }

    #[test]
    fn succeeds_single_number_element() -> Result<(), String> {
        let formula = "101.001";
        let mut parser = Tokenizer::new(&formula);
        parser.parse()?;

        let postfix = parser.to_postfix();
        assert!(postfix.is_ok());

        assert_eq!(
            postfix?,
            vec![Operand::Primitive(Value::Number(
                Number::from_f64(101.001).unwrap()
            )),]
        );
        Ok(())
    }

    #[test]
    fn succeeds_single_variable() -> Result<(), String> {
        let formula = "expectedVariable";
        let mut parser = Tokenizer::new(&formula);
        parser.parse()?;

        let postfix = parser.to_postfix();
        assert!(postfix.is_ok());

        assert_eq!(
            postfix?,
            vec![Operand::Variable("expectedVariable".to_string())]
        );
        Ok(())
    }

    #[test]
    fn succeeds_single_boolean() -> Result<(), String> {
        let formula = "true";
        let mut parser = Tokenizer::new(&formula);
        parser.parse()?;

        let postfix = parser.to_postfix();
        assert!(postfix.is_ok());

        assert_eq!(postfix?, vec![Operand::Primitive(Value::Bool(true))]);
        Ok(())
    }

    #[test]
    fn succeeds_inserting_to_biginning() -> Result<(), String> {
        let formula = "<10";
        let mut parser = Tokenizer::new(&formula);
        parser.parse()?;

        let start_with_operand = parser.starts_with_operand();
        assert_eq!(start_with_operand, true);

        parser.insert_start(Operand::Primitive(Value::Number(
            Number::from_f64(11.0).unwrap(),
        )));

        let postfix = parser.to_postfix();
        assert!(postfix.is_ok());

        assert_eq!(
            postfix?,
            vec![
                Operand::Primitive(Value::Number(Number::from_f64(11.0).unwrap())),
                Operand::Primitive(Value::Number(Number::from_f64(10.0).unwrap())),
                Operand::OperatorToken(Operator::L)
            ]
        );
        Ok(())
    }

    #[test]
    fn succeeds_inserting_implicit_operator() -> Result<(), String> {
        let formula = "10";
        let mut parser = Tokenizer::new(&formula);
        parser.parse()?;

        let start_with_operand = parser.starts_with_operand();
        assert_eq!(start_with_operand, false);

        parser.insert_start(Operand::OperatorToken(Operator::E));
        parser.insert_start(
            // Operand::Number(11.0)
            Operand::Primitive(Value::Number(Number::from_f64(11.0).unwrap())),
        );

        let postfix = parser.to_postfix();
        assert!(postfix.is_ok());

        assert_eq!(
            postfix?,
            vec![
                Operand::Primitive(Value::Number(Number::from_f64(11.0).unwrap())),
                Operand::Primitive(Value::Number(Number::from_f64(10.0).unwrap())),
                Operand::OperatorToken(Operator::E)
            ]
        );
        Ok(())
    }

    #[test]
    fn reserved_bool_operand() -> Result<(), String> {
        let formula = "true + false";
        let mut parser = Tokenizer::new(&formula);
        parser.parse()?;
        let postfix = parser.to_postfix();
        assert_eq!(
            postfix?,
            vec![
                Operand::Primitive(Value::Bool(true)),
                Operand::Primitive(Value::Bool(false)),
                Operand::OperatorToken(Operator::Plus)
            ]
        );
        Ok(())
    }

    #[test]
    fn succeeds_operator_check() -> Result<(), String> {
        let formula = "11+10";
        let mut parser = Tokenizer::new(&formula);
        parser.parse()?;

        let postfix = parser.to_postfix();
        assert!(postfix.is_ok());

        assert_eq!(is_postfix_valid(&postfix?), true);
        Ok(())
    }

    #[test]
    fn fails_operator_check_on_right() -> Result<(), String> {
        let formula = "11+";
        let mut parser = Tokenizer::new(&formula);
        parser.parse()?;
        let postfix = parser.to_postfix();
        assert_eq!(is_postfix_valid(&postfix?), false);
        Ok(())
    }

    #[test]
    fn fails_operator_check_on_left() -> Result<(), String> {
        let formula = "+11";
        let mut parser = Tokenizer::new(&formula);
        parser.parse()?;
        let postfix = parser.to_postfix();
        assert_eq!(is_postfix_valid(&postfix?), false);
        Ok(())
    }
}

fn is_postfix_valid(postfix: &Vec<Operand>) -> bool {
    let mut stack: Vec<&Operand> = Vec::with_capacity(postfix.len());
    let mut valid = false;

    for p in postfix {
        valid = false;

        if let Operand::OperatorToken(_o) = p {
            let right = stack.pop().is_some();
            let left = stack.pop().is_some();

            if right == left {
                stack.push(&Operand::Primitive(Value::Null));
                valid = true;
            }
        } else {
            stack.push(p);
        }
    }

    valid && stack.len() == 1
}
