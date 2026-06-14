use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use yam_tools::{CommandRunner, CommandSpec, ToolError, ToolRun};

#[derive(Debug, Default, Clone)]
pub struct RunnerSpy {
  inner: Rc<RunnerState>,
}

#[derive(Debug, Default)]
struct RunnerState {
  outputs: RefCell<VecDeque<ToolRun>>,
  commands: RefCell<Vec<CommandSpec>>,
}

impl RunnerSpy {
  pub fn new(outputs: Vec<ToolRun>) -> Self {
    Self {
      inner: Rc::new(RunnerState {
        outputs: RefCell::new(VecDeque::from(outputs)),
        commands: RefCell::new(Vec::new()),
      }),
    }
  }

  pub fn commands(&self) -> Vec<CommandSpec> {
    self.inner.commands.borrow().clone()
  }
}

impl CommandRunner for RunnerSpy {
  fn run(&self, command: &CommandSpec) -> Result<ToolRun, ToolError> {
    self.inner.commands.borrow_mut().push(command.clone());
    Ok(
      self
        .inner
        .outputs
        .borrow_mut()
        .pop_front()
        .expect("runner output should be queued"),
    )
  }
}

pub fn success(stdout: impl Into<String>, stderr: impl Into<String>) -> ToolRun {
  ToolRun {
    status_code: Some(0),
    stdout: stdout.into(),
    stderr: stderr.into(),
  }
}
