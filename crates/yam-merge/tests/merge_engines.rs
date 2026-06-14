#![allow(unused_crate_dependencies)]

use yam_merge::{MergeError, MergeInput, MergeableFileType, merge};

const WITCHER_BASE: &str = r"
function Existing() : int
{
  return 1;
}
";

const WITCHER_OURS_ADDS_FUNCTION: &str = r"
function Existing() : int
{
  return 1;
}

function AddedByOurs() : int
{
  return 2;
}
";

const WITCHER_THEIRS_ADDS_FUNCTION: &str = r"
function Existing() : int
{
  return 1;
}

function AddedByTheirs() : int
{
  return 3;
}
";

const WITCHER_OURS_EDITS_FUNCTION: &str = r"
function Existing() : int
{
  return 2;
}
";

const WITCHER_THEIRS_EDITS_FUNCTION: &str = r"
function Existing() : int
{
  return 3;
}
";

const WITCHER_INDEPENDENT_ADDITIONS: MergeInput<'static> = MergeInput::new(
  WITCHER_BASE,
  WITCHER_OURS_ADDS_FUNCTION,
  WITCHER_THEIRS_ADDS_FUNCTION,
);

const WITCHER_SAME_FUNCTION_EDITS: MergeInput<'static> = MergeInput::new(
  WITCHER_BASE,
  WITCHER_OURS_EDITS_FUNCTION,
  WITCHER_THEIRS_EDITS_FUNCTION,
);

const WITCHER_UNPARSEABLE_BASE: &str = r"
function Broken() : int
{
  return 1;
";

const WITCHER_UNPARSEABLE_OURS: &str = r"
function Broken() : int
{
  return 1;
";

const WITCHER_UNPARSEABLE_THEIRS: &str = r"
function Broken() : int
{
  return 1;
";

const WITCHER_UNPARSEABLE_MERGE: MergeInput<'static> = MergeInput::new(
  WITCHER_UNPARSEABLE_BASE,
  WITCHER_UNPARSEABLE_OURS,
  WITCHER_UNPARSEABLE_THEIRS,
);

const XML_BASE: &str = r#"<items><entry id="a" value="one"/><entry id="b" value="two"/></items>"#;
const XML_OURS_REORDERS_AND_ADDS: &str = r#"<items><entry id="b" value="two"/><entry id="a" value="one"/><entry id="c" value="three"/></items>"#;
const XML_THEIRS_EDITS_ATTRIBUTE: &str =
  r#"<items><entry id="a" value="one"/><entry id="b" value="two-updated"/></items>"#;

const XML_INDEPENDENT_EDITS: MergeInput<'static> = MergeInput::new(
  XML_BASE,
  XML_OURS_REORDERS_AND_ADDS,
  XML_THEIRS_EDITS_ATTRIBUTE,
);

const XML_CONFLICTING_BASE: &str = r#"<items><entry id="a" value="one"/></items>"#;
const XML_CONFLICTING_OURS: &str = r#"<items><entry id="a" value="ours"/></items>"#;
const XML_CONFLICTING_THEIRS: &str = r#"<items><entry id="a" value="theirs"/></items>"#;

const XML_CONFLICTING_ATTRIBUTE_EDITS: MergeInput<'static> = MergeInput::new(
  XML_CONFLICTING_BASE,
  XML_CONFLICTING_OURS,
  XML_CONFLICTING_THEIRS,
);

#[test]
fn mergiraf_merges_independent_witcher_script_functions() {
  let outcome = merge(
    MergeableFileType::WitcherScript,
    WITCHER_INDEPENDENT_ADDITIONS,
  )
  .expect("merge should run");

  assert!(outcome.is_clean());
  assert!(outcome.text.contains("function AddedByOurs()"));
  assert!(outcome.text.contains("function AddedByTheirs()"));
  assert!(!outcome.text.contains("<<<<<<<"));

  let ours_position = outcome
    .text
    .find("function AddedByOurs()")
    .expect("ours function should be present");
  let theirs_position = outcome
    .text
    .find("function AddedByTheirs()")
    .expect("theirs function should be present");
  assert!(
    ours_position < theirs_position,
    "Mergiraf should keep left additions before right additions"
  );
}

#[test]
fn repeated_witcher_script_merges_are_stable() {
  let first = merge(
    MergeableFileType::WitcherScript,
    WITCHER_INDEPENDENT_ADDITIONS,
  )
  .expect("first merge should run");
  let second = merge(
    MergeableFileType::WitcherScript,
    WITCHER_INDEPENDENT_ADDITIONS,
  )
  .expect("second merge should run");

  assert!(first.is_clean());
  assert!(second.is_clean());
  assert_eq!(first.text, second.text);
}

#[test]
fn mergiraf_reports_same_function_conflict() {
  let outcome = merge(
    MergeableFileType::WitcherScript,
    WITCHER_SAME_FUNCTION_EDITS,
  )
  .expect("merge should run");

  assert!(!outcome.is_clean());
  assert_eq!(outcome.conflict_count(), 1);
  assert!(outcome.text.contains("<<<<<<<"));
  assert!(outcome.text.contains("return 2;"));
  assert!(outcome.text.contains("return 3;"));
}

#[test]
fn mergiraf_rejects_unmarked_additional_issues() {
  let error = merge(MergeableFileType::WitcherScript, WITCHER_UNPARSEABLE_MERGE)
    .expect_err("unmarked merge issues should be rejected");

  assert!(matches!(error, MergeError::UnmarkedMergeIssues));
}

#[test]
fn xml_merge_combines_out_of_order_addition_and_attribute_change() {
  let outcome = merge(MergeableFileType::Xml, XML_INDEPENDENT_EDITS).expect("merge should run");

  assert!(outcome.is_clean());
  assert!(outcome.text.contains(r#"id="c""#));
  assert!(outcome.text.contains(r#"value="two-updated""#));
}

#[test]
fn xml_merge_reports_conflicting_attribute_changes() {
  let outcome =
    merge(MergeableFileType::Xml, XML_CONFLICTING_ATTRIBUTE_EDITS).expect("merge should run");

  assert!(!outcome.is_clean());
  assert!(outcome.conflict_count() > 0);
  assert!(outcome.text.contains("<<<<<<< ours"));
  assert!(outcome.text.contains("||||||| base"));
  assert!(outcome.text.contains("======="));
  assert!(outcome.text.contains(">>>>>>> theirs"));
  assert!(outcome.text.contains(r#"value="ours""#));
  assert!(outcome.text.contains(r#"value="theirs""#));
}
