use log::info;
use ollama_rs::{Ollama, generation::completion::request::GenerationRequest, models::ModelOptions};
use tokio::io::{self, AsyncWriteExt};
use tokio_stream::StreamExt;

use crate::error::Error;

pub struct Summarizer {
    ollama: Ollama,
    options: ModelOptions,
}

impl Summarizer {
    pub fn new() -> Self {
        Self {
            ollama: Ollama::default(),
            options: ModelOptions::default()
                .temperature(0.2)
                .top_k(25)
                .top_p(0.25),
        }
    }

    fn get_prompt(&self, text: &str) -> String {
        format!(
            r#"
You are an AI assistant that summarizes meeting transcriptions. Your task is to:

1. Extract the key topics and decisions discussed
2. Identify action items and their owners (if mentioned)
3. Note any important technical details or specifications
4. Highlight any unresolved questions or issues
5. Keep the summary concise but comprehensive

Please provide a well-structured summary of the following meeting transcript:
            {text}
            "#
        )
    }

    pub async fn summarize(&self, text: &str, model_name: &str) -> Result<String, Error> {
        let prompt = self.get_prompt(text);

        info!("Summarizing with model: {}", model_name);

        let response = self
            .ollama
            .generate(
                GenerationRequest::new(model_name.to_string(), prompt)
                    .options(self.options.clone()),
            )
            .await
            .map_err(|e| Error::new(&format!("Failed to summarize: {}", e)))?;

        info!("Summarization complete");

        Ok(response.response)
    }

    pub async fn summarize_stream(&self, text: &str, model_name: &str) -> Result<(), Error> {
        let prompt = self.get_prompt(text);

        info!("Summarizing in streaming mode with model: {}", model_name);

        let mut stream = self
            .ollama
            .generate_stream(
                GenerationRequest::new(model_name.to_string(), prompt)
                    .options(self.options.clone()),
            )
            .await
            .map_err(|e| Error::new(&format!("Failed to summarize: {}", e)))?;

        let mut stdout = io::stdout();
        while let Some(res) = stream.next().await {
            let responses =
                res.map_err(|e| Error::new(&format!("Failed to get response: {}", e)))?;
            for resp in responses {
                stdout
                    .write_all(resp.response.as_bytes())
                    .await
                    .map_err(|e| Error::new(&format!("Failed to write to stdout: {}", e)))?;
                stdout
                    .flush()
                    .await
                    .map_err(|e| Error::new(&format!("Failed to flush stdout: {}", e)))?;
            }
        }

        info!("Summarization complete");

        Ok(())
    }
}
