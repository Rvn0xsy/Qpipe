use log::{debug, error};
use crate::config::Config;
use serde::{Deserialize, Serialize};
use surf::http::{mime::JSON, Method, Url};
use surf::{Client, Request};

#[derive(Serialize, Deserialize)]
struct Usage {
    pub completion_tokens: i64,
    pub prompt_tokens: i64,
    pub total_tokens: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    pub content: String,
    pub role: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Struct {
    pub finish_reason: String,
    pub index: i32,
    pub message: Message,
}


#[derive(Serialize, Deserialize)]
struct ResponseStruct {
    pub choices: Vec<Struct>,
    pub created: i64,
    pub id: String,
    pub model: String,
    pub request_id: String,
    pub usage: Usage,
}

pub struct GModel {
    model: String,
    api_key: String,
    url: String,
    prompt: String,
}



#[derive(Debug, Serialize, Deserialize)]
struct RequestStruct{
    model: String,
    text: String,
    messages: Vec<Message>,
}

impl GModel {
    pub fn new(config : &Config)-> Self {
        GModel {
            model: config.model.clone(),
            api_key: config.api_key.clone(),
            url: config.url.clone(),
            prompt: "".to_string(),
        }
    }

    pub fn set_prompt(&mut self, prompt: String) {
        self.prompt = prompt.to_string();
    }

    pub async fn ask(&self, question: String) -> String {
        let url = self.url.clone();
        let api_key = self.api_key.clone();
        let model = self.model.clone();
        let data = RequestStruct {
            model,
            text: question.to_string(),
            messages: vec![
                Message {
                role: "system".to_string(),
                content: self.prompt.to_string(),
            },
                Message {
                role: "user".to_string(),
                content: question.to_string(),
            }],
        };
        debug!("Request: {:?}", data);
        let request = Request::builder(Method::Post, Url::parse(&url).unwrap().clone(),)
            .content_type(JSON)
            .header("Authorization", format!("Bearer {}", api_key))
            .body_json(&data);
        let client = Client::new();
        let mut response = client.send(request.unwrap()).await.unwrap_or_else(|err| {
            error!("Error: {}", err);
            std::process::exit(1);
        });
        // println!("INFO: {:?}", response.body_string().await.unwrap());
        let result : ResponseStruct = response.body_json().await.unwrap_or_else(|err| {
            error!("Json Error: {}", err);
            std::process::exit(1);
        });

        
        result.choices[0].message.content.clone()
    }
}