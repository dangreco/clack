use crate::state::State;

#[derive(Debug, Clone, Copy)]
pub struct Task {
  pub state: State,
  pub priority: u8,
  pub id: u32,
  pub stack_size: u32,
  pub stack_addr: u32,
}
