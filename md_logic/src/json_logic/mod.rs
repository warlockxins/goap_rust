use serde::{Deserialize, Serialize};
use serde_json::{Number, Result, Value};

use crate::context::get_context_var;

type OrderingOperation = Vec<AllCombined>;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Opss {
    #[serde(alias = ">")]
    More(OrderingOperation),
    #[serde(alias = "<")]
    Less(OrderingOperation),
    #[serde(alias = "=")]
    Eq(OrderingOperation),
    #[serde(alias = "<=")]
    LessEq(OrderingOperation),
    #[serde(alias = ">=")]
    MoreEq(OrderingOperation),
    #[serde(alias = "!=")]
    Neq(OrderingOperation),
    #[serde(alias = "+")]
    Plus(OrderingOperation),
    #[serde(alias = "-")]
    Minus(OrderingOperation),
    #[serde(alias = "*")]
    Multiply(OrderingOperation),
    #[serde(alias = "/")]
    Division(OrderingOperation),
    #[serde(alias = "and")]
    And(OrderingOperation),
    #[serde(alias = "var")]
    Var(String),
}

fn binary_op_vars(l: &Vec<AllCombined>, context: &Value) -> Option<Vec<AllCombined>> {
    if l.len() < 2 {
        return None;
    }
    Some(execute_combined_list(&l, context))
}

impl Opss {
    fn execute(&self, context: &Value) -> AllCombined {
        match self {
            Opss::Less(l) => {
                if let Some(built_list) = binary_op_vars(l, context) {
                    return AllCombined::Primitive(Value::Bool(&built_list[0] < &built_list[1]));
                }
                AllCombined::Primitive(Value::Bool(false))
            }
            Opss::More(l) => {
                if let Some(built_list) = binary_op_vars(l, context) {
                    return AllCombined::Primitive(Value::Bool(&built_list[0] > &built_list[1]));
                }
                AllCombined::Primitive(Value::Bool(false))
            }
            Opss::Eq(l) => {
                if let Some(built_list) = binary_op_vars(l, context) {
                    return AllCombined::Primitive(Value::Bool(&built_list[0] == &built_list[1]));
                }
                AllCombined::Primitive(Value::Bool(false))
            }
            Opss::LessEq(l) => {
                if let Some(built_list) = binary_op_vars(l, context) {
                    return AllCombined::Primitive(Value::Bool(&built_list[0] <= &built_list[1]));
                }
                AllCombined::Primitive(Value::Bool(false))
            }
            Opss::MoreEq(l) => {
                if let Some(built_list) = binary_op_vars(l, context) {
                    return AllCombined::Primitive(Value::Bool(&built_list[0] >= &built_list[1]));
                }
                AllCombined::Primitive(Value::Bool(false))
            }
            Opss::Neq(l) => {
                if let Some(built_list) = binary_op_vars(l, context) {
                    return AllCombined::Primitive(Value::Bool(&built_list[0] != &built_list[1]));
                }
                AllCombined::Primitive(Value::Bool(false))
            }
            Opss::Plus(l) => {
                if let Some(built_list) = binary_op_vars(l, context) {
                    match (&built_list[0], &built_list[1]) {
                        (
                            AllCombined::Primitive(Value::Number(n1)),
                            AllCombined::Primitive(Value::Number(n2)),
                        ) => {
                            let res = n1.as_f64().unwrap_or(0.0) + n2.as_f64().unwrap_or(0.0);

                            return AllCombined::Primitive(Value::Number(
                                Number::from_f64(res).unwrap(),
                            ));
                        }
                        (
                            AllCombined::Primitive(Value::String(s1)),
                            AllCombined::Primitive(Value::String(s2)),
                        ) => return AllCombined::Primitive(Value::String(format!("{}{}", s1, s2))),
                        (_, _1) => return AllCombined::Primitive(Value::Null),
                    }
                }
                AllCombined::Primitive(Value::Null)
            }
            Opss::Minus(l) => {
                if let Some(built_list) = binary_op_vars(l, context) {
                    match (&built_list[0], &built_list[1]) {
                        (
                            AllCombined::Primitive(Value::Number(n1)),
                            AllCombined::Primitive(Value::Number(n2)),
                        ) => {
                            let res = n1.as_f64().unwrap_or(0.0) - n2.as_f64().unwrap_or(0.0);

                            return AllCombined::Primitive(Value::Number(
                                Number::from_f64(res).unwrap(),
                            ));
                        }
                        (_, _1) => return AllCombined::Primitive(Value::Null),
                    }
                }
                AllCombined::Primitive(Value::Null)
            }
            Opss::Multiply(l) => {
                if let Some(built_list) = binary_op_vars(l, context) {
                    match (&built_list[0], &built_list[1]) {
                        (
                            AllCombined::Primitive(Value::Number(n1)),
                            AllCombined::Primitive(Value::Number(n2)),
                        ) => {
                            let res = n1.as_f64().unwrap_or(0.0) * n2.as_f64().unwrap_or(0.0);

                            return AllCombined::Primitive(Value::Number(
                                Number::from_f64(res).unwrap(),
                            ));
                        }
                        (_, _1) => return AllCombined::Primitive(Value::Null),
                    }
                }
                AllCombined::Primitive(Value::Null)
            }
            Opss::Division(l) => {
                if let Some(built_list) = binary_op_vars(l, context) {
                    match (&built_list[0], &built_list[1]) {
                        (
                            AllCombined::Primitive(Value::Number(n1)),
                            AllCombined::Primitive(Value::Number(n2)),
                        ) => {
                            let second = n2.as_f64().unwrap_or(0.0);
                            if second == 0.0 {
                                return AllCombined::Primitive(Value::Null);
                            }

                            let res = n1.as_f64().unwrap_or(0.0) / second;

                            return AllCombined::Primitive(Value::Number(
                                Number::from_f64(res).unwrap(),
                            ));
                        }
                        (_, _1) => return AllCombined::Primitive(Value::Null),
                    }
                }
                AllCombined::Primitive(Value::Null)
            }
            Opss::Var(key) => AllCombined::Primitive(get_context_var(key, &context)),
            Opss::And(l) => {
                let l_results = execute_combined_list(&l, context);

                if l_results.len() == 0 {
                    return AllCombined::Primitive(Value::Bool(false));
                }

                let all_true = l_results.iter().all(|x| match x {
                    AllCombined::Primitive(Value::Bool(true)) => true,
                    _ => false,
                });

                return AllCombined::Primitive(Value::Bool(all_true));
            }
        }
    }
}

fn execute_combined_list(l: &Vec<AllCombined>, context: &Value) -> Vec<AllCombined> {
    l.iter().map(|l_item| l_item.execute(context)).collect()
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum AllCombined {
    Ops(Opss),
    OpList(Vec<AllCombined>),
    Primitive(Value),
}

impl AllCombined {
    pub fn execute(&self, context: &Value) -> AllCombined {
        match self {
            AllCombined::OpList(l) => {
                let s: Vec<AllCombined> = execute_combined_list(&l, context);
                AllCombined::OpList(s)
            }
            AllCombined::Ops(o) => o.execute(context),
            AllCombined::Primitive(v) => AllCombined::Primitive(v.clone()),
        }
    }
}

// Ordering operations
impl std::cmp::Ord for AllCombined {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (
                AllCombined::Primitive(Value::Number(n1)),
                AllCombined::Primitive(Value::Number(n2)),
            ) => {
                // Todo, compare as i64 too
                let n1_num = n1.as_f64().unwrap_or(0.0);
                let n2_num = n2.as_f64().unwrap_or(0.0);
                if n1_num > n2_num {
                    return std::cmp::Ordering::Greater;
                }
                if n1_num < n2_num {
                    return std::cmp::Ordering::Less;
                }

                return std::cmp::Ordering::Equal;
            }
            (_, _1) => std::cmp::Ordering::Equal,
        }
    }
}

impl PartialOrd for AllCombined {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for AllCombined {}
// end Ordering operations

mod tests {
    use super::*;

    #[test]
    fn serializes_more_operator_enum_representation() -> Result<()> {
        let cases = vec![
            (r#" { ">" : [3,10] }"#, false),
            (r#" { ">" : [10,3] }"#, true),
            (r#" { "<" : [3,10] }"#, true),
            (r#" { "<" : [30,10] }"#, false),
            (r#" { "=" : [10.0,10.0] }"#, true),
            (r#" { "<=" : [10.0,10.0] }"#, true),
            (r#" { "<=" : [12.0,10.0] }"#, false),
            (r#" { ">=" : [10.0,10.0] }"#, true),
            (r#" { ">=" : [9.0,10.0] }"#, false),
            (r#" { "!=" : [9.0,10.0] }"#, true),
            (r#" { "!=" : [10.0,10.0] }"#, false),
            (r#" { "!=" : [true,false] }"#, true),
            (r#" { "=" : [true,true] }"#, true),
            (r#" { "=" : [{ "<" : [3,10] },{ ">": [1, 0] }] }"#, true),
            (r#" { "=" : ["hi","hi"] }"#, true),
            (r#" { "=" : ["hi","hi2"] }"#, false),
            (r#" { "=" : ["hi", 2] }"#, false),
            (r#" { ">" : ["hi", 2] }"#, false),
            (r#" { "=" : [{"+": [1,1]}, 2.0] }"#, true),
            (r#" { "=" : [{"-": [2,1]}, 1.0] }"#, true),
            (r#" { "=" : [{"*": [2,3]}, 6.0] }"#, true),
            (r#" { "=" : [{"/": [6,3]}, 2.0] }"#, true),
            (r#" { "=" : [{"/": [6,0]}, null] }"#, true),
            (r#" { "=" : [{"var" : "champ.name"}, "Fezzig"] }"#, true),
            (
                r#" { "=" : [{"var" : "challenger.name"}, "Dread Pirate Roberts"] }"#,
                true,
            ),
            (r#" { "=" : [{"var" : "rounds"}, 4] }"#, true),
            (
                r#"{
                "and": [
                    { ">": [3, 1] },
                    { "<": [1, 3] }
                ]
            }"#,
                true,
            ),
        ];

        let context: Value = serde_json::from_str(
            r#"{
            "rounds" : 4, 
            "champ" : {
              "name" : "Fezzig",
              "height" : 223
            },
            "challenger" : {
              "name" : "Dread Pirate Roberts",
              "height" : 183
            }
          }"#,
        )?;

        for (data, expected) in cases {
            let p: AllCombined = serde_json::from_str(data)?;
            let res = p.execute(&context);
            assert_eq!(res, AllCombined::Primitive(Value::Bool(expected)));
        }
        Ok(())
    }
}
