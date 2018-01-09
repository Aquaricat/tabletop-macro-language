use parser::Program;

#[derive(Debug, Deserialize, Serialize)]
pub struct Output {
    /// The original input
    pub input: String,

    /// Errors, if any
    pub errors: Vec<String>,

    /// Time to execute final output
    pub execution_time: u64,

    /// Program generated by the parser
    pub program: Option<Program>,

    /// API Version
    pub version: String,
}

impl Output {
    pub fn new (input: String) -> Output {
        let version = String::from(env!("CARGO_PKG_VERSION"));
        Output {
            input,
            execution_time: 0,
            errors: Vec::new(),
            program: None,
            version,
        }
    }
}
