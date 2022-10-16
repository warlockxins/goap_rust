mod configs;
use configs::PlanConfig;

mod finder;
use finder::Finder;

use serde_json::Result;

use std::fs;
use std::time::Instant;

fn get_setup() -> Result<PlanConfig> {
    let contents =
        fs::read_to_string("actions.json").expect("Something went wrong reading the file");

    let v: PlanConfig = serde_json::from_str(&contents)?;
    Ok(v)
}

fn main() {
    match get_setup() {
        Ok(config) => {
            let start = Instant::now();
            let mut finder = Finder::new(&config);
            let leaves = finder.execute();
            let duration = start.elapsed();

            println!("actions: {:?}", leaves);
            println!("elapsed {:?}", duration);
        }
        Err(e) => println!("error parsing config: {:?}", e),
    }
}
