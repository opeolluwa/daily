use reqwest::Client;
use serde::{Deserialize, Serialize};
use ollama_rs::{
    generation::completion::request::GenerationRequest,
    Ollama,
};
 
#[derive(Serialize)]
struct GenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
}
 
#[derive(Deserialize)]
struct GenerateResponse {
    response: String,
    done: bool,
}
 
async fn ask_ollama(prompt: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
 
    let request = GenerateRequest {
        model: "phi4".to_string(),
        prompt: prompt.to_string(),
        stream: false,  // Get complete response at once
    };
 
    let resp: GenerateResponse = client
        .post("http://localhost:11434/api/generate")
        .json(&request)
        .send()
        .await?
        .json()
        .await?;
 
    Ok(resp.response)
}
 
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to local Ollama (default: http://localhost:11434)
    let ollama = Ollama::default();
 
    // Basic generation
    let res = ollama
        .generate(GenerationRequest::new(
            "qwen2.5:7b".into(),
            "Why is Rust's type system particularly effective for systems programming?".to_string(),
        ))
        .await?;
 
    println!("{}", res.response);
    Ok(())
}