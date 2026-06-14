pub fn marker_block(ours: &str, base: &str, theirs: &str) -> String {
  let mut output = String::new();
  output.push_str("<<<<<<< ours\n");
  push_marker_section(&mut output, ours);
  output.push_str("||||||| base\n");
  push_marker_section(&mut output, base);
  output.push_str("=======\n");
  push_marker_section(&mut output, theirs);
  output.push_str(">>>>>>> theirs");
  output
}

fn push_marker_section(output: &mut String, content: &str) {
  output.push_str(content.trim_end_matches(['\r', '\n']));
  output.push('\n');
}
