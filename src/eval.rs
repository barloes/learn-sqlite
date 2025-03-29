use crate::db::{Row, Table};

// takes in string and return COMMAND, ARGUMENTS
// if command is not recognized, return None
enum StatementType {
    SELECT,
    INSERT,
}

pub struct Statement {
    statement_type: StatementType,
    pub(crate) row_to_insert: Row,
}

// ignore this since it's not used
// #[allow(non_snake_case)]
// enum MetaCommandResult {
//     SUCCESS,
//     UNRECOGNIZED_COMMAND
// }

pub(crate) struct Eval {
    pub db: Table,
}

impl Eval{
    pub(crate) fn new() -> Eval {
        Eval {
            db: Table::new(),
        }
    }

    pub fn eval(&mut self, input: &str) {
        let statement = prepare_statement(input);
        match statement {
            Some(statement) => {
                self.execute_statement(statement);
            },
            None => {
                println!("unrecognized command");
            }
        }
    }

    fn execute_statement(&mut self, statement: StatementType) {
        match statement {
            StatementType::SELECT => {
                println!("executing SELECT");
                self.db.execute_select();
            },
            StatementType::INSERT => {
                let statement = Statement {
                    statement_type: StatementType::INSERT,
                    row_to_insert: Row {
                        id: 312,
                        name: [3; 32],
                        email: [4; 255],
                    },
                };
                match self.db.execute_insert(&statement) {
                    Ok(_) => println!("inserted"),
                    Err(e) => println!("{}", e),
                }
                println!("executing INSERT")

            }
        }
    }
}

fn prepare_statement(statement: &str) -> Option<StatementType> {
    match statement {
        "s" => Some(StatementType::SELECT),
        "i" => Some(StatementType::INSERT),
        _ => None
    }
}
