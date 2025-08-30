use std::process::Command;
use tempfile::TempDir;
use std::path::PathBuf;
use anyhow::Result;

/// Test helper to run multiverse commands
pub struct MultiverseTest {
    temp_dir: TempDir,
    binary_path: PathBuf,
}

impl MultiverseTest {
    pub fn new() -> Result<Self> {
        let temp_dir = TempDir::new()?;
        let binary_path = std::env::current_dir()?.join("target/debug/multiverse");
        
        Ok(Self {
            temp_dir,
            binary_path,
        })
    }
    
    pub fn run_command(&self, args: &[&str]) -> Result<std::process::Output> {
        let output = Command::new(&self.binary_path)
            .args(args)
            .current_dir(self.temp_dir.path())
            .output()?;
        Ok(output)
    }
    
    pub fn query(&self, sql: &str) -> Result<String> {
        let output = self.run_command(&["query", sql])?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
    
    pub fn init_world(&self, name: &str) -> Result<()> {
        let output = self.run_command(&["world", "init", name, "--description", "Test world"])?;
        if !output.status.success() {
            anyhow::bail!("Failed to init world: {}", String::from_utf8_lossy(&output.stderr));
        }
        Ok(())
    }
    
    /// Parse formatted query output to extract numeric values
    fn parse_query_number(&self, query_output: &str) -> Result<i32> {
        let lines: Vec<&str> = query_output.lines().collect();
        for line in lines {
            if line.contains('│') && line.chars().any(|c| c.is_numeric()) {
                let numbers: String = line.chars().filter(|c| c.is_numeric()).collect();
                if let Ok(count) = numbers.parse::<i32>() {
                    return Ok(count);
                }
            }
        }
        Ok(0)
    }
    
    /// Parse formatted query output to extract string values from first data row
    fn parse_query_string(&self, query_output: &str) -> String {
        let lines: Vec<&str> = query_output.lines().collect();
        let mut in_data_section = false;
        
        for line in lines {
            // Skip header separators
            if line.contains("┌") || line.contains("├") || line.contains("└") {
                continue;
            }
            
            // Check for header row (contains column names in uppercase)
            if line.contains('│') && (line.contains("metadata") || line.contains("METADATA")) {
                in_data_section = true;
                continue;
            }
            
            // Extract actual data row
            if in_data_section && line.contains('│') {
                let parts: Vec<&str> = line.split('│').collect();
                if parts.len() > 1 {
                    let content = parts[1].trim();
                    if !content.is_empty() && content != "metadata" {
                        return content.to_string();
                    }
                }
            }
        }
        String::new()
    }
    
    pub fn query_count(&self, table: &str) -> Result<i32> {
        let result = self.query(&format!("SELECT COUNT(*) FROM {}", table))?;
        self.parse_query_number(&result)
    }
    
    pub fn entity_exists(&self, table: &str, name: &str) -> Result<bool> {
        let result = self.query(&format!("SELECT COUNT(*) FROM {} WHERE name = '{}'", table, name))?;
        let count = self.parse_query_number(&result)?;
        Ok(count > 0)
    }
    
    pub fn get_metadata(&self, table: &str, name: &str) -> Result<String> {
        let result = self.query(&format!("SELECT metadata FROM {} WHERE name = '{}'", table, name))?;
        Ok(self.parse_query_string(&result))
    }
    
    pub fn run_command_assert_success(&self, args: &[&str]) -> Result<std::process::Output> {
        let output = self.run_command(args)?;
        if !output.status.success() {
            anyhow::bail!(
                "Command failed: {}\nSTDERR: {}\nSTDOUT: {}",
                args.join(" "),
                String::from_utf8_lossy(&output.stderr),
                String::from_utf8_lossy(&output.stdout)
            );
        }
        Ok(output)
    }
}