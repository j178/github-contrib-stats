use std::collections::HashMap;

use anyhow::anyhow;
use log::info;
use url::Url;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

use github_contrib_stats::{github, render::Render, render::SvgRenderer};
use redis::AsyncCommands;

const FORM_TEMPLATE: &str = include_str!("form.html");
const STATS_TEMPLATE: &str = include_str!("stats.html");

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let h = |req: Request| async move {
        info!("new request: {}", req.uri());

        let res = match (req.uri().path(), req.method()) {
            ("/", method) if method == "GET" => render_form(),
            ("/", method) if method == "POST" => handle_form_submit(req).await,
            (path, method) if method == "GET" && path.ends_with("/created.svg") => {
                let username = path
                    .trim_end_matches("/created.svg")
                    .trim_start_matches("/");
                render_created_svg(username.to_string(), &req).await
            }
            (path, method) if method == "GET" && path.ends_with("/contributed.svg") => {
                let username = path
                    .trim_end_matches("/contributed.svg")
                    .trim_start_matches("/");
                render_contributed_svg(username.to_string(), &req).await
            }
            (path, method) if method == "GET" && path.starts_with("/") => {
                let username = path.trim_start_matches("/");
                if !username.is_empty() {
                    render_stats_page(username.to_string(), &req).await
                } else {
                    render_form()
                }
            }
            _ => Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("Not found"))?),
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

fn render_form() -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(Body::from(FORM_TEMPLATE))?)
}

async fn handle_form_submit(req: Request) -> Result<Response<Body>, Error> {
    let body = req.into_body();
    let params = url::form_urlencoded::parse(&body).collect::<HashMap<_, _>>();

    let username = params
        .get("username")
        .ok_or_else(|| anyhow!("username not found"))?;
    let max_repos = params
        .get("max_repos")
        .filter(|v| !v.is_empty())
        .map(|v| format!("?max_repos={}", v))
        .unwrap_or_default();

    // Redirect to /<username>?max_repos=X
    Ok(Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", format!("/{}{}", username, max_repos))
        .body(Body::Empty)?)
}

async fn render_stats_page(username: String, req: &Request) -> Result<Response<Body>, Error> {
    let url = Url::parse(&req.uri().to_string()).unwrap();
    let query: HashMap<_, _> = url.query_pairs().collect();
    let max_repos_param = query
        .get("max_repos")
        .map(|v| format!("?max_repos={}", v))
        .unwrap_or_default();

    let origin = url.origin().ascii_serialization();
    let created_url = format!("{}/{}/created.svg{}", origin, username, max_repos_param);
    let contributed_url = format!("{}/{}/contributed.svg{}", origin, username, max_repos_param);

    let result_html = STATS_TEMPLATE
        .replace("{username}", &username)
        .replace("{created_url}", &created_url)
        .replace("{contributed_url}", &contributed_url);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(Body::from(result_html))?)
}

async fn get_redis_client() -> Result<redis::Client, Error> {
    let redis_url = std::env::var("KV_URL")?.replace("redis://", "rediss://");
    redis::Client::open(redis_url)
        .map_err(|e| anyhow::anyhow!("Failed to create Redis client: {}", e).into())
}

async fn get_cached_or_compute<T, F, Fut>(cache_key: &str, compute: F) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<T, anyhow::Error>>,
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
            Ok(value) => Ok(value),
            Err(e) => {
                info!("Failed to deserialize cache: {}", e);
                compute().await.map_err(Error::from)
            }
        },
        Ok(None) => {
            info!("Cache miss for key: {}", cache_key);
            let value = compute().await?;

            // Store in cache
            if let Ok(cached_data) = bincode::serialize(&value) {
                if let Err(e) = conn
                    .set_ex::<_, _, ()>(cache_key, cached_data, 2 * 3600)
                    .await
                {
                    info!("Failed to store in cache: {}", e);
                }
            }

            Ok(value)
        }
        Err(e) => {
            info!("Failed to get from cache: {}", e);
            compute().await.map_err(Error::from)
        }
    }
}

async fn render_created_svg(username: String, req: &Request) -> Result<Response<Body>, Error> {
    let url = Url::parse(&req.uri().to_string())?;
    let query: HashMap<_, _> = url.query_pairs().collect();
    let max_repos = query
        .get("max_repos")
        .map(|x| x.parse::<usize>())
        .transpose()?;

    let cache_key = format!("created:{}:{:?}", username, max_repos);
    let repos = get_cached_or_compute(&cache_key, || {
        github::get_created_repos(&username, max_repos)
    })
    .await?;

    let mut buf = String::new();
    SvgRenderer::new().render_created_repos(&mut buf, &repos, &username);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/svg+xml")
        .header("Cache-Control", "public, max-age=3600")
        .header("ETag", format!("\"{}\"", username))
        .body(buf.into())?)
}

async fn render_contributed_svg(username: String, req: &Request) -> Result<Response<Body>, Error> {
    let url = Url::parse(&req.uri().to_string())?;
    let query: HashMap<_, _> = url.query_pairs().collect();
    let max_repos = query
        .get("max_repos")
        .map(|x| x.parse::<usize>())
        .transpose()?;

    let cache_key = format!("contributed:{}:{:?}", username, max_repos);
    let repos = get_cached_or_compute(&cache_key, || {
        github::get_contributed_repos(&username, max_repos)
    })
    .await?;

    let mut buf = String::new();
    SvgRenderer::new().render_contributed_repos(&mut buf, &repos, &username);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/svg+xml")
        .header("Cache-Control", "public, max-age=3600")
        .header("ETag", format!("\"{}\"", username))
        .body(buf.into())?)
}
