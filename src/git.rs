use anyhow::Result;
use git2::Repository;

// get git all branches
pub fn get_branches(repo_path: &str) -> Result<Vec<String>> {
    let repo = Repository::open(repo_path)?;
    let branches = repo
        .branches(None)?
        .filter_map(|branch| branch.ok())
        .filter_map(|(branch, _)| branch.name().ok().flatten().map(|s| s.to_string()))
        .collect();
    Ok(branches)
}

// switch git branch to another
pub fn checkout_branch(repo_path: &str, branch_name: &str) -> Result<()> {
    let repo = Repository::open(repo_path)?;
    let obj = repo.revparse_single(branch_name)?;
    repo.checkout_tree(&obj, None)?;
    repo.set_head(branch_name)?;
    Ok(())
}
