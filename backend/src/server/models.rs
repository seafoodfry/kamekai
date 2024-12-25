use serde::Serialize;
use std::error::Error;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum BuilderError {
    MissingField(&'static str),
    InvalidValue(&'static str),
}

impl fmt::Display for BuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuilderError::MissingField(field) => write!(f, "Missing required field: {}", field),
            BuilderError::InvalidValue(msg) => write!(f, "Invalid value: {}", msg),
        }
    }
}

impl Error for BuilderError {}

#[derive(Debug, Serialize)]
pub struct LanguageTranslation {
    translation: String,
    pronunciation: String,
    grammar: String,
}

#[derive(Debug, Default)]
pub struct LanguageTranslationBuilder {
    translation: Option<String>,
    pronunciation: Option<String>,
    grammar: Option<String>,
}

impl LanguageTranslationBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    // Each setter method takes ownership of self and returns Self
    // This enables method chaining
    pub fn translation(mut self, value: impl Into<String>) -> Self {
        self.translation = Some(value.into());
        self
    }

    pub fn pronunciation(mut self, value: impl Into<String>) -> Self {
        self.pronunciation = Some(value.into());
        self
    }

    pub fn grammar(mut self, value: impl Into<String>) -> Self {
        self.grammar = Some(value.into());
        self
    }

    pub fn build(self) -> Result<LanguageTranslation, BuilderError> {
        // Using the ? operator with our custom error type makes validation cleaner
        let translation = self
            .translation
            .ok_or(BuilderError::MissingField("translation"))?;
        let pronunciation = self
            .pronunciation
            .ok_or(BuilderError::MissingField("pronunciation"))?;
        let grammar = self.grammar.ok_or(BuilderError::MissingField("grammar"))?;

        // Here we could add more validation if needed
        if translation.is_empty() {
            return Err(BuilderError::InvalidValue("translation cannot be empty"));
        }

        Ok(LanguageTranslation {
            translation,
            pronunciation,
            grammar,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct Translation {
    original: String,
    japanese: LanguageTranslation,
    chinese: LanguageTranslation,
}

#[derive(Debug, Default)]
pub struct TranslationBuilder {
    original: Option<String>,
    japanese: Option<LanguageTranslation>,
    chinese: Option<LanguageTranslation>,
}

impl TranslationBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn original(mut self, value: impl Into<String>) -> Self {
        self.original = Some(value.into());
        self
    }

    pub fn japanese(mut self, value: LanguageTranslation) -> Self {
        self.japanese = Some(value);
        self
    }

    pub fn chinese(mut self, value: LanguageTranslation) -> Self {
        self.chinese = Some(value);
        self
    }

    pub fn build(self) -> Result<Translation, BuilderError> {
        Ok(Translation {
            original: self
                .original
                .ok_or(BuilderError::MissingField("original"))?,
            japanese: self
                .japanese
                .ok_or(BuilderError::MissingField("japanese"))?,
            chinese: self.chinese.ok_or(BuilderError::MissingField("chinese"))?,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct TranslationResponse {
    translations: Vec<Translation>,
}

#[derive(Debug, Default)]
pub struct TranslationResponseBuilder {
    translations: Vec<Translation>,
}

impl TranslationResponseBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_translation(mut self, translation: Translation) -> Self {
        self.translations.push(translation);
        self
    }

    pub fn build(self) -> TranslationResponse {
        TranslationResponse {
            translations: self.translations,
        }
    }
}

// Add convenient builder creation methods to our main types.
impl LanguageTranslation {
    pub fn builder() -> LanguageTranslationBuilder {
        LanguageTranslationBuilder::new()
    }
}

impl Translation {
    pub fn builder() -> TranslationBuilder {
        TranslationBuilder::new()
    }
}

impl TranslationResponse {
    pub fn builder() -> TranslationResponseBuilder {
        TranslationResponseBuilder::new()
    }
}

#[derive(serde::Deserialize)]
pub struct TranslationRequest {
    pub text: String,
}
