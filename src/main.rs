use git2::Repository;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <branch1> <branch2> ...", args[0]);
        return;
    }

    let repo = Repository::open(".").unwrap();

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

            println!("Comparing {} with {}:", into_branch, from_branch);
            if analysis.0.is_fast_forward() {
                println!("🚀 No confilcts: fast-forward merge is possible.");
            } else if analysis.0.is_normal() {
                println!("🛠️ A normal merge is possible."); // ⚠️ // 🚧 // 💣
                let out_commit = repo.reference_to_annotated_commit(&our_head).unwrap();
                check_normal_merge(&repo, &their_commit, &out_commit).unwrap();
            // TODO - figure out if there are conflicts
            } else if analysis.0.is_up_to_date() {
                println!("✅ No conflicts: the branches are already up-to-date.");
            } else if analysis.0.is_none() {
                println!("No merge is possible.");
            } else {
                println!("❌ Unknown merge analysis result.");
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
        println!("⚠️ Merge conflicts detected...");
        repo.checkout_index(Some(&mut idx), None)?;
        return Ok(());
    }
    println!("🚀 Found conflicts, but can resolve them automatically.");

    return Ok(());
    // let result_tree = repo.find_tree(idx.write_tree_to(repo)?)?;
    // // now create the merge commit
    // let msg = format!("Merge: {} into {}", remote.id(), local.id());
    // let sig = repo.signature()?;
    // let local_commit = repo.find_commit(local.id())?;
    // let remote_commit = repo.find_commit(remote.id())?;
    // // Do our merge commit and set current branch head to that commit.
    // let _merge_commit = repo.commit(
    //     Some("HEAD"),
    //     &sig,
    //     &sig,
    //     &msg,
    //     &result_tree,
    //     &[&local_commit, &remote_commit],
    // )?;
    // // Set working tree to match head.
    // repo.checkout_head(None)?;
    // Ok(())
}

fn analyse_two_branches() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <into_branch> <from_branch>", args[0]);
        return;
    }

    let into_branch = &args[1];
    let from_branch = &args[2];
    let repo = Repository::open(".").unwrap();
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

    if analysis.0.is_fast_forward() {
        println!("There are conflicts: a fast-forward merge is possible.");
    } else if analysis.0.is_normal() {
        println!("There are conflicts: a normal merge is possible.");
    } else if analysis.0.is_up_to_date() {
        println!("No conflicts: the branches are already up-to-date.");
    } else if analysis.0.is_none() {
        println!("No merge is possible.");
    } else {
        println!("Unknown merge analysis result.");
    }
}
