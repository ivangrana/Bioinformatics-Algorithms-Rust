#[derive(Debug)]
pub struct Place {
    pub name: String,
    tokens: u32,
}

impl Place {
    pub fn new(name: &str, tokens: u32) -> Self {
        Self {
            name: name.to_string(),
            tokens,
        }
    }

    pub fn add_tokens(&mut self, amount: u32) {
        self.tokens += amount;
    }

    pub fn remove_tokens(&mut self, amount: u32) -> Result<(), &'static str> {
        if self.tokens >= amount {
            self.tokens -= amount;
            Ok(())
        } else {
            Err("Not enough tokens")
        }
    }
    pub fn get_tokens(&self) -> u32 { 
        self.tokens
    }
}