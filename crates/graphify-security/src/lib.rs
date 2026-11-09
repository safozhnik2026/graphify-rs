//! Path and label validation/sanitization for graphify.
//!
//! Ensures all file paths and graph labels are safe and well-formed
//! before use. Port of Python `security.py`.

pub mod label_validator;
pub mod path_validator;

pub use label_validator::sanitize_label;
pub use path_validator::{safe_path, validate_graph_path};

use thiserror::Error;

/// Security validation errors.
#[derive(Debug, Error)]
pub enum SecurityError {
    #[error("Path traversal detected: {0}")]
    PathTraversal(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Path not found: {0}")]
    PathNotFound(String),
}
