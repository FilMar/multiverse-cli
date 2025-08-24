use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Months {
    pub names: Vec<String>,
    pub abbrevs: Vec<String>,
    pub meanings: Vec<String>,
}

impl Months {
    pub fn get_name_by_abbrev(&self, abbrev: &str) -> Option<&str> {
        self.abbrevs.iter().position(|a| a == abbrev)
            .map(|idx| self.names[idx].as_str())
    }
    
    pub fn get_order_by_abbrev(&self, abbrev: &str) -> Option<usize> {
        self.abbrevs.iter().position(|a| a == abbrev)
    }
    
    pub fn get_by_order(&self, order: usize) -> Option<(&str, &str)> {
        if order < self.names.len() {
            Some((&self.names[order], &self.abbrevs[order]))
        } else {
            None
        }
    }
}