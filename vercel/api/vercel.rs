use std::collections::HashMap;

use anyhow::anyhow;
use log::info;
use url::Url;
use vercel_runtime::{run, Body, Error, Request, Response, StatusCode};

use github_contrib_stats::{github, render::Render, render::SvgRenderer};

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let h = |req: Request| async move {
        info!("new request: {}", req.uri());

        let res = match req.uri().path() {
            "/created" => render_created_repos(req).await,
            "/contributed" => render_contributed_repos(req).await,
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

pub async fn render_created_repos(req: Request) -> Result<Response<Body>, Error> {
    let url = Url::parse(&req.uri().to_string()).unwrap();
    let query: HashMap<_, _> = url.query_pairs().collect();
    let username = query
        .get("username")
        .ok_or_else(|| anyhow!("name not found"))?;
    let max_repos = query
        .get("max_repos")
        .map(|x| x.parse::<usize>())
        .transpose()?;

    let repos = github::get_created_repos(username, max_repos).await?;

    let mut buf = String::new();
    SvgRenderer::new().render_created_repos(&mut buf, &repos, username);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/markdown")
        .body(buf.into())?)
}

pub async fn render_contributed_repos(req: Request) -> Result<Response<Body>, Error> {
    let url = Url::parse(&req.uri().to_string()).unwrap();
    let query: HashMap<_, _> = url.query_pairs().collect();
    let username = query
        .get("username")
        .ok_or_else(|| anyhow!("name not found"))?;
    let max_repos = query
        .get("max_repos")
        .map(|x| x.parse::<usize>())
        .transpose()?;
    let repos = github::get_contributed_repos(username, max_repos).await?;

    let mut buf = String::new();
    SvgRenderer::new().render_contributed_repos(&mut buf, &repos, username);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/markdown")
        .body(buf.into())?)
}
