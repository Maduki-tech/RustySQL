pub mod handler;

use crate::handler::handler::Handler;

fn main() {
    let handler = Handler::new();
    handler.read_input();
}
