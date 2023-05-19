use once_cell::sync::Lazy;
use prettytable::format::TableFormat;
use prettytable::{row, Table};

use crate::github::{ContributedRepo, Repository};

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
    fn render_contributed_repos(
        &self,
        output: &mut String,
        repos: &[ContributedRepo],
        author: &str,
    );
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
            let archived = if repo.is_archived { "~~" } else { "" };
            table.add_row(row![
                id + 1,
                format!("{archived}[{}]({}){archived}", repo.name(), repo.html_url()),
                repo.language(),
                repo.stargazer_count,
                repo.fork_count,
                repo.pushed_at
                    .map_or("N/A".to_string(), |dt| dt.format("%Y-%m-%d").to_string()),
            ]);
        }
        table.add_row(row![
            "Total",
            "",
            "",
            repos.iter().map(|x| x.stargazer_count).sum::<u32>(),
            repos.iter().map(|x| x.fork_count).sum::<u32>(),
            "",
        ]);

        output.push_str(table.to_string().as_str());
    }

    fn render_contributed_repos(
        &self,
        output: &mut String,
        repos: &[ContributedRepo],
        author: &str,
    ) {
        let mut table = Table::new();
        table.set_format(*MARKDOWN_TABLE);
        table.set_titles(row!["No.", "Name", "First PR", "Last PR", "PR Count"]);

        for (id, repo) in repos.iter().enumerate() {
            table.add_row(row![
                id + 1,
                format!(
                    "[{}](https://github.com/{})",
                    repo.full_name, repo.full_name
                ),
                format!(
                    "[{}]({})",
                    repo.first_pr.created_at.format("%Y-%m-%d"),
                    repo.first_pr.url.as_str()
                ),
                format!(
                    "[{}]({})",
                    repo.last_pr.created_at.format("%Y-%m-%d"),
                    repo.last_pr.url.as_str()
                ),
                format!(
                    "[{}](https://github.com/{}/pulls?q=is%3Apr+author%3A{})",
                    repo.pr_count, repo.full_name, author
                )
            ]);
        }
        table.add_row(row![
            "Total",
            "",
            "",
            "",
            repos.iter().map(|x| x.pr_count).sum::<u32>(),
        ]);
        output.push_str(table.to_string().as_str());
    }
}

pub struct SvgRenderer {}

impl SvgRenderer {
    pub fn new() -> Self {
        SvgRenderer {}
    }
}

impl Render for SvgRenderer {
    fn render_created_repos(&self, output: &mut String, repos: &[Repository]) {
        MarkdownRenderer::new().render_created_repos(output, repos);
    }

    fn render_contributed_repos(
        &self,
        output: &mut String,
        repos: &[ContributedRepo],
        author: &str,
    ) {
        MarkdownRenderer::new().render_contributed_repos(output, repos, author);
    }
}
