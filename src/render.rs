use std::sync::LazyLock;

use prettytable::format::TableFormat;
use prettytable::{row, Table};

use crate::github::{ContributedRepo, Repository};

use svg::node::element::{Anchor, Definitions, Group, Rectangle, Style, Text};
use svg::Document;

use chrono::Local;

static MARKDOWN_TABLE: LazyLock<TableFormat> = LazyLock::new(|| {
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
    fn render_created_repos(&self, output: &mut String, repos: &[Repository], author: &str);
    fn render_contributed_repos(
        &self,
        output: &mut String,
        repos: &[ContributedRepo],
        author: &str,
    );
}

#[derive(Default)]
pub struct MarkdownRenderer {}

impl MarkdownRenderer {
    pub fn new() -> Self {
        MarkdownRenderer {}
    }
}

impl Render for MarkdownRenderer {
    fn render_created_repos(&self, output: &mut String, repos: &[Repository], _author: &str) {
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
                format!("{}", repo.stargazer_count),
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
            "",
            repos.iter().map(|x| x.pr_count).sum::<u32>(),
        ]);
        output.push_str(table.to_string().as_str());
    }
}

#[derive(Default)]
pub struct SvgRenderer {
    font_family: String,
    header_bg: String,
    row_bg_even: String,
    row_bg_odd: String,
    text_color: String,
    link_color: String,
    star_color: String,
    fork_color: String,
    total_row_bg: String,
    title_color: String,
}

impl SvgRenderer {
    pub fn new() -> Self {
        SvgRenderer {
            font_family: "Arial, sans-serif".to_string(),
            header_bg: "#0F172A".to_string(),    // Dark blue header
            row_bg_even: "#ffffff".to_string(),  // White
            row_bg_odd: "#F1F5F9".to_string(),   // Light cool gray
            text_color: "#334155".to_string(),   // Slate gray
            link_color: "#2563EB".to_string(),   // Bright blue for links
            star_color: "#EAB308".to_string(),   // Yellow for stars
            fork_color: "#10B981".to_string(),   // Emerald for forks
            total_row_bg: "#E2E8F0".to_string(), // Cool gray for total
            title_color: "#1E293B".to_string(),  // Dark slate for title
        }
    }

    fn create_title(&self, x: i32, y: i32, content: &str) -> Text {
        Text::new(content)
            .set("x", x)
            .set("y", y)
            .set("fill", self.title_color.as_str())
            .set("font-family", self.font_family.as_str())
            .set("font-size", 20)
            .set("font-weight", "bold")
    }

    fn create_subtitle(&self, x: i32, y: i32, content: &str) -> Text {
        Text::new(content)
            .set("x", x)
            .set("y", y)
            .set("fill", self.text_color.as_str())
            .set("font-family", self.font_family.as_str())
            .set("font-size", 12)
            .set("font-style", "italic")
            .set("text-anchor", "end") // Right align the text
    }

    fn get_language_icon(&self, language: &str) -> Option<(&'static str, &'static str)> {
        // (unicode_char, color)
        match language.to_lowercase().as_str() {
            "go" => Some(("\u{f3a8}", "#00ADD8")),         // fa-golang
            "rust" => Some(("\u{e07a}", "#DEA584")),       // fa-rust
            "python" => Some(("\u{f3e2}", "#3572A5")),     // fa-python
            "java" => Some(("\u{f4e4}", "#B07219")),       // fa-java
            "javascript" => Some(("\u{f3b8}", "#F1E05A")), // fa-js
            "typescript" => Some(("\u{e2b8}", "#3178C6")), // fa-ts
            "shell" => Some(("\u{f120}", "#89E051")),      // fa-terminal
            "c++" => Some(("\u{f61e}", "#F34B7D")),        // fa-cpp
            "c" => Some(("\u{e0d1}", "#555555")),          // fa-c
            "ruby" => Some(("\u{f3e9}", "#701516")),       // fa-ruby
            "php" => Some(("\u{f457}", "#4F5D95")),        // fa-php
            _ => None,
        }
    }

    fn create_language_icon(&self, x: i32, y: i32, language: &str) -> Option<Group> {
        self.get_language_icon(language).map(|(icon_char, color)| {
            Group::new().add(
                Text::new(icon_char)
                    .set("x", x)
                    .set("y", y)
                    .set("fill", color)
                    .set("font-family", "Font Awesome 6 Free") // Use FontAwesome font
                    .set("font-size", 16)
                    .set("dominant-baseline", "middle"),
            )
        })
    }

    // Helper to calculate name column width
    fn calculate_name_width(&self, repos: &[Repository]) -> i32 {
        let longest_name = repos
            .iter()
            .map(|repo| repo.name().len())
            .max()
            .unwrap_or(20);
        // Approximate width based on character count (assuming average char width is 8px)
        (longest_name as i32 * 8).max(200).min(400)
    }

    fn create_number_with_effect(&self, x: i32, y: i32, number: u32, color: &str) -> Group {
        // Calculate width based on number of digits (approximately 8px per digit)
        let num_str = number.to_string();
        let num_width = (num_str.len() as i32 * 8) + 8; // Add some padding

        let text = Text::new(num_str)
            .set("x", x)
            .set("y", y)
            .set("fill", color)
            .set("font-family", self.font_family.as_str())
            .set("font-size", 14)
            .set("dominant-baseline", "middle");

        let mut group = Group::new();

        // Add effects based on number size
        if number >= 1000 {
            // Stronger glow effect for 1000+
            group = group.add(
                Rectangle::new()
                    .set("x", x - 4)
                    .set("y", y - 10)
                    .set("width", num_width)
                    .set("height", 20)
                    .set("fill", color)
                    .set("opacity", 0.15)
                    .set("rx", 10),
            );
        } else if number >= 100 {
            // Subtle glow effect for 100+
            group = group.add(
                Rectangle::new()
                    .set("x", x - 4)
                    .set("y", y - 10)
                    .set("width", num_width)
                    .set("height", 20)
                    .set("fill", color)
                    .set("opacity", 0.08)
                    .set("rx", 10),
            );
        }

        // Add the text on top of the effect
        group.add(text)
    }

    fn create_text(&self, x: i32, y: i32, content: &str, color: &str) -> Text {
        Text::new(content)
            .set("x", x)
            .set("y", y)
            .set("fill", color)
            .set("font-family", self.font_family.as_str())
            .set("font-size", 14)
            .set("dominant-baseline", "middle") // Vertical alignment
    }

    fn create_link(&self, x: i32, y: i32, text: &str, url: &str) -> Anchor {
        Anchor::new()
            .set("href", url)
            .set("target", "_blank")
            .add(self.create_text(x, y, text, &self.link_color))
    }

    fn create_rect(&self, x: i32, y: i32, width: i32, height: i32, fill: &str) -> Rectangle {
        Rectangle::new()
            .set("x", x)
            .set("y", y)
            .set("width", width)
            .set("height", height)
            .set("fill", fill)
            .set("rx", 4) // Rounded corners
    }

    fn create_header_text(&self, x: i32, y: i32, content: &str) -> Text {
        Text::new(content)
            .set("x", x)
            .set("y", y)
            .set("fill", "#ffffff") // White text for header
            .set("font-family", self.font_family.as_str())
            .set("font-size", 14)
            .set("font-weight", "bold")
            .set("dominant-baseline", "middle")
    }
}

impl Render for SvgRenderer {
    fn render_created_repos(&self, output: &mut String, repos: &[Repository], author: &str) {
        let name_width = self.calculate_name_width(repos);
        let col_widths = [50, name_width, 120, 80, 80, 120];
        let row_height = 40;
        let header_height = 50;
        let total_width = col_widths.iter().sum::<i32>();
        let total_height = header_height + (repos.len() as i32 + 2) * row_height;

        let mut document = Document::new()
            .set("width", total_width)
            .set("height", total_height)
            .set("style", "background-color: white");

        // Add defs with style
        document = document.add(
            Definitions::new().add(
                Style::new(
                    "@import url('https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.0.0/css/all.min.css');"
                )
            )
        );

        // Title and date on the same line
        document =
            document.add(self.create_title(10, 30, &format!("Repositories {author} Created")));

        let current_date = Local::now().format("%Y-%m-%d").to_string();
        document = document.add(
            self.create_subtitle(total_width - 10, 30, &current_date), // Align to right margin
        );

        // Header row
        document = document.add(self.create_rect(
            0,
            header_height,
            total_width,
            row_height,
            &self.header_bg,
        ));

        // Header texts
        let headers = ["No.", "Name", "Language", "Stars", "Forks", "Last Update"];
        let mut x = 10;
        for (i, header) in headers.iter().enumerate() {
            document =
                document.add(self.create_header_text(x, header_height + row_height / 2, header));
            x += col_widths[i];
        }

        // Data rows
        let mut y = header_height + row_height;
        for (id, repo) in repos.iter().enumerate() {
            let bg_color = if id % 2 == 0 {
                &self.row_bg_even
            } else {
                &self.row_bg_odd
            };

            // Row background
            document = document.add(self.create_rect(0, y, total_width, row_height, bg_color));

            let mut x = 10;
            // No.
            document = document.add(self.create_text(
                x,
                y + row_height / 2,
                &(id + 1).to_string(),
                &self.text_color,
            ));

            // Name with link
            x += col_widths[0];
            document = document.add(self.create_link(
                x,
                y + row_height / 2,
                &repo.name(),
                &repo.html_url(),
            ));

            // Language
            x += col_widths[1];
            let text_x = x + 25;
            if let Some(lang_icon) =
                self.create_language_icon(x, y + row_height / 2, &repo.language())
            {
                document = document.add(lang_icon);
            }
            document = document.add(self.create_text(
                text_x,
                y + row_height / 2,
                &repo.language(),
                &self.text_color,
            ));

            // Stars
            x += col_widths[2];
            if repo.stargazer_count > 0 {
                document = document.add(self.create_number_with_effect(
                    x,
                    y + row_height / 2,
                    repo.stargazer_count,
                    &self.star_color,
                ));
            } else {
                document =
                    document.add(self.create_text(x, y + row_height / 2, "0", &self.text_color));
            }

            // Forks
            x += col_widths[3];
            if repo.fork_count > 0 {
                document = document.add(self.create_number_with_effect(
                    x,
                    y + row_height / 2,
                    repo.fork_count,
                    &self.fork_color,
                ));
            } else {
                document =
                    document.add(self.create_text(x, y + row_height / 2, "0", &self.text_color));
            }

            // Last Update
            x += col_widths[4];
            let date = repo
                .pushed_at
                .map_or("N/A".to_string(), |dt| dt.format("%Y-%m-%d").to_string());
            document =
                document.add(self.create_text(x, y + row_height / 2, &date, &self.text_color));

            y += row_height;
        }

        // Total row
        document =
            document.add(self.create_rect(0, y, total_width, row_height, &self.total_row_bg));

        document = document.add(
            self.create_text(10, y + row_height / 2, "Total", &self.text_color)
                .set("font-weight", "bold"),
        );

        let total_stars: u32 = repos.iter().map(|x| x.stargazer_count).sum();
        let total_forks: u32 = repos.iter().map(|x| x.fork_count).sum();

        let x_stars = 10 + col_widths[0] + col_widths[1] + col_widths[2];
        document = document.add(
            self.create_text(
                x_stars,
                y + row_height / 2,
                &total_stars.to_string(),
                &self.star_color,
            )
            .set("font-weight", "bold"),
        );

        let x_forks = x_stars + col_widths[3];
        document = document.add(
            self.create_text(
                x_forks,
                y + row_height / 2,
                &total_forks.to_string(),
                &self.fork_color,
            )
            .set("font-weight", "bold"),
        );

        output.push_str(&document.to_string());
    }

    fn render_contributed_repos(
        &self,
        output: &mut String,
        repos: &[ContributedRepo],
        author: &str,
    ) {
        let name_width = repos
            .iter()
            .map(|repo| repo.full_name.len())
            .max()
            .unwrap_or(20) as i32
            * 8;
        let name_width = name_width.max(200).min(400);
        let col_widths = [50, name_width, 80, 120, 120, 100];
        let row_height = 40;
        let header_height = 50;
        let total_width = col_widths.iter().sum::<i32>();
        let total_height = header_height + (repos.len() as i32 + 2) * row_height;

        let mut document = Document::new()
            .set("width", total_width)
            .set("height", total_height)
            .set("style", "background-color: white; @import url('https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.0.0/css/all.min.css');");

        // Title and date on the same line
        document = document.add(self.create_title(
            10,
            30,
            &format!("Repositories {author} Contributed To"),
        ));

        let current_date = Local::now().format("%Y-%m-%d").to_string();
        document = document.add(
            self.create_subtitle(total_width - 10, 30, &current_date), // Align to right margin
        );

        // Header row
        document = document.add(self.create_rect(
            0,
            header_height,
            total_width,
            row_height,
            &self.header_bg,
        ));

        // Header texts
        let headers = ["No.", "Name", "Stars", "First PR", "Last PR", "PR Count"];
        let mut x = 10;
        for (i, header) in headers.iter().enumerate() {
            document =
                document.add(self.create_header_text(x, header_height + row_height / 2, header));
            x += col_widths[i];
        }

        // Data rows
        let mut y = header_height + row_height;
        for (id, repo) in repos.iter().enumerate() {
            let bg_color = if id % 2 == 0 {
                &self.row_bg_even
            } else {
                &self.row_bg_odd
            };

            document = document.add(self.create_rect(0, y, total_width, row_height, bg_color));

            let mut x = 10;
            // No.
            document = document.add(self.create_text(
                x,
                y + row_height / 2,
                &(id + 1).to_string(),
                &self.text_color,
            ));

            // Name with link
            x += col_widths[0];
            document = document.add(self.create_link(
                x,
                y + row_height / 2,
                &repo.full_name,
                &format!("https://github.com/{}", repo.full_name),
            ));

            // Stars
            x += col_widths[1];
            if repo.stargazer_count > 0 {
                document = document.add(self.create_number_with_effect(
                    x,
                    y + row_height / 2,
                    repo.stargazer_count,
                    &self.star_color,
                ));
            } else {
                document =
                    document.add(self.create_text(x, y + row_height / 2, "0", &self.text_color));
            }

            // First PR
            x += col_widths[2];
            document = document.add(self.create_link(
                x,
                y + row_height / 2,
                &repo.first_pr.created_at.format("%Y-%m-%d").to_string(),
                &repo.first_pr.url,
            ));

            // Last PR
            x += col_widths[3];
            document = document.add(self.create_link(
                x,
                y + row_height / 2,
                &repo.last_pr.created_at.format("%Y-%m-%d").to_string(),
                &repo.last_pr.url,
            ));

            // PR Count
            x += col_widths[4];
            document = document.add(self.create_link(
                x,
                y + row_height / 2,
                &repo.pr_count.to_string(),
                &format!(
                    "https://github.com/{}/pulls?q=is%3Apr+author%3A{}",
                    repo.full_name, author
                ),
            ));

            y += row_height;
        }

        // Total row
        document =
            document.add(self.create_rect(0, y, total_width, row_height, &self.total_row_bg));

        document = document.add(
            self.create_text(10, y + row_height / 2, "Total", &self.text_color)
                .set("font-weight", "bold"),
        );

        let total_prs: u32 = repos.iter().map(|x| x.pr_count).sum();
        let x_prs =
            10 + col_widths[0] + col_widths[1] + col_widths[2] + col_widths[3] + col_widths[4];
        document = document.add(
            self.create_text(
                x_prs,
                y + row_height / 2,
                &total_prs.to_string(),
                &self.text_color,
            )
            .set("font-weight", "bold"),
        );

        output.push_str(&document.to_string());
    }
}
