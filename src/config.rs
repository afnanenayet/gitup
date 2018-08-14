/// A serializable struct representing the app configuration. Optional fields should be filled in
/// by default values if they are not supplied.
pub struct AppConfig {
    /// The path to the user's public SSH key
    pub ssh_pub_key_path: Option<String>,

    /// The path to the user's private SSH key
    pub ssh_key_path: Option<String>,

    /// Settings for updating each repo
    pub repos: Vec<RepoConfig>,

}

/// Configuration that pertains to the settings for a specific repository.
pub struct RepoConfig {
    /// Which branches to update. If this is not supplied, it will resolve to the default branch
    /// (master). The corresponding balues in the HashMap dictate whether the application should
    /// stash uncommitted changes before attempting to merge. If this is set to false, the app will
    /// fetch but will not merge the fetched branch into local branch.
    branches: Option<HashMap<String, bool>>,

    /// The remote to pull from. If not supplied, it will resolve to "origin"
    remote: Option<String>,
}
