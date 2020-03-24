pub const BOT_GITHUB_LOGIN: &str = "parity-processbot";
pub const AUTO_MERGE_REQUEST: &str = "bot merge";
pub const AUTO_MERGE_REQUEST_CANCELLED: &str = "bot merge cancel";
pub const AUTO_MERGE_REQUEST_COMPLETE: &str = "bot merge complete";

pub const COMPARE_RELEASE_REQUEST: &str = "bot compare release";
pub const COMPARE_RELEASE_REPLY: &str = "Compare latest release:";

pub const FEATURES_KEY: &str = "features";

pub const PROJECT_NEEDS_BACKLOG: &str =
	"@{owner}, {project_url} needs a backlog column.";

pub const MISMATCHED_PROCESS_FILE: &str = "Process.toml for repo {repo_url} lists projects that do not exist in the repo, so it will be treated as invalid.";

pub const MALFORMED_PROCESS_FILE: &str = "Process.toml for repo {repo_url} is malformed or missing some fields. Please ensure that every listed project contains an owner and a matrix_room_id.";

pub const CLOSE_FOR_NO_ISSUE: &str = "@{author}, this pull request is being closed because it does not explicitly address an issue.";

pub const WILL_CLOSE_FOR_NO_PROJECT: &str =
	"@{author}, {issue_url} will be closed if it is not attached to a project.";

pub const PRIVATE_ISSUE_NEEDS_REASSIGNMENT: &str = "{pr_url} addressing {issue_url} has been opened by {author}. Please reassign the issue or close the pull request.";

pub const PUBLIC_ISSUE_NEEDS_REASSIGNMENT: &str = "@{owner}, {pr_url} addressing {issue_url} has been opened by {author}. Please reassign the issue or close the pull request.";

pub const PROJECT_CONFIRMATION: &str = "{issue_url} has been attached to the project column '{column_name}' in project '{project_name}'. To confirm the change, {owner} or a whitelisted developer should post, \"confirm {issue_id} {column_id}\", to this channel in the next {seconds} seconds.";

pub const ISSUE_REVERT_PROJECT_NOTIFICATION: &str = "The change you made to {issue_url} (attaching a project) has been denied or gone unconfirmed for too long, and so has been reverted. Changes require confirmation from the project owner or a whitelisted developer.";

pub const REQUESTING_REVIEWS_MESSAGE: &str =
	"@{author}, {pr_url} needs reviewers.";

pub const STATUS_FAILURE_NOTIFICATION: &str =
	"{1} failed status checks or is not mergeable.";

pub const REQUEST_DELEGATED_REVIEW_MESSAGE: &str = "{1} needs your review in the next 72 hours, as you are the owner or delegated reviewer.";

pub const PRIVATE_REVIEW_REMINDER_MESSAGE: &str = "{1} needs your review.";

pub const PUBLIC_REVIEW_REMINDER_MESSAGE: &str = "@{2}, please review {1}.";

pub const CORE_SORTING_REPO: &str = "core-sorting";

pub const BACKLOG_DEFAULT_NAME: &str = "backlog";

pub const PROCESS_FILE_NAME: &str = "Process.toml";

pub const LOCAL_STATE_KEY: &str = "LOCAL_STATE_KEY";
