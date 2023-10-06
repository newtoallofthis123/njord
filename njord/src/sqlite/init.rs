use crate::table::Table;
use log::info;
use rusqlite::Result;

// initialize database with tables
pub fn init(mut conn: rusqlite::Connection, tables: Vec<Box<dyn Table>>) -> Result<()> {
    // create a transaction
    let tx = conn.transaction()?;

    // execute all sql statements based on tables vector parameter
    for t in &tables {
        let statement = generate_statement(&**t);
        tx.execute(&statement, [])?;
    }

    // commit the transaction
    tx.commit()?;

    info!("Initialize database, done.");

    Ok(())
}

// generate sql statement for create table
fn generate_statement(table: &dyn Table) -> String {
    // generate the column definitions based on the hashmap
    let mut column_definitions = String::new();
    for (column_name, column_type) in table.get_columns() {
        column_definitions.push_str(&format!("{} {}, ", column_name, column_type));
    }

    // remove the trailing comma and space
    column_definitions.pop();
    column_definitions.pop();

    let sql = format!(
        "CREATE TABLE {} (id INTEGER PRIMARY KEY, {});",
        table.get_name(),
        column_definitions
    );

    println!("{}", sql);

    sql
}