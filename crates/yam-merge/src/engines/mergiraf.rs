use std::{borrow::Cow, sync::Arc, time::Duration};

use mergiraf::{lang_profile::LangProfile, merge::cascading_merge, settings::DisplaySettings};

use crate::{
  MergeError,
  merger::{MergeInput, MergeResult},
};

const WITCHER_SCRIPT_LANGUAGE: &str = "witcherscript";
const MERGE_TIMEOUT: Duration = Duration::from_secs(10);

pub fn merge(input: MergeInput<'_>) -> Result<MergeResult, MergeError> {
  let Some(profile) = LangProfile::find_by_name(WITCHER_SCRIPT_LANGUAGE) else {
    return Err(MergeError::MergirafLanguage(WITCHER_SCRIPT_LANGUAGE));
  };

  let result = cascading_merge(
    Arc::new(Cow::Owned(input.base.to_owned())),
    Arc::new(Cow::Owned(input.ours.to_owned())),
    Arc::new(Cow::Owned(input.theirs.to_owned())),
    Arc::new(Cow::Borrowed(profile)),
    DisplaySettings::new(
      Some(false),
      Some(7),
      Some(Cow::Borrowed("base")),
      Some(Cow::Borrowed("ours")),
      Some(Cow::Borrowed("theirs")),
    ),
    true,
    None,
    MERGE_TIMEOUT,
  )
  .into_iter()
  .min_by_key(|merge| (merge.has_additional_issues, merge.conflict_mass))
  .expect("Mergiraf always returns at least the line-based merge");

  if result.has_additional_issues && result.conflict_count == 0 {
    return Err(MergeError::UnmarkedMergeIssues);
  }

  Ok(MergeResult::new(result.contents, result.conflict_count > 0))
}
