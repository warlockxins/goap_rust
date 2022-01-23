use serde::Deserialize;
use serde_json::{Result, Value};

use std::collections::HashMap;
use std::fs;
use std::time::Instant;

#[derive(Debug)]
struct Gnode {
    from_node: Box<Option<Gnode>>,
    state: HashMap<String, bool>,
    running_cost: i32,
}

fn hasmap_contains(prev_state: HashMap<String, bool>, pre_state: &HashMap<String, bool>) -> bool {
    for (state_key, value) in pre_state {
        if let Some(existing_value) = prev_state.get(state_key) {
            if existing_value != value {
                return false;
            }
        } else {
            return false;
        }
    }

    return true;
}

#[derive(Debug, Deserialize)]
struct Gaction {
    cost: i32,
    pre_state: HashMap<String, bool>,
    post_state: HashMap<String, bool>,
}

impl Gaction {
    fn are_preconditions_met(&self, prev_state: HashMap<String, bool>) -> bool {
        hasmap_contains(prev_state, &self.pre_state)
    }

    fn update_with_post_conditions(&self, prev_state: &mut HashMap<String, bool>) {
        for (state_key, value) in self.post_state.clone() {
            prev_state.insert(state_key, value);
        }
    }
}

#[derive(Debug, Deserialize)]
struct Goal {
    name: String,
    state: HashMap<String, bool>,
}

#[derive(Debug, Deserialize)]
struct PlanConfig {
    actions: HashMap<String, Gaction>,
    goals: Vec<Goal>,
    worldState: HashMap<String, bool>,
}

fn get_setup() -> Result<(PlanConfig)> {
    let contents =
        fs::read_to_string("actions.json").expect("Something went wrong reading the file");

    let v: PlanConfig = serde_json::from_str(&contents)?;
    Ok(v)
}

fn main() {
    let start = Instant::now();
    match get_setup() {
        Ok(config) => {
            execute(config);
        }
        Err(e) => println!("error parsing config: {:?}", e),
    }

    let mut world_state: HashMap<String, bool> = HashMap::new();
    println!("world is: {:?}", world_state);

    let mut pre_state: HashMap<String, bool> = HashMap::new();

    let mut post_state = pre_state.clone();

    let ga = Gaction {
        pre_state: pre_state,
        post_state: post_state,
        cost: 1,
    };

    let passed_check = ga.are_preconditions_met(world_state.clone());

    ga.update_with_post_conditions(&mut world_state);
    println!("new world state {:?}", world_state);

    let duration = start.elapsed();
    println!("elapsed {:?}", duration);
}

fn execute(config: PlanConfig) {
    let mut leaves: Vec<Gnode> = vec![];

    let g = Gnode {
        from_node: Box::new(None),
        state: config.worldState,
        running_cost: 0,
    };

    buildGraph(&g, &config.goals, &config.actions, &mut leaves);
}

fn buildGraph(
    startNode: &Gnode,
    available_goals: &Vec<Goal>,
    available_actions: &HashMap<String, Gaction>,
    leaves: &mut Vec<Gnode>,
) {
    for (actionName, _action) in available_actions {
        println!("{}", actionName);
    }
}
