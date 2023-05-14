use std::cmp::Reverse;
use std::collections::HashMap;

use anyhow::Result;
use log::debug;
use octocrab::{Octocrab, Page};
use octocrab::models::issues::Issue;
use octocrab::models::Repository;

const PER_PAGE: u8 = 100;

pub async fn get_created_repos(
    client: &Octocrab,
    username: &str,
    max_repos: Option<usize>,
) -> Result<Vec<Repository>> {
    let page: Page<Repository> = client
        .get(
            format!("/users/{}/repos", username),
            Some(&[("per_page", PER_PAGE.to_string())]),
        )
        .await?;
    debug!("Got first page of repos");

    let repos: Vec<_> = client.all_pages(page).await?;
    debug!("Got all pages of repos");

    let mut repos: Vec<_> = repos
        .into_iter()
        .filter(|repo| repo.stargazers_count.unwrap() > 0 || repo.forks_count.unwrap() > 0)
        .collect();
    debug!("Filtered repos");

    repos.sort_by(|a, b| b.stargazers_count.cmp(&a.stargazers_count));
    if let Some(n) = max_repos {
        repos.truncate(n);
    }
    debug!("Sorted repos");

    Ok(repos)
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContributedRepo {
    pub repo: Repository,
    pub pr_count: u32,
    pub first_pr: Issue,
    pub last_pr: Issue,
}

pub async fn get_contributed_repos(
    client: &Octocrab,
    username: &str,
    max_repos: Option<usize>,
) -> Result<Vec<ContributedRepo>> {
    let page: Page<Issue> = client
        .search()
        .issues_and_pull_requests(&format!("author:{} type:pr", username))
        .per_page(PER_PAGE)
        .send()
        .await?;

    debug!("Got first page of prs");
    let groups = client
        .all_pages(page)
        .await?
        .into_iter()
        .filter(|pr| pr.author_association != "OWNER")
        .fold(HashMap::new(), |mut groups, pr| {
            let paths: Vec<_> = pr.repository_url.path_segments().unwrap().collect();
            let repo_key = (
                paths[paths.len() - 2].to_string(),
                paths[paths.len() - 1].to_string(),
            );

            groups.entry(repo_key).or_insert_with(Vec::new).push(pr);
            groups
        });
    debug!("Got all pages of prs");

    let mut repos = Vec::new();
    for (repo_key, mut prs) in groups.into_iter() {
        prs.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        let first_pr = prs.first().unwrap();
        let last_pr = prs.last().unwrap();
        let repo = client.repos(repo_key.0, repo_key.1).get().await?;
        repos.push(ContributedRepo {
            repo,
            pr_count: prs.len() as u32,
            first_pr: first_pr.clone(),
            last_pr: last_pr.clone(),
        });
    }
    debug!("Got repo info");

    repos.sort_by_key(|repo| Reverse((repo.pr_count, repo.last_pr.created_at)));
    if let Some(n) = max_repos {
        repos.truncate(n);
    }
    debug!("Sorted repos");

    Ok(repos)
}
