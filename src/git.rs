use git2;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::path::PathBuf;
use std::result;
use std::string::String;

/// Update a repository given the path to the repo and the desired branch to update.
/// The branches hashmap contains a mapping of branches to whether each branch should
/// have any uncommitted work stashed before pulling.
pub fn update_repo(path: &PathBuf, branches: &HashMap<String, bool>) -> RepoResult<()> {
    // Can't update something that isn't a repo
    if !is_valid_repo(path) {
        // set all branches to have the `InvalidRepo` error, since it applies to
        // every potential branch
        let mut error_map: HashMap<String, ErrorType> = HashMap::new();

        for pair in branches {
            error_map.insert(pair.0.to_string(), ErrorType::InvalidRepo);
        }
        return Err(RepoError::new(error_map));
    }
    panic!("Not implemented yet");  // TODO remove
    Ok(())
}

/// Check whether a given path is a valid git repository that can be accessed by `libgit` (via the
/// `git2` crate.
fn is_valid_repo(path: &PathBuf) -> bool {
    match git2::Repository::open(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// Given a valid git repository, pull for a particular set of branches. If
/// there are local changes that have not been committed, they will be stashed
/// and popped after the git repository is updated.
fn git_pull(path: &PathBuf, branches: &HashMap<String, bool>) -> RepoResult<()> {
    panic!("Not implemented");  // TODO remove
}

/// A convenience type for results. Short for `Result<T, RepoError>`
type RepoResult<T> = result::Result<T, RepoError>;

#[derive(Debug, PartialEq)]
pub struct RepoError {
    /// A mapping of which error occurred for each branch. These usually will be identical,
    /// but there can be different errors for each branch.
    pub error_map: HashMap<String, ErrorType>,

    /// A human-readable string representation of the error, useful for debugging.
    pub details: String,
}

#[derive(Debug, PartialEq)]
pub enum ErrorType {
    InvalidRepo,
    NetworkError,
    MergeError,
    Unknown,
}

impl RepoError {
    fn new(error_map: HashMap<String, ErrorType>) -> RepoError {
        let mut error = RepoError {
            error_map: error_map,
            details: String::new(),
        };
        error.details = format!("{:?}", error.error_map);
        return error;
    }
}

impl fmt::Display for RepoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.error_map)
    }
}

impl Error for RepoError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_valid_repo_valid() {
        // Note: this assumes that the working directory is the root of the project directory
        let path = PathBuf::from(".");
        assert!(is_valid_repo(&path));
    }

    #[test]
    fn test_is_valid_repo_invalid() {
        let path = PathBuf::from("/");
        assert!(!is_valid_repo(&path));
    }

    #[test]
    fn test_update_repo_invalid_repo() {
        let path = PathBuf::from("/");
        let mut branches = HashMap::new();
        branches.insert(String::from("master"), true);

        match update_repo(&path, &branches) {
            Ok(_) => panic!("Woops!"),
            Err(e) => assert!(e.error_map["master"] == ErrorType::InvalidRepo),
        }
    }
}
