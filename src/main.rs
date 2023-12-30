use clap::{Arg, Command};
use rand::Rng;
use std::{thread, time::Duration};

const EMPTY_CHAR: char = ' ';
const FILLED_CHAR: char = '█';

fn main() {
    let matches = Command::new("Terminaltomata")
        .author("Dib")
        .about("A simple command line tool for elementary cellular automata")
        .arg(
            Arg::new("rule")
                .short('r')
                .long("rule")
                .help("Rule to use for the automaton generation. The default is a random choice between 0-255")
        )
        .arg(
            Arg::new("num-iterations")
                .short('n')
                .long("num-iterations")
                .help("Number of iterations to generate. The default is infinite.")
        )
        .arg(
            Arg::new("gen-delay")
                .short('d')
                .long("gen-delay")
                .help("Delay (ms) between each generation. The default is 10ms.")
        )
        .arg(
            Arg::new("width")
                .short('w')
                .long("width")
                .help("Width of the automaton generation. The default is the width of your terminal")
        )
        .get_matches();

    let rule = matches
        .get_one::<String>("rule")
        .map(|s| s.parse::<usize>().expect("Could not parse rule into int"))
        .unwrap_or_else(|| rand::thread_rng().gen_range(0..256));

    let num_iterations = matches
        .get_one::<String>("num-iterations")
        .and_then(|s| s.parse::<usize>().ok());

    let gen_delay = matches
        .get_one::<String>("gen-delay")
        .map(|s| s.parse::<u64>().expect("Could not parse generation delay"))
        .unwrap_or(10);

    let width = matches
        .get_one::<String>("width")
        .map(|s| {
            s.parse::<usize>()
                .expect("Could not parse generation delay")
        })
        .unwrap_or(termsize::get().unwrap().cols as usize);

    let mut cells = vec![false; width];
    let mut temp_cells = cells.clone();
    cells[width / 2] = true;

    let mut iteration_cnt = 0;
    loop {
        display_generation(&cells);

        for (idx, neighbourhood) in wrap_around(&cells)
            .windows(3)
            .map(generate_num_repr)
            .enumerate()
        {
            temp_cells[idx] = generate_next(&rule, neighbourhood);
        }

        if let Some(num_iterations) = num_iterations {
            iteration_cnt += 1;
            if iteration_cnt >= num_iterations {
                break;
            }
        }
        std::mem::swap(&mut cells, &mut temp_cells);
        thread::sleep(Duration::from_millis(gen_delay))
    }
}

fn generate_num_repr(bits: &[bool]) -> usize {
    let mut num = 0;
    for &bit in bits.iter() {
        num <<= 1;
        if bit {
            num |= 1;
        }
    }
    num
}

fn generate_next(rule: &usize, neighbourhood: usize) -> bool {
    ((rule >> neighbourhood) & 1) == 1
}

fn wrap_around(cells: &[bool]) -> Vec<bool> {
    let last_element = *cells.last().unwrap();
    let first_element = *cells.first().unwrap();

    // Create a new vector and append elements
    let mut new_vec = vec![last_element];
    new_vec.extend_from_slice(cells);
    new_vec.push(first_element);

    new_vec
}

fn display_generation(cells: &Vec<bool>) {
    for cell in cells {
        print!(
            "{}",
            match cell {
                true => FILLED_CHAR,
                false => EMPTY_CHAR,
            }
        )
    }
    println!();
}
