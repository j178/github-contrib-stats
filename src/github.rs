use std::cmp::Reverse;
use std::collections::HashMap;
use std::sync::LazyLock;

use anyhow::{Context, Result};
use base64::prelude::*;
use chrono::{DateTime, Utc};
use futures::future::join_all;
use http::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use http::{HeaderMap, HeaderValue};
use log::{error, info};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

const PER_PAGE: u8 = 100;
const MAX_RESULTS: u32 = 1000;

static CLIENT: LazyLock<Client> = LazyLock::new(|| {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env var not found");
    let mut headers = HeaderMap::with_capacity(2);
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/vnd.github.v3+json"),
    );
    headers.insert(USER_AGENT, HeaderValue::from_static("github-contrib-stats"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {token}")).unwrap(),
    );

    let mut builder = Client::builder().default_headers(headers);

    #[cfg(not(target_arch = "wasm32"))]
    {
        use std::time::Duration;
        builder = builder.connect_timeout(Duration::from_millis(500));
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

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct PrimaryLanguage {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Repository {
    pub name_with_owner: String,
    pub stargazer_count: u32,
    pub fork_count: u32,
    pub primary_language: Option<PrimaryLanguage>,
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
    edges: Vec<Option<Edge<Repository>>>,
}

impl Repository {
    #[must_use]
    pub fn name(&self) -> &str {
        self.name_with_owner
            .split_once('/')
            .map_or(self.name_with_owner.as_str(), |(_, name)| name)
    }

    #[must_use]
    pub fn owner(&self) -> &str {
        self.name_with_owner
            .split_once('/')
            .map_or("", |(owner, _)| owner)
    }

    #[must_use]
    pub fn html_url(&self) -> String {
        format!("https://github.com/{}", self.name_with_owner)
    }

    #[must_use]
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
    info!("Fetching created repos for {username}");

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
        info!("Fetching repos after {end_cursor:?}");
        body["variables"]["after"] = json!(end_cursor);

        let mut resp: Value = CLIENT
            .clone()
            .post(GRAPHQL_URL)
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        check_graphql_errors(&resp)?;
        check_user_exists(&resp, username)?;

        let repo_result: RepositoryResult =
            serde_json::from_value(resp["data"]["user"]["repositories"].take())
                .context("failed to decode GitHub GraphQL user.repositories response")?;

        has_next_page = repo_result.page_info.has_next_page;
        end_cursor = repo_result.page_info.end_cursor;

        repos.extend(edge_nodes(repo_result.edges));
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

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
pub struct ContributedRepo {
    pub full_name: String,
    pub stargazer_count: u32,
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
struct Edge<T> {
    node: T,
}

fn edge_nodes<T>(edges: Vec<Option<Edge<T>>>) -> impl Iterator<Item = T> {
    edges.into_iter().flatten().map(|Edge { node }| node)
}

fn check_graphql_errors(resp: &Value) -> Result<()> {
    let Some(errors) = resp.get("errors") else {
        return Ok(());
    };

    let Some(error) = errors.as_array().and_then(|arr| arr.first()) else {
        return Err(anyhow::anyhow!("GitHub API returned errors: {errors}"));
    };

    let error_type = error
        .get("type")
        .and_then(|t| t.as_str())
        .unwrap_or("UNKNOWN");
    let message = error
        .get("message")
        .and_then(|m| m.as_str())
        .unwrap_or("Unknown error");

    if error_type == "NOT_FOUND" {
        return Err(anyhow::anyhow!("User not found: {message}"));
    }

    Err(anyhow::anyhow!("GitHub API error: {message}"))
}

fn check_user_exists(resp: &Value, username: &str) -> Result<()> {
    if resp["data"]["user"].is_null() {
        return Err(anyhow::anyhow!(
            "User '{username}' not found or is not a user account"
        ));
    }

    Ok(())
}

fn repository_name_from_pull_request_url(url: &str) -> Option<String> {
    let mut segments = url.rsplit('/');
    let _pull_number = segments.next()?;
    let pulls = segments.next()?;
    let repo = segments.next()?;
    let owner = segments.next()?;

    if pulls != "pull" {
        return None;
    }

    Some(format!("{owner}/{repo}"))
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RepositoryWithStargazerCount {
    pub stargazer_count: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PullRequest {
    pub url: String,
    pub created_at: DateTime<Utc>,
    pub repository: RepositoryWithStargazerCount,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
struct PullRequestSearchResult {
    issue_count: u32,
    page_info: PageInfo,
    edges: Vec<Option<Edge<PullRequest>>>,
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
          repository {
            stargazerCount
          }
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

    let mut data: Value = CLIENT
        .clone()
        .post(GRAPHQL_URL)
        .json(&body)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    check_graphql_errors(&data)?;

    let result: PullRequestSearchResult = serde_json::from_value(data["data"]["search"].take())
        .context("failed to decode GitHub GraphQL search response")?;
    Ok(result)
}

/// For one query, GitHub only returns up to 1000 results (`MAX_RESULTS`) at most, with a maximum of 100 results per page.
/// This function fetches up to `MAX_RESULTS` PRs, paginating if necessary.
/// For results beyond `MAX_RESULTS`, it will be fetched in subsequent queries with limited with the `created:<YYYY-MM-DD` filter.
async fn get_all_pages_of_pr(body: Value, total: Option<u32>) -> Result<(u32, Vec<PullRequest>)> {
    let mut all_prs = Vec::new();

    // total not known, fetch first page to get it
    let (total_all_query, beginning) = if let Some(total_all_query) = total {
        (total_all_query, 0)
    } else {
        let result = get_one_page_of_pr(body.clone(), None).await?;
        all_prs.extend(edge_nodes(result.edges));
        (result.issue_count, u32::from(PER_PAGE))
    };
    let total_this_query = MAX_RESULTS.min(total_all_query);

    // has more pages
    if total_this_query > beginning {
        all_prs.reserve((total_this_query - beginning) as usize);

        // cursor begins with base64("cursor:1")
        let results = join_all(
            (beginning..total_this_query)
                .step_by(usize::from(PER_PAGE))
                .map(|cursor_offset| {
                    let page_body = body.clone();

                    async move {
                        info!("fetching PRs after cursor: {cursor_offset}");
                        let cursor = BASE64_STANDARD.encode(format!("cursor:{cursor_offset}"));
                        get_one_page_of_pr(page_body, Some(cursor))
                            .await
                            .with_context(|| {
                                format!("failed to fetch PR page after cursor {cursor_offset}")
                            })
                    }
                }),
        )
        .await;
        for result in results {
            all_prs.extend(edge_nodes(result?.edges));
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
    // https://docs.github.com/en/search-github/searching-on-github/searching-issues-and-pull-requests
    // -user:USERNAME to exclude PRs from repos owned by USERNAME itself

    info!("Fetching contributed repos for {username}");

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

    let mut remaining_count = total_count.saturating_sub(MAX_RESULTS);
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

    let mut groups: HashMap<String, Vec<_>> = HashMap::new();
    for pr in all_prs {
        let Some(repo_name) = repository_name_from_pull_request_url(&pr.url) else {
            error!("failed to parse repository name from PR URL: {}", pr.url);
            continue;
        };

        groups.entry(repo_name).or_default().push(pr);
    }

    let mut repos: Vec<_> = groups
        .into_iter()
        .filter_map(|(repo_name, mut prs)| {
            prs.sort_by(|a, b| a.created_at.cmp(&b.created_at));
            let first_pr = prs.first()?.clone();
            let last_pr = prs.last()?.clone();
            let pr_count = u32::try_from(prs.len()).ok()?;

            Some(ContributedRepo {
                full_name: repo_name,
                stargazer_count: last_pr.repository.stargazer_count,
                pr_count,
                first_pr,
                last_pr,
            })
        })
        .collect();

    repos.sort_by_key(|repo| Reverse((repo.pr_count, repo.last_pr.created_at)));
    if let Some(n) = max_repos {
        repos.truncate(n);
    }

    Ok(repos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pull_request_search_result_accepts_null_edges() {
        let result: PullRequestSearchResult = serde_json::from_value(json!({
            "issueCount": 2,
            "pageInfo": {
                "hasNextPage": false,
                "endCursor": null,
            },
            "edges": [
                null,
                {
                    "node": {
                        "url": "https://github.com/owner/repo/pull/1",
                        "createdAt": "2026-06-15T00:00:00Z",
                        "repository": {
                            "stargazerCount": 42,
                        },
                    },
                },
            ],
        }))
        .unwrap();

        assert_eq!(result.edges.into_iter().flatten().count(), 1);
    }

    #[test]
    fn repository_result_accepts_null_edges() {
        let result: RepositoryResult = serde_json::from_value(json!({
            "totalCount": 2,
            "pageInfo": {
                "hasNextPage": false,
                "endCursor": null,
            },
            "edges": [
                null,
                {
                    "node": {
                        "nameWithOwner": "owner/repo",
                        "stargazerCount": 42,
                        "forkCount": 7,
                        "primaryLanguage": null,
                        "isArchived": false,
                        "createdAt": "2026-06-15T00:00:00Z",
                        "pushedAt": null,
                    },
                },
            ],
        }))
        .unwrap();

        assert_eq!(result.edges.into_iter().flatten().count(), 1);
    }

    #[test]
    fn repository_name_from_pull_request_url_extracts_owner_and_repo() {
        assert_eq!(
            repository_name_from_pull_request_url("https://github.com/owner/repo/pull/42")
                .as_deref(),
            Some("owner/repo")
        );
    }

    #[test]
    fn repository_name_from_pull_request_url_rejects_invalid_url() {
        assert_eq!(repository_name_from_pull_request_url("not-enough"), None);
        assert_eq!(
            repository_name_from_pull_request_url("https://github.com/owner/repo/issues/42"),
            None
        );
    }
}
