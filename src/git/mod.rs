use git2::{self, Repository, Branch};
use git2::{RepositoryState};

#[derive(Debug)]
pub struct Status {
    pub ahead: usize,
    pub behind: usize,
    pub new_files: usize,
    pub working_files: usize,
    pub index_files: usize,
    pub stash_count: usize,
    pub state: RepositoryState,
    pub hash: String,
    pub branch: String,
    pub tag: String,
}

impl Status {
    pub fn from_cwd() -> Result<Status, git2::Error> {
        match Repository::discover(".") {
            Ok(repo) => Ok(Status::from_repo(repo)),
            Err(err) => Err(err),
        }
    }

    fn from_repo(mut repo: Repository) -> Status {
        let mut status = Status {
            ahead: 0,
            behind: 0,
            new_files: 0,
            working_files: 0,
            index_files: 0,
            stash_count: 0,
            state: repo.state(),
            hash: "".to_string(),
            branch: "".to_string(),
            tag: "".to_string(),
        };
        if let Ok(raw_statuses) = repo.statuses(None) {
            for raw_status in raw_statuses.iter().map(|e| e.status()) {
                if raw_status.intersects(git2::Status::WT_NEW) {
                    status.new_files += 1;
                }
                if raw_status.intersects(git2::Status::INDEX_NEW | git2::Status::INDEX_MODIFIED |
                        git2::Status::INDEX_DELETED | git2::Status::INDEX_RENAMED |
                        git2::Status::INDEX_TYPECHANGE) {
                    status.index_files += 1;
                }
                if raw_status.intersects(git2::Status::WT_MODIFIED | git2::Status::WT_DELETED |
                        git2::Status::WT_TYPECHANGE | git2::Status::WT_RENAMED) {
                    status.working_files += 1;
                }
            }
        }
        if let Ok(raw_head) = repo.head() {
            let head_oid = raw_head.target();
            let raw_branch = Branch::wrap(raw_head);
            let remote_oid = match raw_branch.upstream() {
                Ok(raw_upstream) => raw_upstream.into_reference().target(),
                _ => None
            };

            if let Some(oid) = head_oid {
                status.hash = format!("{}", oid);
                if let Ok(tag) = repo.find_tag(oid) {
                    if let Some(tag_name) = tag.name() {
                        status.tag = tag_name.to_string();
                    }
                }
            }

            if let (Some(oid), Some(roid)) = (head_oid, remote_oid) {
                if let Ok((ahead, behind)) = repo.graph_ahead_behind(oid, roid) {
                    status.ahead = ahead;
                    status.behind = behind;
                }
            }

            if let Ok(Some(branch_name)) = raw_branch.name() {
                status.branch = branch_name.to_string();
            }
        }

        let mut stash_count = 0;
        repo.stash_foreach(|_, _, _| {stash_count += 1; true}).is_ok();
        status.stash_count = stash_count;

        status
    }
}

pub fn format(status: &Status) -> String {
    let mut result = String::with_capacity(60);

    if status.branch.len() > 0 {
        result += status.branch.as_str();
    } else if status.tag.len() > 0 {
        result += status.branch.as_str();
    } else if status.hash.len() != 0 {
        result += &status.hash.as_str()[0..5];
    }

    if status.ahead > 0 {
        result += format!("▲{}", status.ahead).as_str();
    }
    if status.behind > 0 {
        result += format!("▼{}", status.behind).as_str();
    }
    if status.new_files > 0 {
        result += format!("□{}", status.new_files).as_str();
    }
    if status.index_files > 0 {
        result += format!("■{}", status.index_files).as_str();
    }
    if status.working_files > 0 {
        result+= format!("▣{}", status.working_files).as_str();
    }
    if status.stash_count > 0 {
        result += format!("▷{}", status.stash_count).as_str();
    }
    if  RepositoryState::Clean != status.state {
        result += format!("◊{:?}", status.state).to_uppercase().as_str();
    }

    result
}
