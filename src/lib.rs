use std::collections::HashMap;

#[derive(Debug)]
pub struct JsonParser {
    input: String,
    position: usize//why usize
}

impl JsonParser {
    pub fn new(input: &str) -> Self {
        JsonParser {
            input: input.to_string(),
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Result<HashMap<String, String>, String> {
        let mut result = HashMap::new();

        self.skip_whitespece();

        // Expect the object starts with '{'
        if self.peek() != Some('{') {
            return Err("Expected '{' at the beginning of object".to_string());
        }
        self.consume();

        let mut expects_another_item = false;  // To check if it's the first item and avoid trailing commas

        loop {
            self.skip_whitespece();

            // Check for the end of the object '}'
            if self.peek() == Some('}') {
                if (expects_another_item) {
                    return Err("Expected a new key value pair".to_string());
                }
                self.consume();
                break;
            }

            expects_another_item = false;

            // Parse key
            let key = self.parse_string()?;
            self.skip_whitespece();

            if self.peek() != Some(':') {
                return Err("Expected : after a key".to_string());
            }
            self.consume();

            self.skip_whitespece();

            // Parse value
            let value = self.parse_string()?;
            
            result.insert(key, value);
            self.skip_whitespece();

            if self.peek() == Some(',') {
                self.consume();
                expects_another_item = true;
                continue;
            } else if self.peek() == Some('}') {
                self.consume();
                break;
            } else {
                return Err("Expected , or } at the end of key and values".to_string());
            }
        }

        Ok(result)
    }

    fn skip_whitespece(&mut self) {
        while let Some(ch) = self.peek() {
            if ch.is_whitespace() {
                self.consume();
            } else {
                break;
            }
        }
    }

    fn peek(&mut self) -> Option<char> {
        self.input[self.position..].chars().next()
    }

    fn consume(&mut self) {
        self.position += self.input[self.position..].chars().next().unwrap().len_utf8();
    }

    fn parse_string(&mut self) -> Result<String, String> {
        if self.peek() != Some('"') {
            return Err("Expected \" to start a string".to_string());
        }
        self.consume();

        let mut result = String::new();
        while let Some(ch) = self.peek() {
            if ch == '"' {
                self.consume();
                return Ok(result);
            }
            result.push(ch);
            self.consume();
        }

        return Err("Expected \" to end a string".to_string());
    }
}
