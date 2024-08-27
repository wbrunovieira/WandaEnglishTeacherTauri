
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use rand::Rng;

#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}


const FILE_NAME: &str = "stored_data.json";

fn get_initial_instruction() -> Message {
    let mut rng = rand::thread_rng();
    let mood = if rng.gen_bool(0.5) {
        " você está bem humorada hoje."
    } else {
        " você está mais séria hoje."
    };

    let content = format!(
        "You are an English teacher named Marie, teaching a student named Stephanie. \
        Stephanie is Brazilian, lives in Portugal, and is 18 years old. \
        She is a quiet learner, so take the initiative in the lesson. \
        Instead of focusing on grammar rules, guide the lesson through natural conversation. \
        Ask questions, give examples, and provide simple exercises to engage Stephanie. \
        Correct any pronunciation mistakes and provide the correct pronunciation. \
        Do not explicitly mention grammatical terms. \
        Never speak in Portuguese. \
        Keep responses under 20 words.{}",
        mood
    );

    Message {
        role: "system".into(),
        content,
    }
}

pub fn get_recent_messages() -> Vec<Message> {
    let mut messages = vec![get_initial_instruction()];

    if Path::new(FILE_NAME).exists() {
        if let Ok(mut file) = File::open(FILE_NAME) {
            let mut data = String::new();
            if file.read_to_string(&mut data).is_ok() {
                if let Ok(mut stored_messages) = serde_json::from_str::<Vec<Message>>(&data) {
                    let len = stored_messages.len();
                    if len > 5 {
                        messages.append(&mut stored_messages);
                    } else {
                        messages.append(&mut stored_messages[len.saturating_sub(5)..].to_vec());
                    }
                }
            }
        }
    }

    messages
}

pub fn store_messages(request_message: &str, response_message: &str) {
    let mut messages = get_recent_messages();
    messages.push(Message {
        role: "user".into(),
        content: request_message.into(),
    });
    messages.push(Message {
        role: "assistant".into(),
        content: response_message.into(),
    });

    if let Ok(mut file) = OpenOptions::new().write(true).create(true).open(FILE_NAME) {
        if let Ok(data) = serde_json::to_string(&messages) {
            file.write_all(data.as_bytes()).unwrap();
        }
    }
}

pub fn reset_messages() {
    let _ = File::create(FILE_NAME);
}
