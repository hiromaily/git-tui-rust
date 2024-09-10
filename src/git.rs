use anyhow::{anyhow, Result};
use git2::{BranchType, Repository};
use log::debug;

// Get git all branches
pub fn get_branches(repo_path: &str) -> Result<Vec<String>> {
    debug!("get_branches(repo_path): {:?}", repo_path);
    let repo = Repository::open(repo_path)?;
    let branches = repo
        .branches(Some(BranchType::Local))?
        .filter_map(|branch| branch.ok())
        .filter_map(|(branch, _)| branch.name().ok().flatten().map(|s| s.to_string()))
        .collect();
    Ok(branches)
}

// Switch git branch to another
pub fn checkout_branch(repo_path: &str, branch_name: &str) -> Result<()> {
    debug!(
        "checkout_branch(repo_path,branch_name: {:?}, {:?}",
        repo_path, branch_name
    );
    let repo = Repository::open(repo_path)?;
    let (object, reference) = repo.revparse_ext(branch_name)?;
    repo.checkout_tree(&object, None)?;
    if let Some(gref) = reference {
        repo.set_head(gref.name().unwrap())?;
    } else {
        repo.set_head_detached(object.id())?;
    }
    Ok(())
}

// Get current branch name
pub fn get_current_branch(repo_path: &str) -> Result<String> {
    debug!("get_current_branch(repo_path): {:?}", repo_path);
    let repo = Repository::open(repo_path)?;
    let head = repo.head()?;
    let shorthand = head
        .shorthand()
        .ok_or_else(|| anyhow!("Failed to get branch name"))?;
    Ok(shorthand.to_string())
}
