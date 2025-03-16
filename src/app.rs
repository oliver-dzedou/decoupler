use crate::markdown::MarkdownGenerator;
use fp_growth::algorithm::FPGrowth;
use std::path::PathBuf;
use std::process::Command;

fn collect_commits(path: &PathBuf) -> String {
    let commits_command = Command::new("git")
        .arg("log")
        .arg("--pretty=format:%h")
        .current_dir(path)
        .output()
        .expect(r#"Failed running "git log --pretty=format:%h""#);
    String::from_utf8(commits_command.stdout).expect("Failed parsing commits output")
}

fn collect_diff(commit: &str, path: &PathBuf) -> String {
    let diff_command = Command::new("git")
        .arg("diff-tree")
        .arg("--no-commit-id")
        .arg("--name-only")
        .arg(commit)
        .arg("-r")
        .current_dir(path)
        .output()
        .expect(&format!(
            "Failed running git --diff-tree --no-commit-id --name-only {} -r",
            commit
        ));
    String::from_utf8(diff_command.stdout).expect("Failed parsing output")
}

fn collect_commits_with_diffs(path: &PathBuf, ignore: &Vec<String>) -> Vec<Vec<String>> {
    let mut commits_with_diffs: Vec<Vec<String>> = Vec::new();
    let commits: String = collect_commits(path);
    commits.lines().for_each(|commit| {
        let diff: String = collect_diff(commit, path);
        let mut diffs: Vec<String> = Vec::new();
        diff.lines().for_each(|file| {
            if !ignore.iter().any(|ignored| file.contains(ignored)) {
                diffs.push(file.to_string());
            }
        });
        commits_with_diffs.push(diffs);
    });
    commits_with_diffs
}

pub(crate) fn find_coupling(
    path: &PathBuf,
    minimum_frequency: &u32,
    minimum_size: &u32,
    ignore: &Vec<String>,
) {
    let commits_with_diffs = collect_commits_with_diffs(path, ignore);
    let commits_with_diffs_as_str: Vec<Vec<&str>> = commits_with_diffs
        .iter()
        .map(|diff| diff.iter().map(|file| &**file).collect())
        .collect();
    let fp_growth = FPGrowth::new(commits_with_diffs_as_str, *minimum_frequency as usize);
    let fp_result = fp_growth.find_frequent_patterns();
    let frequent_patterns = fp_result.frequent_patterns();
    let mut filtered_fp_result: Vec<&(Vec<&str>, usize)> = frequent_patterns
        .iter()
        .filter(|(pattern, _)| pattern.len() >= *minimum_size as usize)
        .collect();
    filtered_fp_result.sort_unstable_by(|(_, freq_a), (_, freq_b)| freq_b.cmp(freq_a));
    let mut markdown_generator = MarkdownGenerator::new(&PathBuf::from("output.md"));
    markdown_generator.generate_markdown(
        &commits_with_diffs,
        &filtered_fp_result,
        minimum_frequency,
        minimum_size,
        ignore,
    );
}
