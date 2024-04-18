mod place;
use place::Place;

mod transition;
use transition::Transition;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    // Define places
    let mut hydrogen = Place::new("Hydrogen", 2); // Initial amount of hydrogen
    let mut oxygen = Place::new("Oxygen", 2); // Initial amount of Oxygen
    let mut water = Place::new("water", 0); // Initial amount of water

    // Initial state
    println!("Initial state:");
    println!("{:?}", hydrogen);
    println!("{:?}", oxygen);
    println!("{:?}", water);

    // Define transition
    let mut water_synthesis = Transition::new("Water synthesis");
    water_synthesis.add_input(&mut hydrogen, 2); // Consumes 1 hydrogen
    water_synthesis.add_input(&mut oxygen, 1); // Consumes 2 oxygen
    water_synthesis.add_output(&mut water, 2); // Produces 2 water molecules

    // Fire the transition (simulate a step in the metabolic pathway)
    match water_synthesis.fire() {
        Ok(()) => {
            println!("\nAfter firing the transition ({}):", water_synthesis.name);
            println!("{:?}", hydrogen);
            println!("{:?}", oxygen);
            println!("{:?}", water);
        }
        Err(err) => println!("Failed to fire transition: {}", err),
    }

    let duration = start.elapsed();
    let seconds = duration.as_secs_f64(); 
    // Print the elapsed time
    println!("Time taken: {:6}", seconds);
}
