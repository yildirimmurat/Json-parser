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
        let mut result: HashMap<String, String> = HashMap::new();

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
            let value: String = self.parse_value()?; // not only string but can also be integer, null, or boolean (true, false)
            
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

    fn parse_value(&mut self) -> Result<String, String> {
        let ch: Option<char> = self.peek();
        match ch {
            Some('"') => self.parse_string(),
            Some('0'..='9') => self.parse_integer(),
            Some('t') | Some('f') | Some('n') => self.parse_bool_or_null(),
            _ => Err("Unexpected character while parsing".to_string()),
        }
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

    fn parse_integer(&mut self) -> Result<String, String> {
        let mut result: String = String::new();

        while let Some(ch) = self.peek() {
            if ch.is_digit(10) {
                result.push(ch);
                self.consume();
            } else {
                break;
            }
        }

        if result.is_empty() {
            return Err("Expected integer".to_string());
        }

        Ok(result)
    }

    fn parse_bool_or_null(&mut self) -> Result<String, String> {
        let mut result: String = String::new();
        let ch: Option<char> = self.peek();

        match ch {
            Some('t') => {
                if self.input[self.position..].starts_with("true") {
                    self.position += 4;
                    return Ok("true".to_string());
                }
            },
            Some('f') => {
                if self.input[self.position..].starts_with("false") {
                    self.position += 5;
                    return Ok("false".to_string());
                }
            },
            Some('n') => {
                if self.input[self.position..].starts_with("null") {
                    self.position += 4;
                    return Ok("null".to_string());
                }
            },
            _ => {}
        }

        Err("Expected bool or null".to_string())
    }
}
