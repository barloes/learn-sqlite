// takes in string and return COMMAND, ARGUMENTS
// if command is not recognized, return None
enum Statement {
    SELECT,
    INSERT,
}

// ignore this since it's not used
// #[allow(non_snake_case)]
// enum MetaCommandResult {
//     SUCCESS,
//     UNRECOGNIZED_COMMAND
// }

pub(crate) struct Eval {

}

impl Eval {
    pub(crate) fn new() -> Eval {
        Eval {}
    }

    pub fn eval(&self, input: &str) {
        let statement = prepare_statement(input);
        match statement {
            Some(statement) => {
                execute_statement(statement);
            },
            None => {
                println!("unrecognized command");
            }
        }
    }
}

fn prepare_statement(statement: &str) -> Option<Statement> {
    match statement {
        "select" => Some(Statement::SELECT),
        "insert" => Some(Statement::INSERT),
        _ => None
    }
}

fn execute_statement(statement: Statement) {
    match statement {
        Statement::SELECT => {
            println!("executing SELECT")
        },
        Statement::INSERT => {
            println!("executing INSERT")
        }
    }
}
