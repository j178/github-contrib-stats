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

        let res = match (req.uri().path(), req.method()) {
            ("/", method) if method == "GET" => render_form(),
            ("/", method) if method == "POST" => handle_form_submit(req).await,
            ("/created", _) => render_created_repos(req).await,
            ("/contributed", _) => render_contributed_repos(req).await,
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
        .header("Content-Type", "image/svg+xml")
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
        .header("Content-Type", "image/svg+xml")
        .body(buf.into())?)
}

fn render_form() -> Result<Response<Body>, Error> {
    let html = r#"<!DOCTYPE html>
<html>
<head>
    <title>GitHub Contribution Stats</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
            max-width: 800px;
            margin: 2rem auto;
            padding: 0 1rem;
            line-height: 1.5;
            color: #24292e;
        }
        h1 { color: #2f363d; }
        .form-group {
            margin-bottom: 1rem;
        }
        label {
            display: block;
            margin-bottom: 0.5rem;
            font-weight: 600;
        }
        input {
            padding: 0.5rem;
            border: 1px solid #e1e4e8;
            border-radius: 6px;
            width: 100%;
            max-width: 300px;
        }
        button {
            background-color: #2ea44f;
            color: white;
            border: none;
            padding: 0.5rem 1rem;
            border-radius: 6px;
            cursor: pointer;
        }
        button:hover {
            background-color: #2c974b;
        }
        .result {
            margin-top: 2rem;
        }
        .markdown-snippet {
            background: #f6f8fa;
            padding: 0.5rem;
            border-radius: 6px;
            font-family: monospace;
            margin: 1rem 0;
        }
    </style>
</head>
<body>
    <h1>GitHub Contribution Stats Generator</h1>
    <form method="POST">
        <div class="form-group">
            <label for="username">GitHub Username:</label>
            <input type="text" id="username" name="username" required>
        </div>
        <div class="form-group">
            <label for="max_repos">Max Repositories (optional):</label>
            <input type="number" id="max_repos" name="max_repos" min="1">
        </div>
        <button type="submit">Generate Stats</button>
    </form>
</body>
</html>"#;

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(Body::from(html))?)
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
        .map(|v| v.to_string());

    let base_url = "https://github-contrib-stats.vercel.app";
    let max_repos_param = max_repos
        .as_ref()
        .map(|m| format!("&max_repos={}", m))
        .unwrap_or_default();

    let created_url = format!(
        "{}/created?username={}{}",
        base_url, username, max_repos_param
    );
    let contributed_url = format!(
        "{}/contributed?username={}{}",
        base_url, username, max_repos_param
    );

    let result_html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>GitHub Stats for {}</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
            max-width: 800px;
            margin: 2rem auto;
            padding: 0 1rem;
            line-height: 1.5;
            color: #24292e;
        }}
        .markdown-snippet {{
            background: #f6f8fa;
            padding: 0.5rem;
            border-radius: 6px;
            font-family: monospace;
            margin: 1rem 0;
        }}
        img {{
            max-width: 100%;
            height: auto;
            margin: 1rem 0;
        }}
        a {{
            color: #0366d6;
            text-decoration: none;
        }}
        a:hover {{
            text-decoration: underline;
        }}
    </style>
</head>
<body>
    <h1>GitHub Stats for {}</h1>
    <h2>Created Repositories</h2>
    <div class="markdown-snippet">
        ![Repos I created]({})
    </div>
    <img src="{}" alt="Created repositories stats">
    
    <h2>Contributed Repositories</h2>
    <div class="markdown-snippet">
        ![Repos I contributed to]({})
    </div>
    <img src="{}" alt="Contributed repositories stats">
    
    <p><a href="/">← Generate for another user</a></p>
</body>
</html>"#,
        username, username, created_url, created_url, contributed_url, contributed_url
    );

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html")
        .body(Body::from(result_html))?)
}
