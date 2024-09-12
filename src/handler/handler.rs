use crate::handler::sql_command::SqlCommandHandler;
use std::io::{self, Write};
/// Input handler for the sql
pub struct Handler {}

impl Handler {
    pub fn new() -> Handler {
        Handler {}
    }

    pub fn read_input(&self) {
        loop {
            let mut input = String::new();
            print!("> ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).unwrap();
            self.handle_input(input)
        }
    }

    fn handle_input(&self, input: String) {
        match input.trim() {
            ".exit" => {
                println!("Exiting...");
                std::process::exit(0);
            }
            ".table" => {
                self.display_table();
            }
            ".help" => {
                self.display_help();
            }
            _ => {
                self.handle_sql(input);
            }
        }
    }

    fn display_table(&self) {
        println!("Displaying table...");
    }

    fn handle_sql(&self, input: String) {
        SqlCommandHandler::new().handle_sql(input.to_uppercase());
    }

    fn display_help(&self) {
        println!(".exit - Exit the program");
        println!(".table - Display the table");
        println!(".help - Display this help message");
    }
}
