#[derive(Debug)]
pub enum DiagnosticCategory {
    Error,
    Warning,
    Suggestion,
    Message,
}

#[derive(Debug)]
pub struct DiagnosticMessage {
    pub code: i32,
    pub category: DiagnosticCategory,
    pub key: String,
    pub message: String,
    pub reports_unnecessary: Option<bool>,
    pub elided_in_compatibility_pyramid: Option<bool>,
    pub reports_deprecated: Option<bool>,
} 