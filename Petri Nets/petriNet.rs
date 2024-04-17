//Petri net for metabolic flux model.
#[derive(Debug)]
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

struct Transition<'a> {
    // struct for the transition
    name: String,
    input: &'a mut Place,
    weight: i32,
}

impl<'a> Transition<'a> {
    fn new(name: &str, new_weight: i32, origin: &'a mut Place) -> Self {
        Self {
            name: name.to_string(),
            input: origin,
            weight: new_weight,
        }
    }
    fn fire_consumption(&mut self) {
        println!("before {} fire_consumption:\n{:?}\n", self.name, self.input);
        self.input.remove_tokens(self.weight);
        println!("{} -> {:?}", self.input.name, self.input)
    }

    fn fire_production(&mut self) {
        println!("before {} fire_production:\n{:?}\n", self.name, self.input);
        self.input.add_tokens(self.weight);
        println!("{} -> {:?}", self.input.name, self.input)
    }
}

fn main() {
    // Create new places for the reagents's tokens and the product's tokens.
    let mut hydrogen = Place::new("hydrogen", 2);
    let mut oxygen = Place::new("oxygen", 3);
    let mut water = Place::new("water", 0);

    // Create a transition
    let mut reaction0 = Transition::new("Hydrogen consumption", 2, &mut hydrogen);
    let mut reaction1 = Transition::new("Oxygen consumption", 1, &mut oxygen);
    let mut reaction2 = Transition::new("Water production", 2, &mut water);
    // Fire the transitions
    reaction0.fire_consumption();
    reaction1.fire_consumption();
    reaction2.fire_production();
}
