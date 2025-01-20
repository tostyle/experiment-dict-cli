use rig::{agent::Agent, completion::Prompt, providers::openai::CompletionModel};
use std::{
    io::Write,
    sync::{mpsc, mpsc::Receiver, mpsc::Sender, Arc},
};

pub async fn handle_command(agent: Arc<Agent<CompletionModel>>, line: String, tx: Sender<String>) {
    let response = agent.prompt(&line).await;
    match response {
        Ok(output) => {
            writeln!(std::io::stdout(), "{}", output).unwrap();
            tx.send(true.to_string()).unwrap();
        }
        Err(err) => {
            writeln!(std::io::stdout(), "Error: {}", err).unwrap();
        }
    }
}

pub fn handle_receiver(rx: Receiver<String>) {
    loop {
        let receive = rx.recv();
        match receive {
            Ok(_) => {
                writeln!(std::io::stdout(), "Received response").unwrap();
            }
            Err(_) => {
                writeln!(std::io::stdout(), "Error receiving response").unwrap();
            }
        }
    }
}

pub fn readline() -> Result<String, String> {
    write!(std::io::stdout(), "$ ").map_err(|e| e.to_string())?;
    std::io::stdout().flush().map_err(|e| e.to_string())?;
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .map_err(|e| e.to_string())?;
    Ok(buffer)
}
