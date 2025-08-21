use super::config::WorldConfig;
use anyhow::{Result, Context, bail};
use git2::{Repository, Signature, Time, RemoteCallbacks, FetchOptions, PushOptions, Cred};
use std::path::{Path, PathBuf};

// Git status information for a world repository
#[derive(Debug)]
pub struct WorldGitStatus {
    pub ahead: usize,
    pub behind: usize,
    pub modified_files: Vec<String>,
    pub untracked_files: Vec<String>,
}

impl WorldGitStatus {
    pub fn is_clean(&self) -> bool {
        self.modified_files.is_empty() && self.untracked_files.is_empty()
    }
    
    pub fn needs_push(&self) -> bool {
        self.ahead > 0
    }
    
    pub fn needs_pull(&self) -> bool {
        self.behind > 0
    }
}

// Low-level Git operations
fn init_world_repo(world_path: &Path) -> Result<()> {
    let repo = Repository::init(world_path)
        .with_context(|| format!("Failed to initialize Git repository in {}", world_path.display()))?;
    
    // Create initial commit
    let signature = Signature::new("Multiverse CLI", "multiverse@localhost", &Time::new(0, 0))
        .context("Failed to create Git signature")?;
    
    let tree_id = {
        let mut index = repo.index()?;
        index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;
        index.write_tree()?
    };
    
    let tree = repo.find_tree(tree_id)?;
    repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        "Initial commit",
        &tree,
        &[],
    )?;
    
    Ok(())
}

fn clone_world_repo(repo_url: &str, target_path: &Path) -> Result<()> {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key_from_agent(username_from_url.unwrap_or("git"))
    });
    
    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);
    
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fetch_options);
    
    builder.clone(repo_url, target_path)
        .with_context(|| format!("Failed to clone repository {} to {}", repo_url, target_path.display()))?;
    
    Ok(())
}

fn pull_world_repo(world_path: &Path) -> Result<()> {
    let repo = Repository::open(world_path)
        .with_context(|| format!("Failed to open repository at {}", world_path.display()))?;
    
    let mut remote = repo.find_remote("origin")
        .context("No 'origin' remote found")?;
    
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key_from_agent(username_from_url.unwrap_or("git"))
    });
    
    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);
    
    // Fetch from remote
    remote.fetch(&["refs/heads/*:refs/remotes/origin/*"], Some(&mut fetch_options), None)
        .context("Failed to fetch from remote")?;
    
    // Get current HEAD and remote branch
    let head_ref = repo.head().context("Failed to get HEAD reference")?;
    let head_oid = head_ref.target().context("HEAD reference has no target")?;
    
    let remote_ref = repo.find_reference("refs/remotes/origin/main")
        .or_else(|_| repo.find_reference("refs/remotes/origin/master"))
        .context("No remote main/master branch found")?;
    let remote_oid = remote_ref.target().context("Remote reference has no target")?;
    
    // Check if we can fast-forward
    let (ahead, behind) = repo.graph_ahead_behind(head_oid, remote_oid)
        .context("Failed to calculate ahead/behind")?;
    
    if behind == 0 {
        // Already up to date
        return Ok(());
    }
    
    if ahead > 0 {
        // Divergent branches - cannot fast-forward
        bail!("Cannot fast-forward merge - branches have diverged. Resolve conflicts manually with 'git merge' in the world directory");
    }
    
    // Fast-forward merge
    let remote_commit = repo.find_commit(remote_oid).context("Failed to find remote commit")?;
    let refname = head_ref.name().context("HEAD reference has no name")?;
    
    repo.reference(refname, remote_oid, true, "Fast-forward merge")
        .context("Failed to update HEAD reference")?;
    
    // Update working directory
    repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))
        .context("Failed to update working directory")?;
    
    Ok(())
}

fn push_world_repo(world_path: &Path) -> Result<()> {
    let repo = Repository::open(world_path)
        .with_context(|| format!("Failed to open repository at {}", world_path.display()))?;
    
    let mut remote = repo.find_remote("origin")
        .context("No 'origin' remote found")?;
    
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key_from_agent(username_from_url.unwrap_or("git"))
    });
    
    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callbacks);
    
    remote.push(&["refs/heads/main:refs/heads/main"], Some(&mut push_options))
        .context("Failed to push to remote")?;
    
    Ok(())
}

fn get_world_repo_status(world_path: &Path) -> Result<WorldGitStatus> {
    let repo = Repository::open(world_path)
        .with_context(|| format!("Failed to open repository at {}", world_path.display()))?;
    
    // Get working directory status
    let statuses = repo.statuses(None)?;
    let mut modified_files = Vec::new();
    let mut untracked_files = Vec::new();
    
    for status in statuses.iter() {
        if let Some(path) = status.path() {
            let flags = status.status();
            if flags.contains(git2::Status::WT_MODIFIED) || flags.contains(git2::Status::INDEX_MODIFIED) {
                modified_files.push(path.to_string());
            }
            if flags.contains(git2::Status::WT_NEW) {
                untracked_files.push(path.to_string());
            }
        }
    }
    
    // Get ahead/behind counts (simplified)
    let ahead = 0; // Would require more complex Git analysis
    let behind = 0; // Would require more complex Git analysis
    
    Ok(WorldGitStatus {
        ahead,
        behind,
        modified_files,
        untracked_files,
    })
}

/// High-level Git management for individual worlds
pub struct WorldGitRepo {
    world_path: PathBuf,
    world_name: String,
}

impl WorldGitRepo {
    pub fn new<P: AsRef<Path>>(world_path: P) -> Result<Self> {
        let world_path = world_path.as_ref().to_path_buf();
        let world_name = world_path
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("Invalid world path"))?
            .to_string_lossy()
            .to_string();
        
        if !world_path.exists() {
            bail!("World directory does not exist: {}", world_path.display());
        }
        
        // A che serve? non esiste piu' il .world...
        let world_meta_path = world_path.join(".multiverse/");
        if !world_meta_path.exists() {
            bail!("Not a valid world directory (missing config dir): {}", world_path.display());
        }
        
        Ok(Self {
            world_path,
            world_name,
        })
    }
    
    pub fn name(&self) -> &str {
        &self.world_name
    }
    
    pub fn path(&self) -> &Path {
        &self.world_path
    }
    
    /// Initialize a new Git repository for this world
    pub fn init(&self) -> Result<()> {
        init_world_repo(&self.world_path)
            .with_context(|| format!("Failed to initialize Git repository for world '{}'", self.world_name))
    }
    
    /// Clone from a remote repository
    pub fn clone_from(repo_url: &str, target_path: &Path) -> Result<Self> {
        let world_name = target_path
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("Invalid target path"))?
            .to_string_lossy()
            .to_string();
        
        clone_world_repo(repo_url, target_path)
            .with_context(|| format!("Failed to clone world '{world_name}' from {repo_url}"))?;
        
        Ok(Self {
            world_path: target_path.to_path_buf(),
            world_name,
        })
    }
    
    /// Pull updates from remote
    pub fn pull(&self) -> Result<()> {
        pull_world_repo(&self.world_path)
            .with_context(|| format!("Failed to pull updates for world '{}'", self.world_name))
    }
    
    /// Push changes to remote
    pub fn push(&self) -> Result<()> {
        push_world_repo(&self.world_path)
            .with_context(|| format!("Failed to push changes for world '{}'", self.world_name))
    }
    
    /// Get Git status
    pub fn status(&self) -> Result<WorldGitStatus> {
        get_world_repo_status(&self.world_path)
            .with_context(|| format!("Failed to get Git status for world '{}'", self.world_name))
    }
    
    /// Check if repository is clean and up to date
    pub fn is_clean_and_synced(&self) -> Result<bool> {
        let status = self.status()?;
        Ok(status.is_clean() && !status.needs_push() && !status.needs_pull())
    }
}

/// High-level Git management for workspace operations
pub struct WorkspaceGitManager {
    workspace_path: PathBuf,
}

impl WorkspaceGitManager {
    pub fn new() -> Result<Self> {
        // For git-style approach, we work with the current world root
        let world_root = WorldConfig::get_world_root()
            .context("Not in a multiverse project directory")?;
        
        Ok(Self { workspace_path: world_root })
    }
    
    /// Get current world repository
    pub fn get_current_world_repo(&self) -> Result<WorldGitRepo> {
        WorldGitRepo::new(&self.workspace_path)
    }
    
    /// Pull updates for current world
    pub fn pull(&self) -> Result<()> {
        let world = self.get_current_world_repo()?;
        world.pull()
    }
    
    /// Push changes for current world
    pub fn push(&self) -> Result<()> {
        let world = self.get_current_world_repo()?;
        world.push()
    }
    
    /// Get status for current world
    pub fn status(&self) -> Result<()> {
        let world = self.get_current_world_repo()?;
        match world.status() {
            Ok(status) => {
                GitStatusPrinter::print_detailed(&status);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

/// Utility for printing Git status information
pub struct GitStatusPrinter;

impl GitStatusPrinter {
    pub fn print_detailed(status: &WorldGitStatus) {
        if status.is_clean() && !status.needs_push() && !status.needs_pull() {
            println!("   ✅ Repository is clean and up to date");
            return;
        }
        
        if status.needs_pull() {
            println!("   📥 Behind remote by {} commits - run 'pull' to update", status.behind);
        }
        
        if status.needs_push() {
            println!("   📤 Ahead of remote by {} commits - run 'push' to sync", status.ahead);
        }
        
        if !status.modified_files.is_empty() {
            println!("   📝 Modified files ({}):", status.modified_files.len());
            for file in &status.modified_files {
                println!("      M  {file}");
            }
        }
        
        if !status.untracked_files.is_empty() {
            println!("   ➕ Untracked files ({}):", status.untracked_files.len());
            for file in &status.untracked_files {
                println!("      ?  {file}");
            }
        }
    }
    
    pub fn print_compact(status: &WorldGitStatus) {
        let mut parts = Vec::new();
        
        if status.needs_pull() {
            parts.push(format!("📥 -{}", status.behind));
        }
        
        if status.needs_push() {
            parts.push(format!("📤 +{}", status.ahead));
        }
        
        if !status.modified_files.is_empty() {
            parts.push(format!("📝 {}M", status.modified_files.len()));
        }
        
        if !status.untracked_files.is_empty() {
            parts.push(format!("➕ {}?", status.untracked_files.len()));
        }
        
        if !parts.is_empty() {
            println!("      {}", parts.join(" "));
        }
    }
}
