use std::collections::HashMap;

use anyhow::anyhow;
use log::info;
use octocrab::OctocrabBuilder;
use url::Url;
use vercel_runtime::{Body, Error, Request, Response, run, StatusCode};

use github_contrib_stats::github;
use github_contrib_stats::render::{Render, SvgRenderer};

#[tokio::main]
async fn main() -> Result<(), Error> {
    env_logger::init();

    let h = |req: Request| async move {
        let res = handler(req).await;
        match res {
            Ok(res) => Ok(res),
            Err(e) => {
                Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from(format!("Error: {}", e)))?)
            }
        }
    };

    run(h).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    info!("new request: {:?}", req.uri());
    let url = Url::parse(&req.uri().to_string()).unwrap();
    let query: HashMap<String, String> = url.query_pairs().into_owned().collect();
    let username = query.get("username").ok_or(anyhow!("name not found"))?;
    let max_repos = query.get("max_repos").and_then(|x| x.parse::<usize>().ok());

    let client = OctocrabBuilder::new().personal_token(std::env::var("GITHUB_TOKEN")?).build()?;

    let repos = github::get_contributed_repos(&client, username, max_repos).await?;

    let mut buf = String::new();
    SvgRenderer::new().render_contributed_repos(&mut buf, &repos);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/markdown")
        .body(buf.into())?)
}
