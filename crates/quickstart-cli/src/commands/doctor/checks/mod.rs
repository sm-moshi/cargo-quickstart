//! Doctor checks implementations

pub mod dependencies;
pub mod files;
pub mod lints;
pub mod rust;
pub mod templates;

// Re-export all check structs for convenience
pub use dependencies::DependenciesCheck;
pub use files::FilesCheck;
pub use lints::LintsCheck;
pub use rust::RustToolchainCheck;
pub use templates::TemplatesCheck;
