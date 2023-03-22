use git2::{BranchType, Repository};

use std::env;

fn get_sorted_branches(repo_path: &str) -> Result<Vec<String>, git2::Error> {
    let repo = Repository::open(repo_path)?;
    let mut branches = repo
        .branches(Some(BranchType::Local))?
        .map(|b| b.unwrap())
        .collect::<Vec<_>>();
    branches.sort_by_key(|b| b.0.get().peel_to_commit().unwrap().committer().when());
    Ok(branches
        .into_iter()
        .map(|(branch, _)| branch.name().unwrap().unwrap().to_string())
        .collect())
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let path = ".";

    if args.len() == 2 {
        println!("Usage: {} <branch1> <branch2> ...", args[0]);
        return;
    }

    if args.len() < 3 {
        args = get_sorted_branches(path).unwrap();

        // this is cause normally first argument is the path to the executable
        // TODO: make this better
        args.insert(0, "dummy".to_string());
    }

    let repo = Repository::open(path).unwrap();
    let starting_head = repo.head().unwrap();

    for i in 1..args.len() {
        for j in i + 1..args.len() {
            let into_branch = &args[i];
            let from_branch = &args[j];

            let their_head = repo
                .find_reference(&format!("refs/heads/{}", from_branch))
                .unwrap();
            let our_head = repo
                .find_reference(&format!("refs/heads/{}", into_branch))
                .unwrap();
            let their_commit = repo.reference_to_annotated_commit(&their_head).unwrap();
            let analysis = repo
                .merge_analysis_for_ref(&our_head, &[&their_commit])
                .unwrap();

            println!("\nComparing {} with {}:", into_branch, from_branch);
            if analysis.0.is_fast_forward() {
                println!("🚀  No confilcts: fast-forward merge is possible.");
            } else if analysis.0.is_normal() {
                // println!("🛠️  A normal merge is possible."); // ⚠️ // 🚧 // 💣
                let out_commit = repo.reference_to_annotated_commit(&our_head).unwrap();
                check_normal_merge(&repo, &their_commit, &out_commit).unwrap();

                // this is to clean up the repo after the merge, which can leave dirty files
                let starting_head_commit = starting_head.peel_to_commit().unwrap();
                repo.reset(
                    starting_head_commit.as_object(),
                    git2::ResetType::Hard,
                    None,
                )
                .unwrap();
            // TODO - figure out if there are conflicts
            } else if analysis.0.is_up_to_date() {
                println!("✅  No conflicts: the branches are already up-to-date.");
            } else if analysis.0.is_none() {
                println!("❌  No merge is possible.");
            } else {
                println!("🤔  Unknown merge analysis result.");
            }
        }
    }
}

fn check_normal_merge(
    repo: &Repository,
    local: &git2::AnnotatedCommit,
    remote: &git2::AnnotatedCommit,
) -> Result<(), git2::Error> {
    let local_tree = repo.find_commit(local.id())?.tree()?;
    let remote_tree = repo.find_commit(remote.id())?.tree()?;
    let ancestor = repo
        .find_commit(repo.merge_base(local.id(), remote.id())?)?
        .tree()?;
    let mut idx = repo.merge_trees(&ancestor, &local_tree, &remote_tree, None)?;

    if idx.has_conflicts() {
        println!("⚠️  Merge conflicts detected...");
        repo.checkout_index(Some(&mut idx), None)?;
        return Ok(());
    }
    println!("🍀  Found conflicts, but can resolve them automatically.");

    return Ok(());
}
