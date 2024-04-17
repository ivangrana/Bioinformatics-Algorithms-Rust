#[derive(Debug)]
struct Place {
    name: String,
    tokens: u32,
}

impl Place {
    fn new(name: &str, tokens: u32) -> Self {
        Self {
            name: name.to_string(),
            tokens,
        }
    }

    fn add_tokens(&mut self, amount: u32) {
        self.tokens += amount;
    }

    fn remove_tokens(&mut self, amount: u32) -> Result<(), &'static str> {
        if self.tokens >= amount {
            self.tokens -= amount;
            Ok(())
        } else {
            Err("Not enough tokens")
        }
    }
}

#[derive(Debug)]
struct Transition<'a> {
    name: String,
    inputs: Vec<(&'a mut Place, u32)>,
    outputs: Vec<(&'a mut Place, u32)>,
}

impl<'a> Transition<'a> {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    fn add_input(&mut self, place: &'a mut Place, weight: u32) {
        self.inputs.push((place, weight));
    }

    fn add_output(&mut self, place: &'a mut Place, weight: u32) {
        self.outputs.push((place, weight));
    }

    fn fire(&mut self) -> Result<(), &'static str> {
        // Check if the transition can fire
        for (place, weight) in &self.inputs {
            if place.tokens < *weight {
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

fn main() {
    // Define places
    let mut glucose = Place::new("Glucose", 10); // Initial amount of glucose
    let mut atp = Place::new("ATP", 5); // Initial amount of ATP
    let mut pyruvate = Place::new("Pyruvate", 0); // Initial amount of pyruvate

    // Initial state
    println!("Initial state:");
    println!("{:?}", glucose);
    println!("{:?}", atp);
    println!("{:?}", pyruvate);

    // Define transition
    let mut glycolysis = Transition::new("Glycolysis");
    glycolysis.add_input(&mut glucose, 1); // Consumes 1 glucose
    glycolysis.add_input(&mut atp, 2); // Consumes 2 ATP
    glycolysis.add_output(&mut pyruvate, 2); // Produces 2 pyruvate

    // Fire the transition (simulate a step in the metabolic pathway)
    match glycolysis.fire() {
        Ok(()) => {
            println!("\nAfter firing the transition (Glycolysis):");
            println!("{:?}", glucose);
            println!("{:?}", atp);
            println!("{:?}", pyruvate);
        }
        Err(err) => println!("Failed to fire transition: {}", err),
    }
}
