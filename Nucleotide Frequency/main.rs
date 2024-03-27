use std::io::{self, Write};

struct Table {
    pub a: u8,
    pub c: u8,
    pub g: u8,
    pub t: u8,
}

fn main() {
    println!("Please enter the nucleotide sequence:");
    let mut sequence = String::new();
    io::stdout().flush().unwrap(); // Flush the stdout
    io::stdin().read_line(&mut sequence).expect("Failed to read line");

    let freq_table: Table = frequency_table(&sequence);
    println!(
        "Frequency table -> A:{} | C:{} | G:{} | T:{}",
        freq_table.a, freq_table.c, freq_table.g, freq_table.t
    );
}

fn frequency_table(sequence: &String) -> Table {
    let mut freq_table: Table = Table {
        // Create a new struct
        a: 0,
        c: 0,
        g: 0,
        t: 0,
    };
    for i in sequence.trim().chars() {
        // Iterate through the sequence after trimming whitespace
        match i {
            'A' => freq_table.a += 1,
            'C' => freq_table.c += 1,
            'G' => freq_table.g += 1,
            'T' => freq_table.t += 1,
            _ => println!("Invalid character"),
        }
    }

    freq_table
}
