enum StatementKind {
    SELECT,
    INSERT,
}

struct Statement {
    kind: StatementKind
}

fn prepare_statement(command: &str) -> Result<Statement, String> {
    match command {
        "SELECT" => Ok(Statement {  kind: StatementKind::SELECT }),
        "INSERT" => Ok(Statement {  kind: StatementKind::INSERT }),
        _ => Err(format!("Unrecognized command '{command}'"))
    }
}

fn execute_statement(statement: &Statement) {
    match statement.kind {
        StatementKind::SELECT => println!("Executing a SELECT statement"),
        StatementKind::INSERT => println!("Executing an INSERT statement"),
    }
}

pub fn eval(command: &str) {  
    match prepare_statement(command) {
        Ok(statement) => execute_statement(&statement),
        Err(err) => println!("{err}"),
    }
}
