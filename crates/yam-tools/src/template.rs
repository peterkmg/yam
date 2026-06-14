use crate::{ToolError, ToolKind};

pub struct TemplateValue<'a> {
  pub key: &'static str,
  pub value: &'a str,
}

pub fn expand_arguments(
  tool: ToolKind,
  args: &[String],
  values: &[TemplateValue<'_>],
) -> Result<Vec<String>, ToolError> {
  args
    .iter()
    .map(|arg| expand_argument(tool, arg, values))
    .collect()
}

fn expand_argument(
  tool: ToolKind,
  arg: &str,
  values: &[TemplateValue<'_>],
) -> Result<String, ToolError> {
  let mut expanded = arg.to_string();

  for value in values {
    expanded = expanded.replace(&format!("{{{}}}", value.key), value.value);
  }

  // check for unexpanded placeholders
  if expanded.contains('{') && expanded.contains('}') {
    return Err(ToolError::UnknownTemplatePlaceholder {
      tool,
      argument: arg.to_string(),
    });
  }

  Ok(expanded)
}
