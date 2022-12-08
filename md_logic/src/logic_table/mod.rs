use crate::context::var_to_operand;

use crate::expression_parser::executor::interpret;
use crate::expression_parser::operand::{Operand, Operator};
use crate::expression_parser::tokenizer::Tokenizer;
use serde::Serialize;
use serde_json::Value;

use std::collections::HashMap;

#[derive(Debug)]
pub struct Definition {
    pub inputs: Vec<(String, String)>,
    pub outputs: Vec<(String, String)>,
}

#[derive(Debug)]
pub struct Table {
    pub rows: Vec<Row>,
    pub defs: Definition,
}

#[derive(Debug)]
pub struct Row {
    pub cells: Vec<String>,
}

pub fn parse(contents: &str) -> Result<Table, String> {
    let mut table: Table = Table {
        rows: vec![],
        defs: Definition {
            inputs: vec![],
            outputs: vec![],
        },
    };

    let mut current_line = 0;
    let mut definition_rows: Vec<Row> = vec![];

    for untrimmed_line in contents.lines() {
        let line = untrimmed_line.trim();
        if line.len() == 0 {
            continue;
        }

        let mut columns: Vec<&str> = line.split("|").collect();

        // note - split by | will also create/have empty column on left first place, and most right
        if columns.len() < 4 {
            return Err("incorrect table column size - need at least 1 in, 1 out".to_owned());
        }

        columns.pop();
        columns.remove(0);

        let mut row = Row { cells: vec![] };

        for column_content in columns {
            row.cells.push(column_content.trim().to_string());
        }

        // 4 is a number of required definition rows
        if current_line >= 4 {
            table.rows.push(row);
        } else {
            definition_rows.push(row);
        }

        current_line += 1;
    }

    if table.rows.len() == 0 {
        return Err("table has incorrect data row size".to_owned());
    }

    if definition_rows.len() != 4 {
        return Err("table definitions are not correct".to_string());
    }

    let header_row: usize = 0;
    let io_row: usize = 1;
    let type_row: usize = 2;

    for col_index in 0..definition_rows[header_row].cells.len() {
        let io_def = &definition_rows[io_row].cells[col_index];
        let column_variable = &definition_rows[header_row].cells[col_index];
        let type_variable = &definition_rows[type_row].cells[col_index];

        if io_def.starts_with("-") && io_def.ends_with("-") {
            table
                .defs
                .inputs
                .push((column_variable.clone(), type_variable.clone()));
        } else if io_def.ends_with("-:") {
            table
                .defs
                .outputs
                .push((column_variable.clone(), type_variable.clone()));
        }
    }

    Ok(table)
}

#[derive(Serialize)]
pub struct TableOutputs {
    pub list: Vec<HashMap<String, Operand>>,
}

pub fn run_table(table: &Table, context: &serde_json::Value) -> Result<TableOutputs, String> {
    let mut outputs: Vec<HashMap<String, Operand>> = vec![];
    let mut row_is_true;

    for row_index in 0..table.rows.len() {
        row_is_true = true;

        for col_index in 0..table.defs.inputs.len() {
            let (var_name, _var_type) = &table.defs.inputs[col_index];
            let input_operand = var_to_operand(var_name, &context);
            let column_value = &table.rows[row_index].cells[col_index];
            let mut parser = Tokenizer::new(&column_value);

            parser.parse()?;

            let start_with_operand = parser.starts_with_operand();
            if !start_with_operand {
                parser.insert_start(input_operand);
                parser.insert_start(Operand::OperatorToken(Operator::E));
            } else {
                parser.insert_start(input_operand);
            }

            let expression = parser.to_postfix()?;
            let expr_result = interpret(&expression);
            if let Some(
                // Operand::Boolean(true)
                Operand::Primitive(Value::Bool(true)),
            ) = expr_result.get(0)
            {
                row_is_true = true;
            } else {
                row_is_true = false;
                break;
            }
        }

        if row_is_true {
            let mut output_result: HashMap<String, Operand> = HashMap::new();

            let offset = table.defs.inputs.len();
            for col_index in 0..table.defs.outputs.len() {
                let column_output_value = &table.rows[row_index].cells[col_index + offset];
                let (out_key, _operand_type) = &table.defs.outputs[col_index];
                output_result.insert(
                    out_key.to_owned(),
                    Operand::Primitive(Value::String(column_output_value.to_owned())),
                );
            }

            outputs.push(output_result);
        }
    }

    Ok(TableOutputs { list: outputs })
}

mod tests {
    use super::*;
    use std::fs;

    fn get_test_table() -> Result<Table, String> {
        let contents = fs::read_to_string("./samples/table.md")
            .expect("Something went wrong reading the TEST file");

        parse(&contents)
    }

    #[test]
    fn correct_md_table_size() -> Result<(), String> {
        let table = get_test_table()?;
        assert_eq!(table.defs.inputs.len(), 2);
        assert_eq!(table.defs.outputs.len(), 1);
        assert_eq!(table.rows.len(), 2); // actual 2 data/logic rows

        assert_eq!(table.defs.inputs[0].1, "string".to_owned());
        assert_eq!(table.defs.inputs[1].1, "number".to_owned());
        assert_eq!(table.defs.outputs[0].1, "string".to_owned());

        assert_eq!(table.rows[1].cells[2], "\"Roastbeef\"".to_owned());

        Ok(())
    }

    #[test]
    fn execute_md_table() -> Result<(), String> {
        let table = get_test_table()?;
        let json_str = r#"
        { "season": "Fall", "guestCount": 8 }
        "#;

        let context: serde_json::Value = serde_json::from_str(json_str).unwrap();
        let TableOutputs { list } = run_table(&table, &context)?;

        assert_eq!(list.len(), 1);

        let firs_res = &list[0];
        assert_eq!(firs_res.contains_key("desiredDish"), true);
        assert_eq!(
            firs_res.get("desiredDish"),
            Some(&Operand::Primitive(Value::String(
                "\"Spaceribs\"".to_owned()
            )))
        );

        Ok(())
    }

    #[test]
    fn md_table_expect_failure_insufficient_wrows() -> Result<(), String> {
        let contents = r#"
        | season   | guestCount | desiredDish |
        |----------|------------|------------:|
        | string   | number     |      string |
        | ##       | ##         |          ## |
        "#;

        let table = parse(&contents);

        match table {
            Ok(_) => Err("table should be broken".to_string()),
            Err(_) => Ok(()),
        }
    }
}
