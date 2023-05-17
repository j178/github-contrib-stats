use std::cmp::Reverse;
use std::collections::HashMap;
use std::ops::Deref;

use anyhow::Result;
use chrono::{DateTime, Utc};
use http::{HeaderMap, HeaderValue};
use http::header::{ACCEPT, AUTHORIZATION, USER_AGENT};
use log::info;
use once_cell::sync::Lazy;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

const PER_PAGE: u8 = 100;

static CLIENT: Lazy<Client> = Lazy::new(|| {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env var not found");
    let mut headers = HeaderMap::with_capacity(2);
    headers.insert(ACCEPT, HeaderValue::from_static("application/vnd.github.v3+json"));
    headers.insert(USER_AGENT, HeaderValue::from_static("github-contrib-stats"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", &token)).unwrap());

    let mut builder = Client::builder()
        .default_headers(headers);

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

const GITHUB_API_URL: &str = "https://api.github.com";
const GRAPHQL_URL: &str = "https://api.github.com/graphql";

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Repository {
    pub name: String,
    pub full_name: String,
    pub html_url: String,
    pub stargazers_count: u32,
    pub forks_count: u32,
    pub language: Option<String>,
    pub created_at: DateTime<Utc>,
    pub pushed_at: DateTime<Utc>,
}

struct HeaderLinks {
    next: Option<String>,
    #[allow(dead_code)]
    last: Option<String>,
    #[allow(dead_code)]
    prev: Option<String>,
    #[allow(dead_code)]
    first: Option<String>,
}

fn parse_links(headers: &HeaderMap) -> Result<HeaderLinks> {
    let mut first = None;
    let mut prev = None;
    let mut next = None;
    let mut last = None;

    if let Some(link) = headers.get("Link") {
        let links = link.to_str()?;

        for url_with_params in links.split(',') {
            let mut url_and_params = url_with_params.split(';');
            let url = url_and_params
                .next()
                .expect("url to be present")
                .trim()
                .trim_start_matches('<')
                .trim_end_matches('>');

            for param in url_and_params {
                if let Some((name, value)) = param.trim().split_once('=') {
                    let value = value.trim_matches('\"');

                    if name == "rel" {
                        match value {
                            "first" => first = Some(url.into()),
                            "prev" => prev = Some(url.into()),
                            "next" => next = Some(url.into()),
                            "last" => last = Some(url.into()),
                            other => panic!("unexpected rel: {}", other),
                        }
                    }
                }
            }
        }
    }

    Ok(HeaderLinks {
        first,
        prev,
        next,
        last,
    })
}

pub async fn get_created_repos(
    username: &str,
    max_repos: Option<usize>,
) -> Result<Vec<Repository>> {
    let resp = CLIENT
        .deref()
        .clone()
        .get(format!("{GITHUB_API_URL}/users/{username}/repos"))
        .query(&[("per_page", PER_PAGE)])
        .send()
        .await?
        .error_for_status()?;

    let mut links = parse_links(resp.headers())?;
    let mut repos: Vec<Repository> = resp.json().await?;

    while let Some(next) = links.next {
        let resp = CLIENT
            .deref()
            .clone()
            .get(next)
            .send()
            .await?
            .error_for_status()?;

        links = parse_links(resp.headers())?;
        let page: Vec<Repository> = resp.json().await?;
        repos.extend(page);
    }

    let mut repos: Vec<_> = repos
        .into_iter()
        .filter(|repo| repo.stargazers_count > 0 || repo.forks_count > 0)
        .collect();

    repos.sort_by(|a, b| b.stargazers_count.cmp(&a.stargazers_count));
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
struct Edge {
    node: PullRequest,
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
    edges: Vec<Edge>,
}

async fn graphql_all_pages(mut body: Value) -> Result<(u32, Vec<PullRequest>)> {
    let mut all_prs = Vec::new();
    let mut has_next_page = true;
    let mut end_cursor = None;
    let mut total = 0;

    // TODO maybe we can request concurrently
    while has_next_page {
        body["variables"]["cursor"] = json!(end_cursor);

        let data: Value = CLIENT
            .deref()
            .clone()
            .post(GRAPHQL_URL)
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

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

    let first_query =
        format!("author:{username} type:pr is:public sort:created-desc -user:{username}");

    let mut body = json!({
        "query": QUERY_PRS,
        "variables": {
            "q": first_query,
            "perPage": PER_PAGE,
        }
    });

    let (total_count, prs) = graphql_all_pages(body.clone()).await?;

    let mut min_created_at = match prs.last() {
        Some(pr) => pr.created_at,
        None => return Ok(Vec::new()),
    };

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
        let (_, prs) = graphql_all_pages(body.clone()).await?;
        match prs.last() {
            Some(pr) => min_created_at = pr.created_at,
            None => break,
        }
        all_prs.extend(prs);
    }

    let groups = all_prs.into_iter().fold(HashMap::new(), |mut groups, pr| {
        let paths: Vec<_> = pr.url.split('/').collect();
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
