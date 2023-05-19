use std::path::Path;

use anyhow::{anyhow, bail, Result};
use git_testament::git_testament;
use tokio::join;

use github_contrib_stats::{self as github, MarkdownRenderer, Render};

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
            clap::Arg::new("output")
                .short('o')
                .long("output")
                .default_value("README.md")
                .help("The file to write the stats to"),
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
        github::get_created_repos(&username, max_repos),
        github::get_contributed_repos(&username, max_repos),
    );
    let (created_repos, contributed_repos) = (created_repos?, contributed_repos?);

    let output = matches.get_one::<String>("output").unwrap();
    if output.ends_with(".md") {
        let output = Path::new(output);
        let render = MarkdownRenderer::new();
        let mut buf;
        if !output.exists() {
            buf = "# My GitHub Contribution Stats

## Repos I Created

<!-- BEGIN:created_repos -->
<!-- END:created_repos -->

## Repos I've Contributed To

<!-- BEGIN:contributed -->
<!-- END:contributed -->
"
            .to_string();
        } else {
            buf = std::fs::read_to_string(output)?;
        }
        let mut part_buf = String::new();
        render.render_created_repos(&mut part_buf, &created_repos);
        replace_template(&mut buf, "created_repos", &part_buf)?;
        part_buf.clear();
        render.render_contributed_repos(&mut part_buf, &contributed_repos, username);
        replace_template(&mut buf, "contributed", &part_buf)?;

        std::fs::write(output, buf)?;
    } else if output.ends_with(".svg") {
        todo!("SVG output is not implemented yet")
    } else {
        bail!("Unknown output format: {}", output);
    }

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
