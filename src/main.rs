use std::env;

use jstdlib::console::Console;

pub mod ai;
pub mod error;

#[tokio::main]
async fn main() {
    let mut args = env::args().skip(1);

    let mut system_instruction: String = "You are JayI, a helpful AI assistant.".into();

    if let Some(sys_instr) = args.next() {
        system_instruction.push_str(&format!(" {sys_instr}"));
    }

    let mut client = ai::Client::new("llama3.2", &system_instruction);

    println!("JayI — Your Terminal AI Chatbot.");
    println!("/exit - exit the program.");
    println!("/clear-screen - clear the terminal.");
    println!(
        "/clear-history - clear the session's conversation history while retaining the system instruction."
    );

    loop {
        let input: String = Console::input("\nYou: ");
        let command = input.trim();

        if input.is_empty() {
            continue;
        }

        match command {
            "/exit" => {
                println!("Exiting...");
                return;
            }

            "/clear-screen" => {
                Console::clear();
                continue;
            }

            "/clear-history" => {
                client.clear_history();
                println!("Conversation history cleared.");

                continue;
            }

            _ => {}
        }

        match client.send_prompt(&input, &None, "").await {
            Ok(_) => {}
            Err(err) => eprintln!("{err}"),
        }
    }
}
