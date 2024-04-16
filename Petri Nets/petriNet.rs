//Petri net for metabolic flux model.
struct Place {
    name: String,
    tokens: i32,
}

impl Place {
    fn new(name: &str, tokens: i32) -> Self {
        Self {
            name: name.to_string(),
            tokens,
        }
    }

    fn add_tokens(&mut self, count: i32) {
        self.tokens += count;
    }

    fn remove_tokens(&mut self, count: i32) -> bool {
        if self.tokens >= count {
            self.tokens -= count;
            true
        } else {
            false
        }
    }
}

struct Transition { // struct for the transition
    name: String,
    inputs: Vec<Place>,
    outputs: Vec<Place>,
}

impl Transition {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }

    fn add_input(&mut self, place: Place){
        self.inputs.push(place);
    }
    fn add_output(&mut self, place: Place){
        self.outputs.push(place);
    }

    fn fire(&mut self){ // function to fire the transition
        for input in &mut self.inputs {
            if !input.remove_tokens(1) {
                panic!("Not enough tokens in place {}", input.name);
            }
        }
        for output in &mut self.outputs {
            output.add_tokens(1);
        }
    }
}


fn main() {
    // Create new places for the reagents's tokens and the product's tokens.
    let hydrogen = Place::new("hydrogen", 2);
    let oxygen = Place::new("oxygen", 3);
    let water = Place::new("water", 0);

    // Create a transition
    let mut reaction = Transition::new("Water synthesis"); 

    // Add inputs and outputs
    reaction.add_input(hydrogen);
    reaction.add_input(oxygen);
    reaction.add_output(water);

    // fire the transition
    reaction.fire();
}
