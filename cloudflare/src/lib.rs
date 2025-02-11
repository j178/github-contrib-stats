use std::collections::HashMap;

use worker::{self, console_log, Context, Date, Env, Request, Response, Router};

use github_contrib_stats::{github, render::Render, render::SvgRenderer};

mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf()
            .and_then(|cf| cf.coordinates())
            .map_or_else(|| "unknown".to_string(), |coords| format!("{:?}", coords)),
        req.cf()
            .and_then(|cf| cf.region())
            .unwrap_or_else(|| "unknown".to_string())
    );
}

fn to_err(err: anyhow::Error) -> worker::Error {
    worker::Error::RustError(err.to_string())
}

#[worker::event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> worker::Result<Response> {
    log_request(&req);

    utils::set_panic_hook();

    let router = Router::new();

    router
        .get("/", |_, _| Response::ok("Hello from Workers!"))
        .get_async("/created", |req, _ctx| async move {
            let url = req.url()?;
            let query: HashMap<_, _> = url.query_pairs().collect();
            let username = query
                .get("username")
                .ok_or_else(|| worker::Error::RustError("name not found".to_string()))?;
            let max_repos = query
                .get("max_repos")
                .map(|x| x.parse::<usize>())
                .transpose()
                .map_err(|_| worker::Error::RustError("max_repos is not an integer".to_string()))?;

            let repos = github::get_created_repos(username, max_repos)
                .await
                .map_err(to_err)?;

            let mut buf = String::new();
            SvgRenderer::new().render_created_repos(&mut buf, &repos);
            Response::ok(buf)
        })
        .get_async("/contributed", |req, _ctx| async move {
            let url = req.url()?;
            let query: HashMap<_, _> = url.query_pairs().collect();
            let username = query
                .get("username")
                .ok_or_else(|| worker::Error::RustError("name not found".to_string()))?;
            let max_repos = query
                .get("max_repos")
                .map(|x| x.parse::<usize>())
                .transpose()
                .map_err(|_| worker::Error::RustError("max_repos is not an integer".into()))?;

            let repos = github::get_contributed_repos(username, max_repos)
                .await
                .map_err(to_err)?;

            let mut buf = String::new();
            SvgRenderer::new().render_contributed_repos(&mut buf, &repos, username);
            Response::ok(buf)
        })
        .get_async("/worker-version", |_, ctx| async move {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        })
        .run(req, env)
        .await
}
