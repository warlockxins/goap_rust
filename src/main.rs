use serde::Deserialize;
use serde_json::{Result, Value};

use std::collections::HashMap;
use std::fs;
use std::time::Instant;

#[derive(Debug)]
struct Gnode {
    id: String,
    from_node: Option<usize>,
    state: HashMap<String, bool>,
    running_cost: usize,
}

fn hasmap_contains(prev_state: &HashMap<String, bool>, pre_state: &HashMap<String, bool>) -> bool {
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

#[derive(Debug, Deserialize, Clone)]
struct Gaction {
    cost: usize,
    pre_state: HashMap<String, bool>,
    post_state: HashMap<String, bool>,
}

impl Gaction {
    fn are_preconditions_met(&self, prev_state: &HashMap<String, bool>) -> bool {
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
    match get_setup() {
        Ok(config) => {
            let start = Instant::now();
            let leaves = execute(config);
            let duration = start.elapsed();

            println!("elapsed {:?}", duration);
        }
        Err(e) => println!("error parsing config: {:?}", e),
    }
}

fn execute(config: PlanConfig) -> Vec<usize> {
    let g = Gnode {
        id: String::from("start"),
        from_node: None,
        state: config.worldState,
        running_cost: 0,
    };

    let mut nodes = vec![g];

    let mut cheapest: usize = 10000;
    let leaves = build_graph(0, &config.goals, &config.actions, &mut nodes, &mut cheapest);

    for l in &leaves {
        let mut idx_option = Some(*l);
        println!(
            "___________________________ cost {}",
            nodes[*l].running_cost
        );
        while let Some(idx) = idx_option {
            println!("-> {}", nodes[idx].id);
            idx_option = nodes[idx].from_node;
        }
    }
    return leaves;
}

fn build_graph<'a>(
    start_node_index: usize,
    available_goals: &Vec<Goal>,
    available_actions: &HashMap<String, Gaction>,
    nodes: &mut Vec<Gnode>,
    cheapest: &mut usize,
) -> Vec<usize> {
    let mut leaves: Vec<usize> = vec![];
    let start_node_option = nodes.get(start_node_index);

    if start_node_option.is_none() {
        return vec![];
    }

    let start_node = start_node_option.unwrap();

    let next_state_base = start_node.state.clone();
    let running_cost = start_node.running_cost;

    for (key, action) in available_actions {
        let mut next_state = next_state_base.clone();
        let has_preconditions = action.are_preconditions_met(&next_state);

        if has_preconditions {
            action.update_with_post_conditions(&mut next_state);

            let mut cost: usize = running_cost + action.cost;

            let next_node: Gnode = Gnode {
                id: String::from(key),
                from_node: Some(start_node_index),
                state: next_state.clone(),
                running_cost: cost,
            };

            nodes.push(next_node);

            let matching_goal = available_goals
                .into_iter()
                .find(|&g| hasmap_contains(&next_state, &g.state));

            if let Some(_g) = matching_goal {
                if (cheapest > &mut cost) {
                    let existing_size = nodes.len() - 1;
                    leaves.push(existing_size);

                    *cheapest = cost;
                    println!("now seting {}, cost {}", key, cheapest);
                }
            } else {
                let mut next_available_actions: HashMap<String, Gaction> =
                    available_actions.clone();

                next_available_actions.remove(key);

                let mut leaves_internal = build_graph(
                    nodes.len() - 1,
                    available_goals,
                    &next_available_actions,
                    nodes,
                    cheapest,
                );

                leaves.append(&mut leaves_internal);
            }
        }
    }

    return leaves;
}
