mod methods;
use crate::methods::get::get::fetch_data as get;
use crate::methods::post::post::post_data as post;
use std::io::{self, Write}; // Added Write for flushing
use crate::methods::run;


#[tokio::main]
async fn main() {
    println!("\n\n");
    println!(" ██████╗ ██████╗  █████╗ ██████╗ ██╗");
    println!("██╔════╝ ██╔══██╗██╔══██╗██╔══██╗██║");
    println!("██║  ███╗██████╔╝███████║██████╔╝██║");
    println!("██║   ██║██╔══██╗██╔══██║██╔══██╗██║");
    println!("╚██████╔╝██║  ██║██║  ██║██████╔╝███████╗");
    println!(" ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚═════╝ ╚══════╝");
    println!("\n\n");

    loop {
        let mut input = String::new();
        
        print!("command@grabl >>> ");

        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let command = input.trim();

        if command == "exit" || command == "quit" {
            println!("Exiting Grabl...");
            break;
        }

        run(command).await;
    }

}