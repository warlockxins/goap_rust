use std::collections::HashMap;

// use axum::response::Response;
use axum::{extract, Json};
use axum::{http::StatusCode, response::IntoResponse};
use goap_runner::PlanConfig;
use goap_runner::{Finder, Gaction, Goal};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct ParameterValue {
    pub parameter: String,
    pub value: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BodyAction {
    pub name: String,
    pub cost: usize,
    pub pre_state: Vec<ParameterValue>,
    pub post_state: Vec<ParameterValue>,
}

#[derive(Debug, Deserialize)]
pub struct BodyGoal {
    pub name: String,
    pub state: Vec<ParameterValue>,
}

#[derive(Debug, Deserialize)]
pub struct PlanBody {
    pub actions: Vec<BodyAction>,
    pub goals: Vec<BodyGoal>,
    pub worldState: Vec<ParameterValue>,
}

fn body_state_to_hash_map(p: &Vec<ParameterValue>) -> HashMap<String, bool> {
    let mut h: HashMap<String, bool> = HashMap::new();

    for ParameterValue { parameter, value } in p {
        h.insert(parameter.clone(), *value);
    }

    return h;
}

pub fn to_plan_config(plan_config: PlanBody) -> Result<PlanConfig, String> {
    let PlanBody {
        actions,
        goals,
        worldState,
    } = plan_config;

    // exxtract action map
    let mut plan_actions: HashMap<String, Gaction> = HashMap::new();

    for a in actions {
        let plan_action: Gaction = Gaction {
            cost: a.cost,
            pre_state: body_state_to_hash_map(&a.pre_state),
            post_state: body_state_to_hash_map(&a.post_state),
        };

        plan_actions.insert(a.name, plan_action);
    }
    // extract goals
    let mut plan_goal_list: Vec<Goal> = vec![];

    for g in goals {
        let plan_goal = Goal {
            name: g.name,
            state: body_state_to_hash_map(&g.state),
        };

        plan_goal_list.push(plan_goal);
    }

    // extract world state
    let plan_world_state: HashMap<String, bool> = body_state_to_hash_map(&worldState);

    // minor error/empty state check to show Errors on Client later
    if plan_world_state.is_empty() || plan_goal_list.is_empty() || plan_actions.is_empty() {
        Err("Actions/Goals/World State must not be empty".to_string())
    } else {
        Ok(PlanConfig {
            actions: plan_actions,
            goals: plan_goal_list,
            worldState: plan_world_state,
        })
    }
}

#[derive(Serialize)]
pub struct ListActionNames {
    pub action_names: Vec<String>,
}

pub async fn goap_run(
    extract::Json(payload): extract::Json<PlanBody>, // Important, need to EXTRACT
) -> Result<impl IntoResponse, (StatusCode, String)> {
    match to_plan_config(payload) {
        Ok(config) => {
            let mut finder = Finder::new(&config);
            let leaves = finder.execute();

            Ok(Json(ListActionNames {
                action_names: leaves,
            }))
        }
        Err(reason) => Err((StatusCode::BAD_REQUEST, reason)),
    }
}
