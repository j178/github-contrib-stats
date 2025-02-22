use std::path::Path;

use anyhow::{Result, anyhow, bail};
use git_testament::git_testament;
use tokio::join;

use github_contrib_stats::github::{ContributedRepo, Repository};
use github_contrib_stats::render::SvgRenderer;
use github_contrib_stats::{github, render::MarkdownRenderer, render::Render};

git_testament!(TESTAMENT);

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let matches = clap::Command::new("github-contrib-stats")
        .version(TESTAMENT.branch_name.unwrap_or("v0.1.0"))
        .author("j178")
        .about("Get your GitHub contribution stats")
        .arg(
            clap::Arg::new("username")
                .short('u')
                .long("username")
                .help("GitHub username")
                .required(true),
        )
        .arg(
            clap::Arg::new("update")
                .long("update")
                .help("Update the markdown file")
                .conflicts_with("format"),
        )
        .arg(
            clap::Arg::new("format")
                .short('f')
                .long("format")
                .value_parser(["markdown", "svg"])
                .default_value("markdown")
                .help("The output format"),
        )
        .arg(
            clap::Arg::new("max-repos")
                .short('m')
                .long("max-repos")
                .help("Maximum number of repositories to show"),
        )
        .get_matches();

    let username = matches.get_one::<String>("username").unwrap();
    let max_repos = matches.get_one::<usize>("max-repos").copied();

    let (created_repos, contributed_repos) = join!(
        github::get_created_repos(username, max_repos),
        github::get_contributed_repos(username, max_repos),
    );
    let (created_repos, contributed_repos) = (created_repos?, contributed_repos?);

    // Handle different output scenarios
    if let Some(update_file) = matches.get_one::<String>("update") {
        // Scenario 1: Update existing markdown file
        let path = Path::new(update_file);
        if path.exists() {
            update_markdown(path, created_repos, contributed_repos, username)?;
        } else {
            bail!("File {} does not exist", update_file);
        }
    } else {
        // Handle format-based output
        match matches.get_one::<String>("format").unwrap().as_str() {
            "markdown" => {
                // Scenario 2: Create new markdown file
                let output = Path::new("github-contrib-stats.md");
                let render = MarkdownRenderer::new();
                let mut buf =
                    String::from("# My GitHub Contribution Stats\n\n## Repos I Created\n\n");
                render.render_created_repos(&mut buf, &created_repos, username);
                buf.push_str("\n## Repos I've Contributed To\n\n");
                render.render_contributed_repos(&mut buf, &contributed_repos, username);
                std::fs::write(output, buf)?;
            }
            "svg" => {
                // Scenario 3: Create separate SVG files
                let render = SvgRenderer::new();
                let mut buf = String::new();
                render.render_created_repos(&mut buf, &created_repos, username);
                std::fs::write("created.svg", &buf)?;

                buf.clear();
                render.render_contributed_repos(&mut buf, &contributed_repos, username);
                std::fs::write("contributed.svg", buf)?;
            }
            _ => unreachable!("Invalid format"),
        }
    }

    Ok(())
}

fn update_markdown(
    path: &Path,
    created_repos: Vec<Repository>,
    contributed_repos: Vec<ContributedRepo>,
    username: &str,
) -> Result<()> {
    let render = MarkdownRenderer::new();
    let mut buf;
    if !path.exists() {
        buf = format!(
            "# My GitHub Contribution Stats

## Repos Created by {username}

<!-- BEGIN:created_repos -->
<!-- END:created_repos -->

## Repos {username} Contributed To

<!-- BEGIN:contributed -->
<!-- END:contributed -->
"
        );
    } else {
        buf = std::fs::read_to_string(path)?;
    }
    let mut part_buf = String::new();
    render.render_created_repos(&mut part_buf, &created_repos, username);
    replace_template(&mut buf, "created_repos", &part_buf)?;
    part_buf.clear();
    render.render_contributed_repos(&mut part_buf, &contributed_repos, username);
    replace_template(&mut buf, "contributed", &part_buf)?;

    std::fs::write(path, buf)?;
    Ok(())
}

fn replace_template(buf: &mut String, name: &str, part_buf: &str) -> Result<()> {
    let start = format!("<!-- BEGIN:{} -->\n", name);
    let end = format!("<!-- END:{} -->", name);
    let start_pos = buf
        .find(&start)
        .ok_or(anyhow!("begin marker {} not found", start))?;
    let end_pos = buf
        .find(&end)
        .ok_or(anyhow!("end marker {} not found", end))?;
    buf.replace_range(start_pos + start.len()..end_pos, part_buf);
    Ok(())
}
