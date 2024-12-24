use crate::error::AppError;
use aws_sdk_bedrockruntime::types::{ContentBlock, ConversationRole, Message};

#[derive(Debug)]
pub struct ConversationBuilder {
    system_prompt: Option<String>,
    conversation_history: Vec<String>,
}

impl ConversationBuilder {
    pub fn new() -> Self {
        Self {
            system_prompt: None,
            conversation_history: Vec::new(),
        }
    }

    pub fn with_system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system_prompt = Some(prompt.into());
        self
    }

    pub fn add_user_message(mut self, message: impl Into<String>) -> Self {
        self.conversation_history
            .push(format!("User: {}", message.into()));
        self
    }

    #[allow(dead_code)]
    pub fn add_assistant_message(mut self, message: impl Into<String>) -> Self {
        self.conversation_history
            .push(format!("Assistant: {}", message.into()));
        self
    }

    pub fn build(self) -> Result<Message, AppError> {
        let mut content = String::new();

        // Add system prompt if present.
        if let Some(system) = self.system_prompt {
            content.push_str(&format!("System: {}\n\n", system));
        }

        // Add conversation history.
        for message in self.conversation_history {
            content.push_str(&message);
            content.push_str("\n\n");
        }

        Message::builder()
            .role(ConversationRole::User)
            .content(ContentBlock::Text(content))
            .build()
            .map_err(|e| AppError::MessageParse(format!("Failed to build message: {}", e)))
    }
}
