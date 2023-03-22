// use git2::{Repository, MergeAnalysis};

// fn main() {
//     let repo = Repository::open(".").unwrap();
//     let reference = repo.find_reference("refs/heads/master").unwrap();
//     let analysis = repo.merge_analysis(&[&reference]).unwrap();

//     if analysis.0.is_fast_forward() {
//         println!("A fast-forward merge is possible.");
//     } else if analysis.0.is_normal() {
//         println!("A normal merge is possible.");
//     } else if analysis.0.is_up_to_date() {
//         println!("The branch is already up-to-date.");
//     } else {
//         println!("No merge is possible.");
//     }
// }

// use git2::{MergeAnalysis, Repository};

// fn main() {
//     let repo = Repository::open("/path/to/repo").unwrap();
//     let their_head = repo.find_reference("refs/heads/develop").unwrap();
//     let our_head = repo.find_reference("refs/heads/feature/1").unwrap();
//     let analysis = repo.merge_analysis_for_ref(&our_head, &their_head).unwrap();

//     if analysis.0.is_fast_forward() {
//         println!("A fast-forward merge is possible.");
//     } else if analysis.0.is_normal() {
//         println!("A normal merge is possible.");
//     } else if analysis.0.is_up_to_date() {
//         println!("The branch is already up-to-date.");
//     } else {
//         println!("No merge is possible.");
//     }
// }

use git2::{MergeAnalysis, Repository};
use std::env;

fn main() {
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
        println!("A fast-forward merge is possible.");
    } else if analysis.0.is_normal() {
        println!("A normal merge is possible.");
    } else if analysis.0.is_up_to_date() {
        println!("The branch is already up-to-date.");
    } else {
        println!("No merge is possible.");
    }
}
