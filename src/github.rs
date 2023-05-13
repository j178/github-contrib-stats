use std::cmp::Reverse;
use std::collections::HashMap;

use anyhow::Result;
use octocrab::models::issues::Issue;
use octocrab::models::Repository;
use octocrab::{Octocrab, Page};

const PER_PAGE: u8 = 100;

pub async fn get_created_repos(client: &Octocrab, username: &str, max_repos: Option<usize>) -> Result<Vec<Repository>> {
    let page: Page<Repository> = client
        .get(
            format!("/users/{}/repos", username),
            Some(&[
                ("per_page", PER_PAGE.to_string()),
                ("type", "owner".to_string()),
            ]),
        )
        .await?;

    let mut repos: Vec<_> = client
        .all_pages(page)
        .await?
        .into_iter()
        .filter(|repo| repo.fork.is_none() || repo.stargazers_count.unwrap() > 0)
        .collect();

    repos.sort_by(|a, b| b.stargazers_count.cmp(&a.stargazers_count));
    if let Some(n) = max_repos {
        repos.truncate(n);
    }

    Ok(repos)
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContributedRepo {
    pub full_name: String,
    pub stargazers_count: u32,
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

    let groups = client
        .all_pages(page)
        .await?
        .into_iter()
        .filter(|pr| pr.author_association != "OWNER")
        .fold(HashMap::new(), |mut groups, pr| {
            let paths: Vec<_> = pr.repository_url.path_segments().unwrap().collect();
            let repo_name = format!("{}/{}", paths[paths.len() - 2], paths[paths.len() - 1]);

            groups.entry(repo_name).or_insert_with(Vec::new).push(pr);
            groups
        });

    let mut repos: Vec<_> = groups
        .into_iter()
        .map(|(repo_name, mut prs)| {
            prs.sort_by(|a, b| a.created_at.cmp(&b.created_at));
            let first_pr = prs.first().unwrap();
            let last_pr = prs.last().unwrap();
            ContributedRepo {
                full_name: repo_name,
                stargazers_count: 0,
                pr_count: prs.len() as u32,
                first_pr: first_pr.clone(),
                last_pr: last_pr.clone(),
            }
        })
        .collect();

    repos.sort_by_key(|repo| Reverse((repo.pr_count, repo.last_pr.created_at)));
    if let Some(n) = max_repos {
        repos.truncate(n);
    }

    Ok(repos)
}
