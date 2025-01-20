mod agent;
mod cli;

use agent::create_agent;
use cli::{handle_command, handle_receiver, readline};
use rig::{agent::Agent, providers::openai::CompletionModel};
use std::{
    io::Write,
    sync::{mpsc, mpsc::Receiver, mpsc::Sender, Arc},
};
#[tokio::main]
async fn main() {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
    let agent = Arc::new(create_agent());
    tokio::spawn(async move {
        handle_receiver(rx);
    });
    loop {
        let line = readline();
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }

        let agent: Arc<Agent<CompletionModel>> = Arc::clone(&agent);
        let tx = tx.clone();
        tokio::spawn(async move {
            handle_command(agent, line, tx).await;
            // tx.send(true.to_string()).unwrap();
        });
    }
}
