use chrono::DateTime;
use chrono::prelude::Utc;
use parser::{StepValue, Program};
use roll::Roll;
use std::collections::HashMap;
use token::Token;

#[derive(Debug, Deserialize, Serialize)]
pub struct Output {
    /// The original input
    pub input: String,

    /// Errors, if any
    pub errors: Vec<String>,

    /// Timestamp
    pub executed: DateTime<Utc>,

    /// Time to execute final output
    pub execution_time: u64,

    /// Chat messages to be sent
    pub messages: Vec<String>,

    /// Program generated by the parser
    pub program: Option<Program>,

    /// Dice rolls
    pub rolls: Vec<Roll>,

    /// Results
    pub results: HashMap<String, StepValue>,

    /// Tokens
    pub tokens: HashMap<String, Token>,
 
    /// API Version
    pub version: String,
}

impl Output {
    pub fn new (input: String) -> Output {
        let version = String::from(env!("CARGO_PKG_VERSION"));
        let executed = Utc::now();
        Output {
            input,
            executed,
            execution_time: 0,
            errors: Vec::new(),
            messages: Vec::new(),
            program: None,
            rolls: Vec::new(),
            tokens: HashMap::new(),
            results: HashMap::new(),
            version,
        }
    }
}
