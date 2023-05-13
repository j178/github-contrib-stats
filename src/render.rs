use octocrab::models::Repository;
use once_cell::sync::Lazy;
use prettytable::format::TableFormat;
use prettytable::{row, Table};

use crate::github::ContributedRepo;

static MARKDOWN_TABLE: Lazy<TableFormat> = Lazy::new(|| {
    prettytable::format::FormatBuilder::new()
        .column_separator('|')
        .separator(
            prettytable::format::LinePosition::Title,
            prettytable::format::LineSeparator::new('-', '|', '|', '|'),
        )
        .borders('|')
        .padding(1, 1)
        .build()
});

pub trait Render {
    fn render_created_repos(&self, output: &mut String, repos: &[Repository]);
    fn render_contributed_repos(&self, output: &mut String, repos: &[ContributedRepo]);
}

pub struct MarkdownRenderer {}

impl MarkdownRenderer {
    pub fn new() -> Self {
        MarkdownRenderer {}
    }
}

impl Render for MarkdownRenderer {
    fn render_created_repos(&self, output: &mut String, repos: &[Repository]) {
        let mut table = Table::new();
        table.set_format(*MARKDOWN_TABLE);
        table.set_titles(row![
            "No.",
            "Name",
            "Language",
            "Stars",
            "Forks",
            "Last Update"
        ]);

        for (id, repo) in repos.iter().enumerate() {
            table.add_row(row![
                id + 1,
                format!("[{}]({})", repo.name, repo.html_url.as_ref().unwrap()),
                repo.language
                    .as_ref()
                    .map_or("N/A".to_string(), |x| x.as_str().unwrap().to_string()),
                repo.stargazers_count.unwrap(),
                repo.forks_count.unwrap(),
                repo.pushed_at.unwrap().format("%Y-%m-%d").to_string(),
            ]);
        }
        table.add_row(row![
            "Total",
            "",
            "",
            repos
                .iter()
                .map(|x| x.stargazers_count.unwrap())
                .sum::<u32>(),
            repos.iter().map(|x| x.forks_count.unwrap()).sum::<u32>(),
            "",
        ]);

        output.push_str(table.to_string().as_str());
    }

    fn render_contributed_repos(&self, output: &mut String, repos: &[ContributedRepo]) {
        let mut table = Table::new();
        table.set_format(*MARKDOWN_TABLE);
        table.set_titles(row![
            "No.", "Name", "Stars", "First PR", "Last PR", "PR Count"
        ]);

        for (id, repo) in repos.iter().enumerate() {
            table.add_row(row![
                id + 1,
                format!(
                    "[{}](https://github.com/{})",
                    repo.full_name, repo.full_name
                ),
                repo.stargazers_count,
                format!(
                    "[{}]({})",
                    repo.first_pr.created_at.format("%Y-%m-%d").to_string(),
                    repo.first_pr.html_url.as_str()
                ),
                format!(
                    "[{}]({})",
                    repo.last_pr.created_at.format("%Y-%m-%d").to_string(),
                    repo.last_pr.html_url.as_str()
                ),
                repo.pr_count,
            ]);
        }
        table.add_row(row![
            "Total",
            "",
            "",
            "",
            "",
            repos.iter().map(|x| x.pr_count).sum::<u32>(),
        ]);
        output.push_str(table.to_string().as_str());
    }
}
