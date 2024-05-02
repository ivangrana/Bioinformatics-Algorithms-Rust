use rand::prelude::*;
use std::collections::HashMap;

struct MarkovChain {
    // Struct for the Markov Chain
    transition_table: HashMap<String, Vec<(String, f64)>>,
}

impl MarkovChain {
    fn new() -> Self {
        // Constructor for the Markov Chain
        MarkovChain {
            transition_table: HashMap::new(),
        }
    }

    // Function to add a new transition to the Markov Chain
    fn add_transition(&mut self, from_state: &str, to_state: &str, probability: f64) {
        self.transition_table
            .entry(String::from(from_state))
            .or_insert(Vec::new())
            .push((String::from(to_state), probability));
    }

    // Function to get the next state based on the current state and a random number
    fn get_next_state(&self, current_state: &str) -> Option<String> {
        let transitions = match self.transition_table.get(current_state) {
            Some(transitions) => transitions,
            None => return None,
        };

        let mut rng = thread_rng(); // Random number generator
        let rand_num: f64 = rng.gen();
        let mut cumulative_prob = 0.0; // cumulative probability of all transitions

        for (next_state, probability) in transitions {
            cumulative_prob += probability;
            if rand_num <= cumulative_prob {
                return Some(next_state.clone());
            }
        }

        None
    }
}

fn main() {
    let mut mc = MarkovChain::new();

    // Add transitions to the Markov chain
    mc.add_transition("A", "T", 0.4);
    mc.add_transition("A", "C", 0.4);
    mc.add_transition("A", "G", 0.2);
    mc.add_transition("T", "A", 0.4);
    mc.add_transition("T", "C", 0.4);
    mc.add_transition("T", "G", 0.2);
    mc.add_transition("C", "A", 0.4);
    mc.add_transition("C", "T", 0.4);
    mc.add_transition("C", "G", 0.2);
    mc.add_transition("G", "A", 0.4);
    mc.add_transition("G", "T", 0.4);
    mc.add_transition("G", "C", 0.2);
    
    let mut current_state = String::from("A");
    println!("Starting state: {}", current_state);

    for i in 0..15 {
        if let Some(next_state) = mc.get_next_state(&current_state) {
            current_state = next_state;
            println!("Character {}: {}", i + 1, current_state);
        } else {
            println!("No valid transition found.");
            break;
        }
    }
}
