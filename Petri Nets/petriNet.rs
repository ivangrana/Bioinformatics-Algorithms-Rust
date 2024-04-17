mod place;
use place::Place;

#[derive(Debug)]
struct Transition<'a> {
    name: String,
    inputs: Vec<(&'a mut Place, u32)>, // (place, amount) tuple
    outputs: Vec<(&'a mut Place, u32)>, // (place, amount) tuple
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
    let mut hydrogen = Place::new("Hydrogen", 1); // Initial amount of hydrogen
    let mut oxygen = Place::new("Oxygen", 2); // Initial amount of Oxygen
    let mut water = Place::new("water", 0); // Initial amount of water

    // Initial state
    println!("Initial state:");
    println!("{:?}", hydrogen);
    println!("{:?}", oxygen);
    println!("{:?}", water);

    // Define transition
    let mut water_synthesis = Transition::new("Glycolysis");
    water_synthesis.add_input(&mut hydrogen, 2); // Consumes 1 hydrogen
    water_synthesis.add_input(&mut oxygen, 1); // Consumes 2 oxygen
    water_synthesis.add_output(&mut water, 2); // Produces 2 water molecules

    // Fire the transition (simulate a step in the metabolic pathway)
    match water_synthesis.fire() {
        Ok(()) => {
            println!("\nAfter firing the transition (Glycolysis):");
            println!("{:?}", hydrogen);
            println!("{:?}", oxygen);
            println!("{:?}", water);
        }
        Err(err) => println!("Failed to fire transition: {}", err),
    }
}
