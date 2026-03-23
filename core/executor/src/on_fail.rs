//! on_fail handler: retry / skip / rollback / collect_and_abort / escalate_plan

use crate::plan::{OnFail, OnFailAction};

#[derive(Debug)]
pub enum OnFailDecision {
    Retry { attempts_left: u32 },
    Skip,
    Rollback { to: Option<String> },
    Abort { collected: Vec<String> },
    Escalate { hint: Option<String> },
}

pub fn decide(on_fail: &OnFail, attempt: u32) -> OnFailDecision {
    let max = on_fail.max_attempts.unwrap_or(1);
    match &on_fail.action {
        OnFailAction::Retry => {
            if attempt < max {
                OnFailDecision::Retry { attempts_left: max - attempt }
            } else {
                OnFailDecision::Abort { collected: on_fail.collect.clone() }
            }
        }
        OnFailAction::Skip => OnFailDecision::Skip,
        OnFailAction::Rollback => OnFailDecision::Rollback {
            to: on_fail.rollback_to.clone(),
        },
        OnFailAction::CollectAndAbort => OnFailDecision::Abort {
            collected: on_fail.collect.clone(),
        },
        OnFailAction::EscalatePlan => OnFailDecision::Escalate {
            hint: on_fail.escalate_hint.clone(),
        },
    }
}
