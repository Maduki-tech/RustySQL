pub struct SqlCommandHandler {}

#[derive(strum_macros::Display)]
enum SqlCommand {
    Select,
    Insert,
    Create,
}

impl SqlCommandHandler {
    pub fn new() -> SqlCommandHandler {
        SqlCommandHandler {}
    }
    pub fn handle_sql(&self, input: String) {
        let sql = self.get_sql_command(input.clone());
        println!("Handling SQL: {}", sql);
    }

    fn get_sql_command(&self, input: String) -> String {
        let mut iter = input.split_whitespace();
        match iter.next() {
            Some("SELECT") => SqlCommand::Select.to_string().to_uppercase(),
            Some("INSERT") => SqlCommand::Insert.to_string().to_uppercase(),
            Some("CREATE") => SqlCommand::Create.to_string().to_uppercase(),
            _ => "Invalid SQL command".to_string(),
        }
    }
}
