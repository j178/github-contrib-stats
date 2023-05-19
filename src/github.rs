use std::cmp::Reverse;
use std::collections::HashMap;

use anyhow::Result;
use base64::prelude::*;
use chrono::{DateTime, Utc};
use futures::future::join_all;
use http::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use http::{HeaderMap, HeaderValue};
use log::{error, info};
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_json::{json, Value};

const PER_PAGE: u8 = 100;
const MAX_RESULTS: u32 = 1000;

static CLIENT: Lazy<Client> = Lazy::new(|| {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env var not found");
    let mut headers = HeaderMap::with_capacity(2);
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/vnd.github.v3+json"),
    );
    headers.insert(USER_AGENT, HeaderValue::from_static("github-contrib-stats"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", &token)).unwrap(),
    );

    let mut builder = Client::builder().default_headers(headers);

    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::time::Duration;
        builder = builder.connect_timeout(Duration::from_secs(500));
    }
    #[cfg(target_arch = "wasm32")]
    {
        builder = builder;
    }

    builder.build().unwrap()
});

const GRAPHQL_URL: &str = "https://api.github.com/graphql";
const QUERY_REPOS: &str = "\
query ($username: String!, $perPage: Int!, $after: String) {
  user(login: $username) {
    repositories(
      ownerAffiliations: OWNER
      isFork: false
      first: $perPage
      after: $after
    ) {
      totalCount
      pageInfo {
        hasNextPage
        endCursor
      }
      edges {
        node {
          nameWithOwner
          stargazerCount
          forkCount
          primaryLanguage {
            name
          }
          isArchived
          createdAt
          pushedAt
        }
      }
    }
  }
}
";

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
struct PrimaryLanguage {
    name: String,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub name_with_owner: String,
    pub stargazer_count: u32,
    pub fork_count: u32,
    primary_language: Option<PrimaryLanguage>,
    pub is_archived: bool,
    pub created_at: DateTime<Utc>,
    pub pushed_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RepositoryResult {
    #[allow(dead_code)]
    total_count: u32,
    page_info: PageInfo,
    edges: Vec<Edge<Repository>>,
}

impl Repository {
    pub fn name(&self) -> &str {
        self.name_with_owner.split_once('/').unwrap().1
    }

    pub fn owner(&self) -> &str {
        self.name_with_owner.split_once('/').unwrap().0
    }

    pub fn html_url(&self) -> String {
        format!("https://github.com/{}", self.name_with_owner)
    }

    pub fn language(&self) -> &str {
        self.primary_language
            .as_ref()
            .map_or("N/A", |l| l.name.as_str())
    }
}

pub async fn get_created_repos(
    username: &str,
    max_repos: Option<usize>,
) -> Result<Vec<Repository>> {
    info!("fetching created repos for {}", username);

    let mut body = json!({
        "query": QUERY_REPOS,
        "variables": {
            "username": username,
            "perPage": PER_PAGE,
        }
    });

    let mut has_next_page = true;
    let mut end_cursor = None;
    let mut repos = Vec::new();

    while has_next_page {
        info!("fetching Repos after {:?}", end_cursor);
        body["variables"]["after"] = json!(end_cursor);

        let resp: Value = CLIENT
            .clone()
            .post(GRAPHQL_URL)
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        let repo_result: RepositoryResult =
            serde_json::from_value(resp["data"]["user"]["repositories"].clone())?;

        has_next_page = repo_result.page_info.has_next_page;
        end_cursor = repo_result.page_info.end_cursor;

        repos.extend(repo_result.edges.into_iter().map(|edge| edge.node));
    }

    let mut repos: Vec<_> = repos
        .into_iter()
        .filter(|repo| repo.stargazer_count > 0 || repo.fork_count > 0)
        .collect();

    repos.sort_by(|a, b| b.stargazer_count.cmp(&a.stargazer_count));
    if let Some(n) = max_repos {
        repos.truncate(n);
    }

    Ok(repos)
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ContributedRepo {
    pub full_name: String,
    pub pr_count: u32,
    pub first_pr: PullRequest,
    pub last_pr: PullRequest,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
struct PageInfo {
    has_next_page: bool,
    end_cursor: Option<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
struct Edge<T>
where
    T: DeserializeOwned,
{
    // https://github.com/serde-rs/serde/issues/1296
    #[serde(bound = "")]
    node: T,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PullRequest {
    pub url: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
struct PullRequestSearchResult {
    issue_count: u32,
    page_info: PageInfo,
    edges: Vec<Edge<PullRequest>>,
}

const QUERY_PRS: &str = "\
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
}";

async fn get_one_page_of_pr(
    mut body: Value,
    cursor: Option<String>,
) -> Result<PullRequestSearchResult> {
    body["variables"]["cursor"] = json!(cursor);

    let data: Value = CLIENT
        .clone()
        .post(GRAPHQL_URL)
        .json(&body)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    let result: PullRequestSearchResult = serde_json::from_value(data["data"]["search"].clone())?;
    Ok(result)
}

async fn get_all_pages_of_pr(body: Value, total: Option<u32>) -> Result<(u32, Vec<PullRequest>)> {
    let mut all_prs = Vec::new();

    let total_all_query;
    let mut beginning = 0u32;
    // total not known, fetch first page to get it
    if total.is_none() {
        let result = get_one_page_of_pr(body.clone(), None).await?;
        total_all_query = result.issue_count;
        all_prs.extend(result.edges.into_iter().map(|edge| edge.node));
        beginning = PER_PAGE as u32;
    } else {
        total_all_query = total.unwrap();
    }
    let total_this_query = MAX_RESULTS.min(total_all_query);

    // has more pages
    if total_this_query > beginning {
        all_prs.reserve((total_this_query - beginning) as usize);

        // cursor begins with base64("cursor:1")
        let futures = (beginning..total_this_query)
            .step_by(100)
            .map(|cursor| {
                info!("fetching PRs after cursor: {}", cursor);
                let cursor = BASE64_STANDARD.encode(format!("cursor:{cursor}"));
                get_one_page_of_pr(body.clone(), Some(cursor))
            })
            .collect::<Vec<_>>();

        let results = join_all(futures).await;
        for result in results {
            if let Ok(result) = result {
                all_prs.extend(result.edges.into_iter().map(|edge| edge.node));
            } else {
                error!("failed to fetch a page of PRs")
            }
        }
    }

    Ok((total_all_query, all_prs))
}

pub async fn get_contributed_repos(
    username: &str,
    max_repos: Option<usize>,
) -> Result<Vec<ContributedRepo>> {
    // https://docs.github.com/en/rest/search?apiVersion=2022-11-28
    // For authenticated requests, you can make up to 30 requests per minute for all search endpoints except for the "Search code" endpoint.
    // The "Search code" endpoint requires you to authenticate and limits you to 10 requests per minute.
    // For unauthenticated requests, the rate limit allows you to make up to 10 requests per minute.

    // search returns 1000 results max, regardless of the actual matches, use `created:<YYYY-MM-DD` to filter
    // sort:created or sort:created-desc (default)

    info!("fetching contributed repos for {}", username);

    let first_query =
        format!("author:{username} type:pr is:public sort:created-desc -user:{username}");

    let mut body = json!({
        "query": QUERY_PRS,
        "variables": {
            "q": first_query,
            "perPage": PER_PAGE,
        }
    });

    let (total_count, prs) = get_all_pages_of_pr(body.clone(), None).await?;

    let mut min_created_at = match prs.last() {
        Some(pr) => pr.created_at,
        None => return Ok(Vec::new()),
    };

    let mut all_prs = Vec::with_capacity(total_count as usize);
    all_prs.extend(prs);

    let mut remaining_count = total_count - MAX_RESULTS;
    while remaining_count > 0 {
        info!(
            "total: {}, remaining: {}, min_created_at: {}",
            total_count,
            remaining_count,
            min_created_at.to_rfc3339()
        );

        body["variables"]["q"] = json!(format!(
            "{} created:<{}",
            first_query,
            min_created_at.to_rfc3339()
        ));
        let (_, prs) = get_all_pages_of_pr(body.clone(), Some(remaining_count)).await?;
        match prs.last() {
            Some(pr) => min_created_at = pr.created_at,
            None => break,
        }
        all_prs.extend(prs);
        remaining_count = remaining_count.saturating_sub(MAX_RESULTS);
    }

    // Group PRs by repo
    let groups = all_prs.into_iter().fold(HashMap::new(), |mut groups, pr| {
        let paths: Vec<_> = pr.url.split('/').collect();
        let repo_name = format!("{}/{}", paths[paths.len() - 4], paths[paths.len() - 3]);

        groups.entry(repo_name).or_insert_with(Vec::new).push(pr);
        groups
    });

    // Transform groups into ContributedRepo
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
