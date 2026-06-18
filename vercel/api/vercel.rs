use std::borrow::Cow;
use std::collections::HashMap;

use anyhow::anyhow;
use log::info;
use redis::AsyncCommands;
use url::Url;
use vercel_runtime::{Body, Error, Request, Response, StatusCode, run};

use github_contrib_stats::github::{ContributedRepo, Repository};
use github_contrib_stats::{github, render::Render, render::SvgRenderer};

const GENERATOR_TEMPLATE: &str = include_str!("generator.html");

type Query<'a> = HashMap<Cow<'a, str>, Cow<'a, str>>;

#[derive(Clone, Copy, Debug)]
struct StatsParams {
    max_repos: Option<usize>,
    min_stars: u32,
    min_forks: u32,
    min_prs: u32,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let h = |req: Request| async move {
        info!("new request: {}", req.uri());

        let res = match (req.uri().path(), req.method()) {
            ("/", method) if method == "GET" => render_stats_page(),
            (path, method) if method == "GET" && path.ends_with("/created.svg") => {
                match username_from_svg_path(path, "/created.svg") {
                    Some(username) => render_created_svg(username, &req).await,
                    None => not_found(),
                }
            }
            (path, method) if method == "GET" && path.ends_with("/contributed.svg") => {
                match username_from_svg_path(path, "/contributed.svg") {
                    Some(username) => render_contributed_svg(username, &req).await,
                    None => not_found(),
                }
            }
            (path, method) if method == "GET" && username_from_page_path(path).is_some() => {
                render_stats_page()
            }
            _ => not_found(),
        };
        match res {
            Ok(res) => Ok(res),
            Err(e) => Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(format!("Error: {}", e)))?),
        }
    };

    run(h).await
}

fn not_found() -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("Not found"))?)
}

fn username_from_svg_path<'a>(path: &'a str, suffix: &str) -> Option<&'a str> {
    let username = path.strip_prefix('/')?.strip_suffix(suffix)?;
    is_valid_username(username).then_some(username)
}

fn username_from_page_path(path: &str) -> Option<&str> {
    let username = path.strip_prefix('/')?;
    (!username.contains('/') && is_valid_username(username)).then_some(username)
}

fn is_valid_username(username: &str) -> bool {
    !username.is_empty()
        && username.len() <= 39
        && !username.starts_with('-')
        && !username.ends_with('-')
        && username
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
}

fn render_stats_page() -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(Body::from(GENERATOR_TEMPLATE))?)
}

fn parse_stats_params(req: &Request) -> Result<StatsParams, Error> {
    let url = Url::parse(&req.uri().to_string())?;
    let query: Query<'_> = url.query_pairs().collect();

    Ok(StatsParams {
        max_repos: parse_optional_usize(&query, "max_repos")?,
        min_stars: parse_u32(&query, "min_stars")?,
        min_forks: parse_u32(&query, "min_forks")?,
        min_prs: parse_u32(&query, "min_prs")?,
    })
}

fn parse_optional_usize(query: &Query<'_>, name: &str) -> Result<Option<usize>, Error> {
    query
        .get(name)
        .filter(|value| !value.is_empty())
        .map(|value| {
            value
                .parse()
                .map_err(|_| anyhow!("{} must be a positive integer", name).into())
        })
        .transpose()
}

fn parse_u32(query: &Query<'_>, name: &str) -> Result<u32, Error> {
    query
        .get(name)
        .filter(|value| !value.is_empty())
        .map_or(Ok(0), |value| {
            value
                .parse()
                .map_err(|_| anyhow!("{} must be a non-negative integer", name).into())
        })
}

fn filter_created_repos(mut repos: Vec<Repository>, params: StatsParams) -> Vec<Repository> {
    repos.retain(|repo| {
        repo.stargazer_count >= params.min_stars && repo.fork_count >= params.min_forks
    });
    if let Some(max_repos) = params.max_repos {
        repos.truncate(max_repos);
    }
    repos
}

fn filter_contributed_repos(
    mut repos: Vec<ContributedRepo>,
    params: StatsParams,
) -> Vec<ContributedRepo> {
    repos
        .retain(|repo| repo.stargazer_count >= params.min_stars && repo.pr_count >= params.min_prs);
    if let Some(max_repos) = params.max_repos {
        repos.truncate(max_repos);
    }
    repos
}

async fn get_redis_client() -> Result<redis::Client, Error> {
    let redis_url = std::env::var("KV_URL")?.replace("redis://", "rediss://");
    redis::Client::open(redis_url)
        .map_err(|e| anyhow::anyhow!("Failed to create Redis client: {}", e).into())
}

async fn get_cached_or_compute<T, F>(cache_key: &str, compute: F) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
    F: AsyncFnOnce() -> Result<T, anyhow::Error>,
{
    let redis_client = get_redis_client().await?;
    let mut conn = match redis_client.get_multiplexed_tokio_connection().await {
        Ok(conn) => conn,
        Err(e) => {
            info!("Failed to connect to Redis: {}", e);
            return compute().await.map_err(Error::from);
        }
    };

    // Try to get from cache first
    match conn.get::<_, Option<Vec<u8>>>(cache_key).await {
        Ok(Some(cached_data)) => match bincode::deserialize(&cached_data) {
            Ok(value) => {
                info!("Cache hit for key: {}", cache_key);
                Ok(value)
            }
            Err(e) => {
                info!("Failed to deserialize cache: {}", e);
                compute().await.map_err(Error::from)
            }
        },
        Ok(None) => {
            info!("Cache miss for key: {}", cache_key);
            let value = compute().await?;

            // Store in cache
            if let Ok(cached_data) = bincode::serialize(&value)
                && let Err(e) = conn
                    .set_ex::<_, _, ()>(cache_key, cached_data, 12 * 3600)
                    .await
            {
                info!("Failed to store in cache: {}", e);
            }

            Ok(value)
        }
        Err(e) => {
            info!("Failed to get from cache: {}", e);
            compute().await.map_err(Error::from)
        }
    }
}

async fn render_created_svg(username: &str, req: &Request) -> Result<Response<Body>, Error> {
    let params = parse_stats_params(req)?;

    let cache_key = format!("created:{}:all", username);
    let repos =
        get_cached_or_compute(&cache_key, || github::get_created_repos(&username, None)).await?;
    let repos = filter_created_repos(repos, params);

    let mut buf = String::new();
    SvgRenderer::new().render_created_repos(&mut buf, &repos, &username);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/svg+xml")
        .header(
            "Cache-Control",
            "public, max-age=7200, s-maxage=7200, stale-while-revalidate=86400",
        )
        .header("ETag", format!("\"{}\"", username))
        .body(buf.into())?)
}

async fn render_contributed_svg(username: &str, req: &Request) -> Result<Response<Body>, Error> {
    let params = parse_stats_params(req)?;

    let cache_key = format!("contributed:{}:all", username);
    let repos = get_cached_or_compute(&cache_key, || {
        github::get_contributed_repos(&username, None)
    })
    .await?;
    let repos = filter_contributed_repos(repos, params);

    let mut buf = String::new();
    SvgRenderer::new().render_contributed_repos(&mut buf, &repos, &username);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/svg+xml")
        .header(
            "Cache-Control",
            "public, max-age=7200, s-maxage=7200, stale-while-revalidate=86400",
        )
        .header("ETag", format!("\"{}\"", username))
        .body(buf.into())?)
}

#[cfg(test)]
mod tests {
    use super::{username_from_page_path, username_from_svg_path};

    #[test]
    fn username_page_path_accepts_single_username_segment() {
        assert_eq!(username_from_page_path("/j178"), Some("j178"));
        assert_eq!(username_from_page_path("/j178/created.svg"), None);
        assert_eq!(username_from_page_path("/"), None);
    }

    #[test]
    fn username_svg_path_accepts_valid_svg_routes() {
        assert_eq!(
            username_from_svg_path("/j178/created.svg", "/created.svg"),
            Some("j178")
        );
        assert_eq!(
            username_from_svg_path("/j178/contributed.svg", "/contributed.svg"),
            Some("j178")
        );
        assert_eq!(username_from_svg_path("/j178", "/created.svg"), None);
    }
}
