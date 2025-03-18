use dioxus::prelude::Asset;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Quiz {
    pub title: &'static str,
    pub icon: Option<Asset>,
    pub questions: Vec<Question>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Question {
    pub stem: String,
    pub answer: String,
    pub options: Vec<String>,
}
