use rig::{agent::Agent, completion::Prompt, providers::openai::CompletionModel};
use std::{
    io::Write,
    sync::{mpsc, mpsc::Receiver, mpsc::Sender, Arc},
};

fn remove_empty_lines(s: &str) -> String {
    s.lines()
        .filter(|line| !line.trim().is_empty())
        .collect::<Vec<&str>>()
        .join("\n")
}

pub async fn handle_command(
    agent: Arc<Agent<CompletionModel>>,
    line: &String,
    tx: &Sender<(String, String)>,
) {
    let response = agent.prompt(&line).await;
    match response {
        Ok(output) => {
            // writeln!(std::io::stdout(), "{}", output).unwrap();
            tx.send((line.clone(), output)).unwrap();
        }
        Err(err) => {
            writeln!(std::io::stdout(), "Error: {}", err).unwrap();
        }
    }
}

pub fn handle_receiver(message: (String, String)) -> Option<()> {
    writeln!(
        std::io::stdout(),
        "press 'enter' to save {} , press 'c' to cancel",
        &message.0
    )
    .unwrap();
    let confirmation = readline();
    match confirmation {
        Ok(buffer) if buffer.trim() == "c" => {
            writeln!(std::io::stdout(), "Cancelled").unwrap();
            Some(())
        }
        Ok(b) => {
            writeln!(std::io::stdout(), "Writing... {}", b).unwrap();
            let mut file = std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open("output.txt")
                .unwrap();
            writeln!(
                file,
                "{};\"{}\"",
                &message.0,
                remove_empty_lines(&message.1)
            )
            .unwrap();
            None
        }
        Err(_) => {
            writeln!(std::io::stdout(), "Error receiving confirmation").unwrap();
            None
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
