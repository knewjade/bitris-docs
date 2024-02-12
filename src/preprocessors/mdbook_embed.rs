use std::fs;

use regex::{Captures, Regex};

use crate::mapping_processor::{Mapper, MappingProcessor};
use crate::process::execute;

mod process;
mod mapping_processor;

fn main() {
    let mapper = LoadingMapper::new();
    let processor = MappingProcessor::new(Box::new(mapper));
    execute(&processor).expect("error")
}

struct LoadingMapper;

impl LoadingMapper {
    fn new() -> Self {
        Self
    }

    fn read_from_file(&self, filename: &str, method_name: &str) -> String {
        let filepath = format!(r"src\examples\{}.rs", filename);
        let content = fs::read_to_string(filepath)
            .expect("Failed to read the content");

        let imports_text = self.read_imports_text(&content)
            .map(|text| text + "\n\n")
            .unwrap_or("".to_string());

        let code = self.read_code(method_name, &content);

        format!("```rust\n{}{}\n```", imports_text, code)
    }

    fn read_imports_text(&self, content: &String) -> Option<String> {
        let regex = Regex::new(r"// ### START IMPORTS ###(?P<body>[\s\S]*)// ### END IMPORTS ###")
            .expect("Failed to build regex to read the imports");

        let captures = regex.captures(&content)?;

        let body = captures.name("body")
            .map(|m| m.as_str())
            .unwrap_or("")
            .replace("\r", "");

        let lines: Vec<&str> = body.split("\n")
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect();

        if lines.is_empty() {
            return None;
        }

        let text = lines.iter()
            .map(|line| line.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        Some(text)
    }

    fn read_code(&self, method_name: &str, content: &String) -> String {
        let body = self.read_code_from_content(&method_name, &content);

        let lines: Vec<&str> = body.split("\n")
            .map(|line| line.trim_end())
            .collect();

        let min_indent = lines.iter()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let before = line.len();
                let after = line.trim_start().len();
                before - after
            })
            .min()
            .unwrap_or(0);

        let formatted_code = lines.iter()
            .map(|line| {
                if line.is_empty() {
                    line.to_string()
                } else {
                    line.chars().skip(min_indent).collect()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        formatted_code.trim().to_string()
    }

    fn read_code_from_content(&self, method_name: &&str, content: &&String) -> String {
        // Search for `fn <method_name>() { ... ファイルの末尾の}`
        let regex = Regex::new(&format!(r"fn {}\(\)\s*?\{{(?P<body>[\s\S]*)\}}", method_name))
            .expect("Failed to build regex to read the code");

        let captures = regex.captures(&content)
            .expect("Failed to find the code");

        let body = captures.name("body")
            .expect("Failed to read the code body")
            .as_str()
            .replace("\r", "");

        let mut opened_braces = 1;
        let mut result = String::with_capacity(body.len());
        for ch in body.chars() {
            match ch {
                '{' => opened_braces += 1,
                '}' => opened_braces -= 1,
                _ => (),
            }
            if opened_braces == 0 {
                break;
            }
            result.push(ch);
        }

        result
    }
}

impl Mapper for LoadingMapper {
    fn name(&self) -> &str {
        "load-preprocessor"
    }

    fn exec(&self, content: &String) -> String {
        // Search for `{{#embed <file_name>::<method_name>}}`
        let regex_embed_tag = Regex::new(r"\{\{\s*?#embed\s+?(?P<file>\S+?)::(?P<method>[\S\s]*?)}}")
            .expect("Failed to build regex to find the embed tags");

        regex_embed_tag.replace_all(content, |embed_captures: &Captures| {
            let filename = embed_captures.name("file")
                .expect("Failed to read the file name")
                .as_str();

            let method_name = embed_captures.name("method")
                .expect("Failed to read the method name")
                .as_str();

            self.read_from_file(filename, method_name)
        }).to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_remove_extern_crate_statement() {
        let content = r##"{{#embed hello_world::foo}} {{#embed hello_world::bar}}"##;
        let expected = r##"```rust
use std::println;

println!("hello world!");
``` ```rust
use std::println;

{
    println!("hello");
}
{
    println!("world");
}
```"##;

        let embed = LoadingMapper::new();
        let actual = embed.exec(&content.to_string());
        assert_eq!(actual, expected);
    }
}
