//! Budget enforcement: tracks step count, wall time, retries, output size.

use std::time::Instant;
use crate::plan::Budgets;

#[derive(Debug)]
pub struct BudgetTracker {
    start: Instant,
    budgets: Budgets,
    pub steps_run: usize,
    pub retries_used: usize,
    pub output_kb_used: usize,
    pub cloud_rounds_used: u32,
}

#[derive(Debug, thiserror::Error)]
pub enum BudgetError {
    #[error("Step budget exceeded: {steps} >= {max}")]
    TooManySteps { steps: usize, max: usize },
    #[error("Wall time budget exceeded: {elapsed_s}s >= {max}s")]
    WallTimeExceeded { elapsed_s: u64, max: u64 },
    #[error("Retry budget exceeded: {retries} >= {max}")]
    TooManyRetries { retries: usize, max: usize },
    #[error("Output size budget exceeded: {kb}KB >= {max}KB")]
    OutputTooBig { kb: usize, max: usize },
    #[error("Cloud Round budget exceeded: {rounds} >= {max}")]
    TooManyCloudRounds { rounds: u32, max: u32 },
}

impl BudgetTracker {
    pub fn new(budgets: Budgets) -> Self {
        Self { start: Instant::now(), budgets, steps_run: 0, retries_used: 0, output_kb_used: 0, cloud_rounds_used: 0 }
    }

    pub fn check_step(&mut self) -> Result<(), BudgetError> {
        self.steps_run += 1;
        if self.steps_run > self.budgets.max_steps {
            return Err(BudgetError::TooManySteps {
                steps: self.steps_run, max: self.budgets.max_steps,
            });
        }
        let elapsed = self.start.elapsed().as_secs();
        if elapsed >= self.budgets.max_wall_time_s {
            return Err(BudgetError::WallTimeExceeded {
                elapsed_s: elapsed, max: self.budgets.max_wall_time_s,
            });
        }
        Ok(())
    }

    pub fn record_retry(&mut self) -> Result<(), BudgetError> {
        self.retries_used += 1;
        if self.retries_used > self.budgets.max_retries_total {
            return Err(BudgetError::TooManyRetries {
                retries: self.retries_used, max: self.budgets.max_retries_total,
            });
        }
        Ok(())
    }

    pub fn record_output(&mut self, bytes: usize) -> Result<(), BudgetError> {
        let kb = bytes / 1024;
        self.output_kb_used += kb;
        if self.output_kb_used > self.budgets.max_cmd_output_kb {
            return Err(BudgetError::OutputTooBig {
                kb: self.output_kb_used, max: self.budgets.max_cmd_output_kb,
            });
        }
        Ok(())
    }

    pub fn record_cloud_round(&mut self) -> Result<(), BudgetError> {
        self.cloud_rounds_used += 1;
        if self.cloud_rounds_used > self.budgets.max_cloud_rounds {
            return Err(BudgetError::TooManyCloudRounds {
                rounds: self.cloud_rounds_used, max: self.budgets.max_cloud_rounds,
            });
        }
        Ok(())
    }

    pub fn elapsed_s(&self) -> u64 { self.start.elapsed().as_secs() }
}
