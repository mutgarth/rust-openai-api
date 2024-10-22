use serde::Serialize;
use reqwest::Client;

#[derive(Serialize)]
struct RequestBody{
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let request_body = RequestBody{
        model: "gpt-4o-mini".to_string(),
        messages: vec![
            Message{
                role: "system".to_string(),
                content: "You are expert in the Quenya elf language and a very good programmer. Answer the user prompt in plain Quenya only.".to_string()
            },
            Message{
                role: "user".to_string(),
                content: "What are the strenghts of the Rust programming language?".to_string()
            }
        ]
    };

    let request_body_string = serde_json::to_string(&request_body).unwrap();

    let openai_api_key = "your-api-key-here";
    let client = Client::new();
    let url = "https://api.openai.com/v1/chat/completions";

    let res = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", openai_api_key))
        .body(request_body_string)
        .send()
        .await?;

    let json_res: serde_json::Value = res.json().await?;

    println!("{:?}", json_res);

    Ok(())
}
