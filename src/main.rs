use agent::create_agent;
use clap::Arg;
use cli::{handle_command, handle_receiver, readline};
use experiment_dict_cli::{agent, cli};
use rig::{agent::Agent, providers::openai::CompletionModel};
use std::{
    io::Write,
    sync::{mpsc, mpsc::Receiver, mpsc::Sender, Arc},
};

fn clear_stdout() {
    print!("\x1B[2J\x1B[1;1H");
    std::io::stdout().flush().unwrap();
}

fn clear_stdin() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
}

#[tokio::main]
async fn main() {
    let (tx, rx): (Sender<(String, String)>, Receiver<(String, String)>) = mpsc::channel();
    let agent = Arc::new(create_agent());

    loop {
        let message = rx.try_recv();
        if let Ok(message) = message {
            handle_receiver(message);
            std::io::stdout().flush().unwrap();
        }
        let line = readline();
        let line = line.unwrap();
        let line = line.trim().to_string();
        if line.is_empty() {
            continue;
        }
        writeln!(std::io::stdout(), "Processing... '{}'", line).unwrap();

        let agent: Arc<Agent<CompletionModel>> = Arc::clone(&agent);
        let tx = tx.clone();
        tokio::spawn(async move {
            handle_command(agent, &line, &tx).await;
            // tx.send(line.to_string()).unwrap();
        });
    }
}
