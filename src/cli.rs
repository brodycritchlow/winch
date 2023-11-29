mod config_parser;
mod install_script_creation;

use clap::Parser;
use reqwest;
use serde_json::*;
use serde_derive::{Deserialize, Serialize};
use regex::Regex;


#[derive(Serialize, Deserialize, Debug)]
struct Owner {
    login: String,
    id: u64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    r#type: String,
    site_admin: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct License {
    key: String,
    name: String,
    spdx_id: String,
    url: String,
    node_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct GithubAPI {
    id: u64,
    node_id: String,
    name: String,
    full_name: String,
    private: bool,
    owner: Owner,
    html_url: String,
    description: Option<String>,
    fork: bool,
    url: String,
    forks_url: String,
    keys_url: String,
    collaborators_url: String,
    teams_url: String,
    hooks_url: String,
    issue_events_url: String,
    events_url: String,
    assignees_url: String,
    branches_url: String,
    tags_url: String,
    blobs_url: String,
    git_tags_url: String,
    git_refs_url: String,
    trees_url: String,
    statuses_url: String,
    languages_url: String,
    stargazers_url: String,
    contributors_url: String,
    subscribers_url: String,
    subscription_url: String,
    commits_url: String,
    git_commits_url: String,
    comments_url: String,
    issue_comment_url: String,
    contents_url: String,
    compare_url: String,
    merges_url: String,
    archive_url: String,
    downloads_url: String,
    issues_url: String,
    pulls_url: String,
    milestones_url: String,
    notifications_url: String,
    labels_url: String,
    releases_url: String,
    deployments_url: String,
    created_at: String,
    updated_at: String,
    pushed_at: String,
    git_url: String,
    ssh_url: String,
    clone_url: String,
    svn_url: String,
    homepage: Option<String>,
    size: u64,
    stargazers_count: u64,
    watchers_count: u64,
    language: String,
    has_issues: bool,
    has_projects: bool,
    has_downloads: bool,
    has_wiki: bool,
    has_pages: bool,
    has_discussions: bool,
    forks_count: u64,
    mirror_url: Option<String>,
    archived: bool,
    disabled: bool,
    open_issues_count: u64,
    license: License,
    allow_forking: bool,
    is_template: bool,
    web_commit_signoff_required: bool,
    topics: Vec<String>,
    visibility: String,
    forks: u64,
    open_issues: u64,
    watchers: u64,
    default_branch: String,
    temp_clone_token: Option<String>,
    network_count: u64,
    subscribers_count: u64,
}

#[derive(Parser, Debug)]
struct Args {
    #[clap(subcommand)]
    cmd: Option<Command>,
}

#[derive(Parser, Debug)]
enum Command {
    Install {
        #[clap()]
        package: String,

        #[clap(long, short = 'f')]
        force: bool,

        #[clap(long, short = 'r')]
        remote_repository: bool,
    },
    Remove {
        #[clap()]
        package: String,

        #[clap(long, short = 'f')]
        force: bool,

        #[clap(long, short = 'r')]
        remote_repository: bool,
    },
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Some(Command::Install { package, force, remote_repository}) => {
            // println!("Install command with package: {}", package);
            // println!("force: {}, remote_repo: {}", force, remote_repo);
            match remote_repository {
                true => {
                    if package.contains("github") {
                        // raw.githubusercontent.com
                        // <branch>/winch/config.winch
                        // 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
                        // let owner = package.find("/.*/.*").unwrap_or(0).to_string();
                        // let repo: String = owner.split("/").collect::<Vec<&str>>()[0].to_string();
                        let re = Regex::new(r"\/(.*)\/(.*)").unwrap();
                        let caps = re.captures(&package).unwrap();
                        let caps2 = Regex::new(r"(.*)/(.*)").unwrap().captures(caps.get(0).unwrap().as_str().strip_prefix("//github.com/").unwrap()).unwrap();
                        // println!("{:?}", caps2);
                        let owner = caps2.get(1).unwrap().as_str();
                        let repo = caps2.get(2).unwrap().as_str();

                        println!("{:?}:{:?}", owner, repo);
                        let package = package.replace("github", "raw.githubusercontent");

                        let client = reqwest::blocking::Client::new();
                        let response = client
                            .get(format!("https://api.github.com/repos/{}/{}", owner, repo))
                            .header("Accept", "application/vnd.github+json")
                            .header("User-Agent", "Awesome-Octocat-App")
                            .header("Authorization", format!("Bearer {}", "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"))
                            .header("X-GitHub-Api-Version", "2022-11-28")
                            .send();
                        //println!("{}", response.unwrap().text().unwrap().as_str());
                        let github_api: GithubAPI = serde_json::from_str(&response.unwrap().text().unwrap().as_str()).unwrap();
                        // println!("{:?}", github_api);
                        let gh_main_branch = github_api.default_branch;

                        let end_suffix = format!("/{}/winch/config.winch", gh_main_branch);

                        let package = format!("{}{}", package.to_owned(), (&end_suffix));
                        println!("{}", package);
                    }
                }
                _ => {
                    // index regular winch packages
                }
            }
        }
        Some(Command::Remove { package, force, remote_repository}) => {
            println!("Remove command with package: {}", package);
            println!("force: {}, remote_repo: {}", force, remote_repository);
        }
        None => {
            println!("No subcommand provided");
        }
    }
}