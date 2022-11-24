use std::collections::HashMap;

use serde::Deserialize;

pub fn hasmap_contains(
    prev_state: &HashMap<String, bool>,
    pre_state: &HashMap<String, bool>,
) -> bool {
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

#[derive(Debug)]
pub struct Gnode {
    pub id: String,
    pub from_node: Option<usize>,
    pub state: HashMap<String, bool>,
    pub running_cost: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Gaction {
    pub cost: usize,
    pub pre_state: HashMap<String, bool>,
    pub post_state: HashMap<String, bool>,
}

impl Gaction {
    pub fn are_preconditions_met(&self, prev_state: &HashMap<String, bool>) -> bool {
        hasmap_contains(prev_state, &self.pre_state)
    }

    pub fn update_with_post_conditions(&self, prev_state: &mut HashMap<String, bool>) {
        for (state_key, value) in self.post_state.clone() {
            prev_state.insert(state_key, value);
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Goal {
    pub name: String,
    pub state: HashMap<String, bool>,
}

#[derive(Debug, Deserialize)]
pub struct PlanConfig {
    pub actions: HashMap<String, Gaction>,
    pub goals: Vec<Goal>,
    pub worldState: HashMap<String, bool>,
}
