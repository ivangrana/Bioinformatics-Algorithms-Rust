use crate::Place;

#[derive(Debug)]
pub struct Transition<'a> {
    pub name: String,
    inputs: Vec<(&'a mut Place, u32)>,  // (place, amount) tuple
    outputs: Vec<(&'a mut Place, u32)>, // (place, amount) tuple
}

impl<'a> Transition<'a> {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    pub fn add_input(&mut self, place: &'a mut Place, weight: u32) {
        self.inputs.push((place, weight));
    }

    pub fn add_output(&mut self, place: &'a mut Place, weight: u32) {
        self.outputs.push((place, weight));
    }

    pub fn fire(&mut self) -> Result<(), &'static str> {
        // Check if the transition can fire
        for (place, weight) in &self.inputs {
            if place.get_tokens() < *weight {
                return Err("Cannot fire transition: insufficient tokens in input places");
            }
        }

        // Fire the transition: consume input tokens and produce output tokens
        for (place, weight) in &mut self.inputs {
            place.remove_tokens(*weight)?;
        }
        for (place, weight) in &mut self.outputs {
            place.add_tokens(*weight);
        }

        Ok(())
    }
}
