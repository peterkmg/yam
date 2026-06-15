use xml_3dm::{
  BaseNodeFactory,
  BranchNodeFactory,
  Merge,
  NodeRef,
  TriMatching,
  XmlParser,
  XmlPrinter,
};

use crate::{
  MergeError,
  markers::marker_block,
  merger::{MergeInput, MergeResult},
};

pub fn merge(input: MergeInput<'_>) -> Result<MergeResult, MergeError> {
  tracing::trace!("running 3dm xml merge");
  let base_parser = XmlParser::new(BaseNodeFactory);
  let branch_parser = XmlParser::new(BranchNodeFactory);

  let base = base_parser.parse_str(input.base)?;
  let ours = branch_parser.parse_str(input.ours)?;
  let theirs = branch_parser.parse_str(input.theirs)?;

  let matching = TriMatching::new(ours, base, theirs);
  let mut merger = Merge::new(matching);
  let tree = merger.merge_to_tree();
  let mut content = render_document(&tree)?;
  let conflict_count = merger.conflict_log.conflict_count();
  tracing::debug!(conflict_count, "3dm xml merge completed");

  if conflict_count > 0 {
    let fragments = merger
      .conflict_log
      .conflicts()
      .iter()
      .map(|conflict| {
        Ok(ConflictFragments {
          selected: selected_conflict_side(&conflict.text),
          ours: render_optional_fragment(conflict.branch1.as_ref())?,
          base: render_optional_fragment(conflict.base.as_ref())?,
          theirs: render_optional_fragment(conflict.branch2.as_ref())?,
        })
      })
      .collect::<Result<Vec<_>, MergeError>>()?;

    if let Some(rendered) = render_xml_conflicts(content, &fragments) {
      content = rendered;
    } else {
      tracing::warn!(
        conflict_count,
        "xml conflict fragments could not be inserted; using whole-file conflict"
      );
      content = whole_file_conflict(input);
    }
  }

  Ok(MergeResult::new(content, conflict_count > 0))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConflictSide {
  Ours,
  Theirs,
}

#[derive(Debug)]
struct ConflictFragments {
  selected: ConflictSide,
  ours: String,
  base: String,
  theirs: String,
}

impl ConflictFragments {
  fn selected_fragment(&self) -> &str {
    match self.selected {
      ConflictSide::Ours => &self.ours,
      ConflictSide::Theirs => &self.theirs,
    }
  }
}

fn selected_conflict_side(text: &str) -> ConflictSide {
  if text.contains("using branch 2") {
    ConflictSide::Theirs
  } else {
    ConflictSide::Ours
  }
}

fn render_document(node: &NodeRef) -> Result<String, MergeError> {
  let mut output = Vec::new();
  {
    let mut printer = XmlPrinter::new(&mut output);
    printer.print(node)?;
  }
  Ok(String::from_utf8(output)?)
}

fn render_optional_fragment(node: Option<&NodeRef>) -> Result<String, MergeError> {
  let Some(node) = node else {
    return Ok(String::new());
  };

  let mut output = Vec::new();
  {
    let mut printer = XmlPrinter::new(&mut output);
    printer.print_fragment(node)?;
  }
  Ok(String::from_utf8(output)?)
}

fn render_xml_conflicts(mut content: String, conflicts: &[ConflictFragments]) -> Option<String> {
  for conflict in conflicts {
    let selected_fragment = conflict.selected_fragment();
    if selected_fragment.is_empty() || content.matches(selected_fragment).count() != 1 {
      return None;
    }

    content = content.replacen(
      selected_fragment,
      &marker_block(&conflict.ours, &conflict.base, &conflict.theirs),
      1,
    );
  }

  Some(content)
}

fn whole_file_conflict(input: MergeInput<'_>) -> String {
  marker_block(input.ours, input.base, input.theirs)
}
