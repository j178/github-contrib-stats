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
        .header("Content-Type", "text/html; charset=utf-8")
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

    let result_html = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>GitHub Stats for {username}</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, Ubuntu, sans-serif;
            max-width: 1200px;
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
            cursor: pointer;
            position: relative;
            transition: background-color 0.2s;
        }}
        .markdown-snippet:hover {{
            background: #e1e4e8;
        }}
        .markdown-snippet::after {{
            content: 'Click to copy';
            position: absolute;
            right: 0.5rem;
            top: 50%;
            transform: translateY(-50%);
            font-size: 0.8rem;
            color: #6a737d;
            opacity: 0;
            transition: opacity 0.2s;
        }}
        .markdown-snippet:hover::after {{
            opacity: 1;
        }}
        .markdown-snippet.copied::after {{
            content: 'Copied!';
            color: #28a745;
        }}
        img {{
            max-width: 100%;
            height: auto;
            margin: 1rem 0;
            min-height: 200px;
            background: #f6f8fa;
            border-radius: 6px;
            display: block;
        }}
        .loading {{
            position: relative;
        }}
        .loading::after {{
            content: 'Loading...';
            position: absolute;
            left: 50%;
            top: 50%;
            transform: translate(-50%, -50%);
            color: #6a737d;
            opacity: 1;
            transition: opacity 0.3s;
        }}
        .loading:has(img.loaded)::after {{
            opacity: 0;
            pointer-events: none;
        }}
        .top-buttons {{
            position: fixed;
            top: 1rem;
            right: 1rem;
            display: flex;
            gap: 1rem;
            align-items: center;
        }}
        .github-button {{
            display: flex;
            align-items: center;
            gap: 0.5rem;
            padding: 0.5rem 1rem;
            background-color: #24292e;
            color: white;
            text-decoration: none;
            border-radius: 6px;
            font-size: 0.9rem;
            transition: background-color 0.2s;
        }}
        .github-button:hover {{
            background-color: #000;
        }}
        .github-icon {{
            width: 20px;
            height: 20px;
            fill: currentColor;
        }}
        a {{
            color: #0366d6;
            text-decoration: none;
        }}
        a:hover {{
            text-decoration: underline;
        }}
        .stats-grid {{
            display: grid;
            grid-template-columns: 1fr 1fr;
            gap: 2rem;
            margin: 2rem 0;
        }}
        .stats-column {{
            min-width: 0;
        }}
        @media (max-width: 768px) {{
            .stats-grid {{
                grid-template-columns: 1fr;
            }}
        }}
        .markdown-label {{
            font-size: 0.9rem;
            color: #586069;
            margin-bottom: 0.5rem;
            font-weight: 600;
        }}
    </style>
    <script>
        function copyMarkdown(element) {{
            const text = element.textContent.trim();
            navigator.clipboard.writeText(text).then(() => {{
                element.classList.add('copied');
                setTimeout(() => {{
                    element.classList.remove('copied');
                }}, 2000);
            }});
        }}
    </script>
</head>
<body>
    <div class="top-buttons">
        <a href="/">← Generate for another user</a>
        <a href="https://github.com/j178/github-contrib-stats" class="github-button" target="_blank" rel="noopener noreferrer">
            <svg class="github-icon" viewBox="0 0 16 16" version="1.1" aria-hidden="true">
                <path fill-rule="evenodd" d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"></path>
            </svg>
            Star on GitHub
        </a>
    </div>

    <h1>GitHub Stats for <a href="https://github.com/{username}">{username}</a></h1>
    <div class="stats-grid">
        <div class="stats-column">
            <h2>Created Repositories</h2>
            <div class="markdown-label">📋 Markdown</div>
            <div class="markdown-snippet" onclick="copyMarkdown(this)">
                ![Repos I created]({created_url})
            </div>
            <div class="loading">
                <img src="{created_url}" alt="Created repositories stats" onload="this.classList.add('loaded')">
            </div>
        </div>
        
        <div class="stats-column">
            <h2>Contributed Repositories</h2>
            <div class="markdown-label">📋 Markdown</div>
            <div class="markdown-snippet" onclick="copyMarkdown(this)">
                ![Repos I contributed to]({contributed_url})
            </div>
            <div class="loading">
                <img src="{contributed_url}" alt="Contributed repositories stats" onload="this.classList.add('loaded')">
            </div>
        </div>
    </div>
</body>
</html>"#,
    );

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(Body::from(result_html))?)
}

async fn render_created_svg(username: String, req: &Request) -> Result<Response<Body>, Error> {
    let url = Url::parse(&req.uri().to_string()).unwrap();
    let query: HashMap<_, _> = url.query_pairs().collect();
    let max_repos = query
        .get("max_repos")
        .map(|x| x.parse::<usize>())
        .transpose()?;

    let repos = github::get_created_repos(&username, max_repos).await?;

    let mut buf = String::new();
    SvgRenderer::new().render_created_repos(&mut buf, &repos, &username);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/svg+xml")
        .body(buf.into())?)
}

async fn render_contributed_svg(username: String, req: &Request) -> Result<Response<Body>, Error> {
    let url = Url::parse(&req.uri().to_string()).unwrap();
    let query: HashMap<_, _> = url.query_pairs().collect();
    let max_repos = query
        .get("max_repos")
        .map(|x| x.parse::<usize>())
        .transpose()?;

    let repos = github::get_contributed_repos(&username, max_repos).await?;

    let mut buf = String::new();
    SvgRenderer::new().render_contributed_repos(&mut buf, &repos, &username);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "image/svg+xml")
        .body(buf.into())?)
}
