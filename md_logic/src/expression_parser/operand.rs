use serde::Serialize;
use serde_json::{Number, Value};
use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub enum Operator {
    Plus,
    Substract,
    Multiply,
    Division,
    L,
    G,
    LE,
    GE,
    E,
    NE,
}

#[derive(PartialEq, Debug, Clone, Serialize)]
pub enum Operand {
    Primitive(Value),
    Variable(String),
    OperatorToken(Operator),
    OpenParen,
    CloseParen,
}

impl Add for Operand {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Operand::Primitive(p1), Operand::Primitive(p2)) => match (p1, p2) {
                (Value::Number(n1), Value::Number(n2)) => {
                    let res = n1.as_f64().unwrap_or(0.0) + n2.as_f64().unwrap_or(0.0);
                    Operand::Primitive(Value::Number(Number::from_f64(res).unwrap()))
                }
                (Value::String(s1), Value::String(s2)) => {
                    Operand::Primitive(Value::String(format!("{}{}", s1, s2)))
                }
                (_, _1) => Operand::Primitive(Value::Null),
            },
            (_, _1) => Operand::Primitive(Value::Null),
        }
    }
}

impl Sub for Operand {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Operand::Primitive(p1), Operand::Primitive(p2)) => match (p1, p2) {
                (Value::Number(n1), Value::Number(n2)) => {
                    let res = n1.as_f64().unwrap_or(0.0) - n2.as_f64().unwrap_or(0.0);
                    Operand::Primitive(Value::Number(Number::from_f64(res).unwrap()))
                }
                (_, _1) => Operand::Primitive(Value::Null),
            },
            (_, _1) => Operand::Primitive(Value::Null),
        }
    }
}

impl Mul for Operand {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Operand::Primitive(p1), Operand::Primitive(p2)) => match (p1, p2) {
                (Value::Number(n1), Value::Number(n2)) => {
                    let res = n1.as_f64().unwrap_or(0.0) * n2.as_f64().unwrap_or(0.0);
                    Operand::Primitive(Value::Number(Number::from_f64(res).unwrap()))
                }
                (_, _1) => Operand::Primitive(Value::Null),
            },
            (_, _1) => Operand::Primitive(Value::Null),
        }
    }
}

impl Div for Operand {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Operand::Primitive(p1), Operand::Primitive(p2)) => match (p1, p2) {
                (Value::Number(n1), Value::Number(n2)) => {
                    let n2_unwrapped = n2.as_f64().unwrap_or(0.0);
                    if n2_unwrapped == 0.0 {
                        return Operand::Primitive(Value::Null);
                    }
                    let res = n1.as_f64().unwrap_or(0.0) / n2_unwrapped;
                    Operand::Primitive(Value::Number(Number::from_f64(res).unwrap()))
                }
                (_, _1) => Operand::Primitive(Value::Null),
            },
            (_, _1) => Operand::Primitive(Value::Null),
        }
    }
}

impl std::cmp::Ord for Operand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Operand::Primitive(p1), Operand::Primitive(p2)) => match (p1, p2) {
                (Value::Number(n1_base), Value::Number(n2_base)) => {
                    let n1 = n1_base.as_f64().unwrap_or(0.0);
                    let n2 = n2_base.as_f64().unwrap_or(0.0);
                    // (Operand::Number(n1), Operand::Number(n2)) => {
                    if n1 > n2 {
                        return std::cmp::Ordering::Greater;
                    }
                    if n1 < n2 {
                        return std::cmp::Ordering::Less;
                    }

                    return std::cmp::Ordering::Equal;
                }
                (_, _1) => std::cmp::Ordering::Equal,
            },
            (_, _1) => std::cmp::Ordering::Equal,
        }
    }
}

impl PartialOrd for Operand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Operand {}
