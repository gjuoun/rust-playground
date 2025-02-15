use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Network failure: {source}")]
    Network {
        #[from]
        source: reqwest::Error,
    },
    
    #[error("Invalid HTTP status: {status}")]
    HttpStatus {
        status: reqwest::StatusCode,
    },
    
    #[error("Validation errors:\n{}", .errors.iter().map(|(f, r)| format!("  - {}: {}", f, r)).collect::<Vec<_>>().join("\n"))]
    Validation {
        errors: Vec<(String, String)>,
    },
    
    #[error("JSON parsing failed: {source}")]
    JsonParse {
        #[from]
        source: serde_json::Error,
    },
}

// Implement conversion from garde::Report
impl From<garde::Report> for AppError {
    fn from(report: garde::Report) -> Self {
        let errors = report
            .iter()
            .map(|(path, error)| {
                (path.to_string(), error.message().to_string())
            })
            .collect();

        AppError::Validation { errors }
    }
}
