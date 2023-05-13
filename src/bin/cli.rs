use anyhow::Result;
use prettytable::{format, row, Table};

use github_contrib_stats::github;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = clap::Command::new("github-contrib-stats")
        .version("0.1.0")
        .author("j178")
        .about("Get your GitHub contribution stats")
        .arg(
            clap::Arg::new("username")
                .short('u')
                .long("username")
                .value_name("USERNAME")
                .help("GitHub username")
                .required(true),
        )
        .arg(
            clap::Arg::new("token")
                .short('t')
                .long("token")
                .value_name("TOKEN")
                .env("GITHUB_TOKEN")
                .help("GitHub personal access token")
                .required(true),
        )
        .arg(
            clap::Arg::new("max-repos")
                .short('m')
                .long("max-repos")
                .value_name("MAX_REPOS")
                .help("Maximum number of repositories to show"),
        )
        .get_matches();

    let username = matches.get_one::<String>("username").unwrap();
    let max_repos = matches.get_one::<usize>("max-repos").map(|n| *n);
    let client = octocrab::OctocrabBuilder::new()
        .personal_token(matches.get_one::<String>("token").unwrap().clone())
        .build()?;

    let created_repos = github::get_created_repos(&client, username, max_repos).await?;
    let contributed_repos = github::get_contributed_repos(&client, username, max_repos).await?;

    let markdown_table = format::FormatBuilder::new()
        .column_separator('|')
        .separator(
            format::LinePosition::Title,
            format::LineSeparator::new('-', '|', '|', '|'),
        )
        .borders('|')
        .padding(1, 1)
        .build();

    let mut created_table = Table::new();
    created_table.set_format(markdown_table);
    created_table.set_titles(row![
        "No.",
        "Name",
        "Language",
        "Stars",
        "Forks",
        "Last Update"
    ]);

    for (id, repo) in created_repos.iter().enumerate() {
        created_table.add_row(row![
            id + 1,
            format!("[{}]({})", repo.name, repo.html_url.as_ref().unwrap()),
            repo.language
                .as_ref()
                .map_or("N/A".to_string(), |x| x.as_str().unwrap().to_string()),
            repo.stargazers_count.unwrap(),
            repo.forks_count.unwrap(),
            repo.updated_at.unwrap().format("%Y-%m-%d").to_string(),
        ]);
    }
    created_table.add_row(row![
        "Total",
        "",
        "",
        created_repos
            .iter()
            .map(|x| x.stargazers_count.unwrap())
            .sum::<u32>(),
        created_repos
            .iter()
            .map(|x| x.forks_count.unwrap())
            .sum::<u32>(),
        "",
    ]);
    created_table.printstd();

    println!();

    let mut contributed_table = Table::new();
    contributed_table.set_format(markdown_table);
    contributed_table.set_titles(row![
        "No.",
        "Name",
        "Stars",
        "First PR",
        "Last PR",
        "PR Count"
    ]);

    for (id, repo) in contributed_repos.iter().enumerate() {
        contributed_table.add_row(row![
            id + 1,
            format!("[{}](https://github.com/{})", repo.full_name, repo.full_name),
            repo.stargazers_count,
            format!("[{}]({})", repo.first_pr.created_at.format("%Y-%m-%d").to_string(), repo.first_pr.html_url.as_str()),
            format!("[{}]({})", repo.last_pr.created_at.format("%Y-%m-%d").to_string(), repo.last_pr.html_url.as_str()),
            repo.pr_count,
        ]);
    }
    contributed_table.add_row(row![
        "Total",
        "",
        "",
        "",
        "",
        contributed_repos.iter().map(|x| x.pr_count).sum::<u32>(),
    ]);
    contributed_table.printstd();

    Ok(())
}
