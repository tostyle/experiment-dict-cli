use clap::Command;
use rig::{agent::Agent, completion::Prompt, providers::openai::CompletionModel};
use std::{
    io::Write,
    sync::{mpsc, Arc},
};

mod agent;
use agent::create_agent;
#[tokio::main]
async fn main() {
    let (tx, rx): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
    // let agent = create_agent();
    let agent = Arc::new(create_agent());
    loop {
        let line = readline();
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }

        let agent = Arc::clone(&agent);

        tokio::spawn(async move {
            let response = &agent.prompt(&line).await;
            match response {
                Ok(output) => {
                    writeln!(std::io::stdout(), "{}", output).unwrap();
                }
                Err(err) => {
                    writeln!(std::io::stdout(), "Error: {}", err).unwrap();
                }
            }
        });
    }
}

fn readline() -> Result<String, String> {
    write!(std::io::stdout(), "$ ").map_err(|e| e.to_string())?;
    std::io::stdout().flush().map_err(|e| e.to_string())?;
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}
