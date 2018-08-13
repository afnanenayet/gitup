use consts;
use git2;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fmt::{self, Display};
use std::path::{self, Path, PathBuf};
use std::result;
use std::string::String;

/// Callback function for libgit2 that retrieves the proper credentials for a repository for
/// remote access.
pub fn git_credentials_callback(
    url: &str,
    username: Option<&str>,
    cred_type: git2::CredentialType,
) -> Result<git2::Cred, git2::Error> {
    debug!("git credential callback activated");
    debug!("credential type: {:?}", cred_type);
    let user = username.unwrap_or(consts::DEFAULT_SSH_USERNAME);

    if cred_type.contains(git2::CredentialType::USERNAME) {
        return git2::Cred::username(user);
    } else {
        return git2::Cred::ssh_key_from_agent(username.unwrap_or(consts::DEFAULT_SSH_USERNAME));
    }
}

/// Update a repository given the path to the repo and the desired branch to update.
/// The branches hashmap contains a mapping of branches to whether each branch should
/// have any uncommitted work stashed before pulling.
pub fn update_repo(
    path: &PathBuf,
    remote: &str,
    branches: &HashMap<String, bool>,
) -> RepoResult<()> {
    // Can't update something that isn't a repo
    if !is_valid_repo(path) {
        error!(
            "invalid repo supplied at {}",
            path.to_str().unwrap_or("(unknown)")
        );

        // set all branches to have the `InvalidRepo` error, since it applies to
        // every potential branch
        let mut error_map: HashMap<String, ErrorType> = HashMap::new();

        for pair in branches {
            error_map.insert(pair.0.to_string(), ErrorType::InvalidRepo);
        }
        return Err(RepoError::new(error_map));
    }

    // If repo is valid, attempt to git pull on each branch
    // We deref pair.1 because it's a pointer to a bool, and we want to pass
    // a copy of the bool. pair.0 is a reference to a String, giving us a &str
    for pair in branches {
        debug!("starting to update {}/{}", remote, pair.0);
        git_pull(path, remote, pair.0, *pair.1).unwrap();
    }
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

/// Given a valid git repository, pull for a particular branch. If
/// there are local changes that have not been committed, they will be stashed
/// and popped after the git repository is updated.
fn git_pull(path: &PathBuf, remote: &str, branch: &str, stash: bool) -> Result<(), git2::Error> {
    let repo = git2::Repository::open(path)?;
    let mut upstream = repo.find_remote(remote)?;

    // set up authentication callbacks so that credentials can be resolved
    let mut cbs = git2::RemoteCallbacks::default();
    cbs.credentials(&git_credentials_callback);

    // set up the fetch to use the auth callback
    let mut fetch_opts = git2::FetchOptions::default();
    fetch_opts.remote_callbacks(cbs);

    debug!("(git pull): fetching {}/{}", remote, branch);
    upstream
        .fetch(&[branch], Some(&mut fetch_opts), None)
        .unwrap();
    Ok(())
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

impl Display for RepoError {
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

        match update_repo(&path, "origin", &branches) {
            Ok(_) => panic!("Woops!"),
            Err(e) => assert!(e.error_map["master"] == ErrorType::InvalidRepo),
        }
    }

    #[test]
    fn test_git_credentials() {
        let url = "git@github.com:afnanenayet/gitup.git";
        let creds = git_credentials_callback(url, None, git2::CredentialType::SSH_KEY);
        assert!(creds.is_ok());
    }

    // Note: method this test fixes is not working and will time out
    #[test]
    fn test_git_pull() {
        let path = PathBuf::from(".");
        let branch = "master";
        let remote = "origin";
        git_pull(&path, remote, branch, true).unwrap();
    }
}
