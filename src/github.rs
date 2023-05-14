use std::cmp::Reverse;
use std::collections::HashMap;

use anyhow::Result;
use chrono::{DateTime, Utc};
use log::info;
use octocrab::models::Repository;
use octocrab::{Octocrab, Page};
use serde::Deserialize;
use serde_json::{json, Value};

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

    let repos: Vec<_> = client.all_pages(page).await?;

    let mut repos: Vec<_> = repos
        .into_iter()
        .filter(|repo| repo.stargazers_count.unwrap() > 0 || repo.forks_count.unwrap() > 0)
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
    pub pr_count: u32,
    pub first_pr: PullRequest,
    pub last_pr: PullRequest,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PageInfo {
    has_next_page: bool,
    end_cursor: Option<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Edge {
    pub node: PullRequest,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PullRequest {
    pub url: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PullRequestSearchResult {
    pub issue_count: u32,
    pub page_info: PageInfo,
    pub edges: Vec<Edge>,
}

async fn get_all_pages(client: &Octocrab, mut body: Value) -> Result<(u32, Vec<PullRequest>)> {
    let mut all_prs = Vec::new();
    let mut has_next_page = true;
    let mut end_cursor = None;
    let mut total = 0;

    while has_next_page {
        body["variables"]["cursor"] = json!(end_cursor);
        let data: Value = client.post("/graphql", Some(&body)).await?;
        let page: PullRequestSearchResult = serde_json::from_value(data["data"]["search"].clone())?;
        total = page.issue_count;
        has_next_page = page.page_info.has_next_page;
        end_cursor = page.page_info.end_cursor;
        info!(
            "batch: {}, cursor: {:?}",
            page.edges.len(),
            end_cursor.as_ref()
        );
        all_prs.extend(page.edges.into_iter().map(|x| x.node));
    }

    Ok((total, all_prs))
}

pub async fn get_contributed_repos(
    client: &Octocrab,
    username: &str,
    max_repos: Option<usize>,
) -> Result<Vec<ContributedRepo>> {
    // https://docs.github.com/en/rest/search?apiVersion=2022-11-28
    // For authenticated requests, you can make up to 30 requests per minute for all search endpoints except for the "Search code" endpoint.
    // The "Search code" endpoint requires you to authenticate and limits you to 10 requests per minute.
    // For unauthenticated requests, the rate limit allows you to make up to 10 requests per minute.
    // todo search returns 1000 results max, regardless of the actual matches, use `created:<YYYY-MM-DD` to filter
    // sort:created or sort:created-desc (default)
    let query = r#"
query ($q: String!, $perPage: Int!, $cursor: String) {
  search(type: ISSUE, query: $q, first: $perPage, after: $cursor) {
    pageInfo {
      hasNextPage
      endCursor
    }
    edges {
      node {
        ... on PullRequest {
          url
          createdAt
        }
      }
    }
    issueCount
  }
}
    "#;

    let first_query = format!(
        "author:{} type:pr is:public sort:created-desc -user:{}",
        username, username
    );

    let mut body = json!({
        "query": query,
        "variables": {
            "q": first_query,
            "perPage": PER_PAGE,
        }
    });

    let (total_count, prs) = get_all_pages(client, body.clone()).await?;

    let mut min_created_at = prs.last().unwrap().created_at.clone();

    let mut all_prs = Vec::with_capacity(total_count as usize);
    all_prs.extend(prs);

    while all_prs.len() < total_count as usize {
        info!(
            "total: {}, current: {}, min_created_at: {}",
            total_count,
            all_prs.len(),
            min_created_at.to_rfc3339()
        );

        body["variables"]["q"] = json!(format!(
            "{} created:<{}",
            first_query,
            min_created_at.to_rfc3339()
        ));
        let (_, prs) = get_all_pages(client, body.clone()).await?;
        match prs.last() {
            Some(pr) => min_created_at = pr.created_at.clone(),
            None => break,
        }
        all_prs.extend(prs);
    }

    let groups = all_prs.into_iter().fold(HashMap::new(), |mut groups, pr| {
        let paths: Vec<_> = pr.url.split("/").collect();
        let repo_name = format!("{}/{}", paths[paths.len() - 4], paths[paths.len() - 3]);

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
