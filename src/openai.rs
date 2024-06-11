use reqwest::Client;
use serde::{Serialize, Deserialize};
use std::error::Error;

#[derive(Serialize)]
struct OpenAIRequest<'a> {
    model: &'a str,
    messages: Vec<Message<'a>>,
    max_tokens: u32,
    temperature: f32,
    top_p: f32,
    frequency_penalty: f32,
    presence_penalty: f32,
}

#[derive(Serialize)]
struct Message<'a> {
    role: &'a str,
    content: &'a str,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageResponse,
}

#[derive(Deserialize)]
struct MessageResponse {
    content: String,
}

pub async fn send_prompt_to_openai_code_reviwer(prompt: &str, api_key: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let messages = vec![
        Message {
            role: "system",
            content: get_code_reviewer_ai_role(),
        },
        Message {
            role: "user",
            content: prompt,
        },
    ];
    let request = OpenAIRequest {
        model: "gpt-4o-2024-05-13", // Adjust the model as necessary
        messages,
        max_tokens: 1024,
        temperature: 1.0,
        top_p: 1.0,
        frequency_penalty: 0.0,
        presence_penalty: 0.0
    };

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request)
        .send()
        .await?;

    if response.status().is_success() {
      let openai_response: OpenAIResponse = response.json().await?;
      if let Some(choice) = openai_response.choices.first() {
        Ok(choice.message.content.clone())
      } else {
        Err("No choices found in response".into())
      }
    } else {
      let error_message = format!("Request failed with status: {}, {}", response.status(), response.json::<serde_json::Value>().await?.to_string());
      Err(error_message.into())
    }
}

fn get_code_reviewer_ai_role() -> &'static str {
  "As an AI code reviewer, your role is to assist in the code review process by 
  analyzing changes in Merge Requests (MRs) within the software development pipeline. 
  Your primary responsibilities include identifying potential issues, suggesting improvements, 
  and ensuring adherence to best coding practices. 
  You will provide feedback on syntax, logic, efficiency, style, and security aspects of the code."
}
