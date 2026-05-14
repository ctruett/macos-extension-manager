use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct OpenAtLoginItem {
    pub name: String,
    pub path: Option<PathBuf>,
    pub hidden: bool,
}
