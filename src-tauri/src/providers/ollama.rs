use crate::providers::base::Provider;
use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};

pub struct OllamaProvider {
    client: Ollama,
    model: String,
}

impl 