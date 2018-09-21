use errors::*;
use git2;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use std::string::String;

pub fn with_auth<T, F>(url: &str, cfg: &git2::Config, mut f: F) -> Result<T>
where
    F: FnMut(&mut git2::Credentials) -> Result<T>,
{
    let mut cred_helper = git2::CredentialHelper::new(url);
    cred_helper.config(cfg);

    let mut ssh_uname_requested = false;
    let mut cred_helper_bad = None;
    let mut ssh_agent_attempts = Vec::new();
    let mut any_attempts = false;
    let mut tried_ssh_key = false;

    let mut res = f(&mut |url, username, allowed| {
        any_attempts = true;

        if allowed.contains(git2::CredentialType::USERNAME) {
            debug_assert!(username.is_none());
            ssh_uname_requested = true;
            return Err(git2::Error::from_str(
                "Will attempt to authenticate with usernames later",
            ));
        }

        if allowed.contains(git2::CredentialType::SSH_KEY) && !tried_ssh_key {
            tried_ssh_key = true;
            let username = username.unwrap();
            ssh_agent_attempts.push(username.to_string());
            return git2::Cred::ssh_key_from_agent(username);
        }

        if allowed.contains(git2::CredentialType::USER_PASS_PLAINTEXT) {
            let r = git2::Cred::credential_helper(cfg, url, username);
            cred_helper_bad = Some(r.is_err());
            return r;
        }

        if allowed.contains(git2::CredentialType::DEFAULT) {
            return git2::Cred::default();
        }

        // if everything fails...
        Err(git2::Error::from_str("no authentication available"))
    });

    if ssh_uname_requested {
        let mut attempts = Vec::new();
        attempts.push("git".to_string());

        if let Ok(s) = env::var("USER").or_else(|_| env::var("USERNAME")) {
            attempts.push(s);
        }

        if let Some(ref s) = cred_helper.username {
            attempts.push(s.clone());
        }

        while let Some(s) = attempts.pop() {
            // We should get `USERNAME` first, where we just return our attempt,
            // and then after that we should get `SSH_KEY`. If the first attempt
            // fails we'll get called again, but we don't have another option so
            // we bail out.
            let mut attempts = 0;
            res = f(&mut |_url, username, allowed| {
                if allowed.contains(git2::CredentialType::USERNAME) {
                    return git2::Cred::username(&s);
                }
                if allowed.contains(git2::CredentialType::SSH_KEY) {
                    debug_assert_eq!(Some(&s[..]), username);
                    attempts += 1;
                    if attempts == 1 {
                        ssh_agent_attempts.push(s.to_string());
                        return git2::Cred::ssh_key_from_agent(&s);
                    }
                }
                Err(git2::Error::from_str("no authentication available"))
            });

            // If we made two attempts then that means:
            //
            // 1. A username was requested, we returned `s`.
            // 2. An ssh key was requested, we returned to look up `s` in the
            //    ssh agent.
            // 3. For whatever reason that lookup failed, so we were asked again
            //    for another mode of authentication.
            //
            // Essentially, if `attempts == 2` then in theory the only error was
            // that this username failed to authenticate (e.g. no other network
            // errors happened). Otherwise something else is funny so we bail
            // out.
            if attempts != 2 {
                break;
            }
        }
    }
    if res.is_ok() || !any_attempts {
        return res.map_err(From::from);
    }

    // In the case of an authentication failure (where we tried something) then
    // we try to give a more helpful error message about precisely what we
    // tried.
    let res = res.map_err(Error::from).chain_err(|| {
        let mut msg = "failed to authenticate when downloading \
                       repository"
            .to_string();
        if !ssh_agent_attempts.is_empty() {
            let names = ssh_agent_attempts
                .iter()
                .map(|s| format!("`{}`", s))
                .collect::<Vec<_>>()
                .join(", ");
            msg.push_str(&format!(
                "\nattempted ssh-agent authentication, but \
                 none of the usernames {} succeeded",
                names
            ));
        }
        if let Some(failed_cred_helper) = cred_helper_bad {
            if failed_cred_helper {
                msg.push_str(
                    "\nattempted to find username/password via \
                     git's `credential.helper` support, but failed",
                );
            } else {
                msg.push_str(
                    "\nattempted to find username/password via \
                     `credential.helper`, but maybe the found \
                     credentials were incorrect",
                );
            }
        }
        msg
    })?;
    Ok(res) // TODO put back the proper error chain
}

/// Update a repository given the path to the repo and the desired branch to update.
/// The branches hashmap contains a mapping of branches to whether each branch should
/// have any uncommitted work stashed before pulling.
pub fn update_repo(path: &PathBuf, remote: &str, branches: &HashMap<String, bool>) -> Result<()> {
    // Can't update something that isn't a repo
    if !is_valid_repo(path) {
        bail!(
            "invalid repo supplied at {}",
            path.to_str().unwrap_or("(unknown)")
        );
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
fn git_pull(path: &PathBuf, remote: &str, branch: &str, _stash: bool) -> Result<()> {
    // set up all of the config stuff
    let repo =
        git2::Repository::open(path).chain_err(|| "could not open git repo at supplied path")?;
    let upstream = repo
        .find_remote(remote)
        .chain_err(|| "failed to find remote for git repo")?;
    let url = upstream
        .url()
        .chain_err(|| "could not retrieve remote URL")?;
    let git_config =
        git2::Config::open_default().chain_err(|| "could not retrieve any git config")?;

    with_auth(url, &git_config, |f| {
        // set up authentication callbacks so that credentials can be resolved
        let mut cbs = git2::RemoteCallbacks::default();
        cbs.credentials(f);

        // set up the fetch to use the auth callback
        let mut fetch_opts = git2::FetchOptions::default();
        fetch_opts.remote_callbacks(cbs);

        debug!("(git pull): fetching {}/{}", remote, branch);

        // need to make a copy of the upstream object, otherwise
        let mut upstream = upstream.clone();
        upstream
            .fetch(&[branch], Some(&mut fetch_opts), None)
            .chain_err(|| "failed to fetch remote")?;
        Ok(())
    })?;
    Ok(())
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
        assert!(update_repo(&path, "origin", &branches).is_err());
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
