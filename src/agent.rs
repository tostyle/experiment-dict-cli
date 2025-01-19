use rig::{
    agent::Agent,
    providers::{self, openai::CompletionModel},
};

pub fn create_agent() -> Agent<CompletionModel> {
    let client = providers::openai::Client::from_url("ollama", "http://localhost:11434/v1");
    let agent = client
        .agent("llama3.2")
        .preamble(
            "You are an English translator. \
            I will give you a vocabulary word, slang, and you must provide: \
            The meaning of the word along with its type (e.g., noun, verb, adjective, adverb). \
            An example sentence using the word in context. \
            Respond in the following format: \
            [meaning] <your response here>  
            [example] <your response here>",
        )
        .build();
    agent
}
