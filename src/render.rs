use std::sync::LazyLock;

use prettytable::format::TableFormat;
use prettytable::{row, Table};

use crate::github::{ContributedRepo, Repository};

use svg::node::element::{Anchor, Definitions, Group, Path, Rectangle, Text};
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
        // Font Awesome paths and colors for different languages
        let (color, path) = match language.to_lowercase().as_str() {
            "go" => ("#00ADD8", "M400.1 194.8C389.2 197.6 380.2 199.1 371 202.4C363.7 204.3 356.3 206.3 347.8 208.5L347.2 208.6C343 209.8 342.6 209.9 338.7 205.4C334 200.1 330.6 196.7 324.1 193.5C304.4 183.9 285.4 186.7 267.7 198.2C246.5 211.9 235.6 232.2 235.9 257.4C236.2 282.4 253.3 302.9 277.1 306.3C299.1 309.1 316.9 301.7 330.9 285.8C333 283.2 334.9 280.5 337 277.5V277.5L337 277.5C337.8 276.5 338.5 275.4 339.3 274.2H279.2C272.7 274.2 271.1 270.2 273.3 264.9C277.3 255.2 284.8 239 289.2 230.9C290.1 229.1 292.3 225.1 296.1 225.1H397.2C401.7 211.7 409 198.2 418.8 185.4C441.5 155.5 468.1 139.9 506 133.4C537.8 127.8 567.7 130.9 594.9 149.3C619.5 166.1 634.7 188.9 638.8 218.8C644.1 260.9 631.9 295.1 602.1 324.4C582.4 345.3 557.2 358.4 528.2 364.3C522.6 365.3 517.1 365.8 511.7 366.3C508.8 366.5 506 366.8 503.2 367.1C474.9 366.5 449 358.4 427.2 339.7C411.9 326.4 401.3 310.1 396.1 291.2C392.4 298.5 388.1 305.6 382.1 312.3C360.5 341.9 331.2 360.3 294.2 365.2C263.6 369.3 235.3 363.4 210.3 344.7C187.3 327.2 174.2 304.2 170.8 275.5C166.7 241.5 176.7 210.1 197.2 184.2C219.4 155.2 248.7 136.8 284.5 130.3C313.8 124.1 341.8 128.4 367.1 145.6C383.6 156.5 395.4 171.4 403.2 189.5C405.1 192.3 403.8 193.9 400.1 194.8z"),
            "rust" => ("#DEA584", "M508.5 249.8 486.7 236.2c-.2-2-.3-3.9-.6-5.9l18.7-17.5a7.4 7.4 0 0 0 -2.4-12.3l-24-9c-.5-1.9-1.1-3.8-1.7-5.6l15-20.8a7.4 7.4 0 0 0 -4.8-11.5l-25.4-4.2c-.9-1.7-1.8-3.5-2.7-5.2l10.7-23.4a7.4 7.4 0 0 0 -7-10.4l-25.8 .9q-1.8-2.2-3.6-4.4L439 81.8A7.4 7.4 0 0 0 430.2 73L405 78.9q-2.2-1.8-4.4-3.6l.9-25.8a7.4 7.4 0 0 0 -10.4-7L367.7 53.2c-1.7-.9-3.4-1.8-5.2-2.7L358.4 25.1a7.4 7.4 0 0 0 -11.5-4.8L326 35.3c-1.9-.6-3.8-1.1-5.6-1.7l-9-24a7.4 7.4 0 0 0 -12.3-2.4l-17.5 18.7c-2-.2-3.9-.4-5.9-.6L262.3 3.5a7.4 7.4 0 0 0 -12.5 0L236.2 25.3c-2 .2-3.9 .3-5.9 .6L212.9 7.1a7.4 7.4 0 0 0 -12.3 2.4l-9 24c-1.9 .6-3.8 1.1-5.7 1.7l-20.8-15a7.4 7.4 0 0 0 -11.5 4.8l-4.2 25.4c-1.7 .9-3.5 1.8-5.2 2.7L120.9 42.6a7.4 7.4 0 0 0 -10.4 7l.9 25.8c-1.5 1.2-3 2.4-4.4 3.6L81.8 73A7.4 7.4 0 0 0 73 81.8L78.9 107c-1.2 1.5-2.4 2.9-3.6 4.4l-25.8-.9a7.4 7.4 0 0 0 -6.4 3.3 7.4 7.4 0 0 0 -.6 7.1l10.7 23.4c-.9 1.7-1.8 3.4-2.7 5.2L25.1 153.6a7.4 7.4 0 0 0 -4.8 11.5l15 20.8c-.6 1.9-1.1 3.8-1.7 5.7l-24 9a7.4 7.4 0 0 0 -2.4 12.3l18.7 17.5c-.2 2-.4 3.9-.6 5.9L3.5 249.8a7.4 7.4 0 0 0 0 12.5L25.3 275.8c.2 2 .3 3.9 .6 5.9L7.1 299.1a7.4 7.4 0 0 0 2.4 12.3l24 9c.6 1.9 1.1 3.8 1.7 5.7l-15 20.8a7.4 7.4 0 0 0 4.8 11.5l25.4 4.2c.9 1.7 1.8 3.5 2.7 5.1L42.6 391.1a7.4 7.4 0 0 0 .6 7.1 7.1 7.1 0 0 0 6.4 3.3l25.8-.9q1.8 2.2 3.6 4.4L73 430.2A7.4 7.4 0 0 0 81.8 439L107 433.1q2.2 1.8 4.4 3.6l-.9 25.8a7.4 7.4 0 0 0 10.4 7l23.4-10.7c1.7 .9 3.4 1.8 5.1 2.7l4.2 25.4a7.3 7.3 0 0 0 11.5 4.8l20.8-15c1.9 .6 3.8 1.1 5.7 1.7l9 24a7.4 7.4 0 0 0 12.3 2.4l17.5-18.7c2 .2 3.9 .4 5.9 .6l13.5 21.8a7.4 7.4 0 0 0 12.5 0l13.5-21.8c2-.2 3.9-.3 5.9-.6l17.5 18.7a7.4 7.4 0 0 0 12.3-2.4l9-24c1.9-.6 3.8-1.1 5.7-1.7l20.8 15a7.3 7.3 0 0 0 11.5-4.8l4.2-25.4c1.7-.9 3.5-1.8 5.2-2.7l23.4 10.7a7.4 7.4 0 0 0 10.4-7l-.9-25.8q2.2-1.8 4.4-3.6L430.2 439a7.4 7.4 0 0 0 8.8-8.8L433.1 405q1.8-2.2 3.6-4.4l25.8 .9a7.2 7.2 0 0 0 6.4-3.3 7.4 7.4 0 0 0 .6-7.1L458.8 367.7c.9-1.7 1.8-3.4 2.7-5.2l25.4-4.2a7.4 7.4 0 0 0 4.8-11.5l-15-20.8c.6-1.9 1.1-3.8 1.7-5.7l24-9a7.4 7.4 0 0 0 2.4-12.3l-18.7-17.5c.2-2 .4-3.9 .6-5.9l21.8-13.5a7.4 7.4 0 0 0 0-12.5zm-151 129.1A13.9 13.9 0 0 0 341 389.5l-7.6 35.7A187.5 187.5 0 0 1 177 424.4l-7.6-35.7a13.9 13.9 0 0 0 -16.5-10.7l-31.5 6.8a187.4 187.4 0 0 1 -16.3-19.2H258.3c1.7 0 2.9-.3 2.9-1.9V309.6c0-1.6-1.2-1.9-2.9-1.9H213.5l.1-34.4H262c4.4 0 23.7 1.3 29.8 25.9 1.9 7.6 6.2 32.1 9.1 40 2.9 8.8 14.6 26.5 27.1 26.5H407a187.3 187.3 0 0 1 -17.3 20.1zm25.8 34.5A15.2 15.2 0 1 1 368 398.1h.4A15.2 15.2 0 0 1 383.2 413.3zm-225.6-.7a15.2 15.2 0 1 1 -15.3-15.3h.5A15.3 15.3 0 0 1 157.6 412.6zM69.6 234.2l32.8-14.6a13.9 13.9 0 0 0 7.1-18.3L102.7 186h26.6V305.7H75.7A187.7 187.7 0 0 1 69.6 234.2zM58.3 198.1a15.2 15.2 0 0 1 15.2-15.3H74a15.2 15.2 0 1 1 -15.7 15.2zm155.2 24.5 .1-35.3h63.3c3.3 0 23.1 3.8 23.1 18.6 0 12.3-15.2 16.7-27.7 16.7zM399 306.7c-9.8 1.1-20.6-4.1-22-10.1-5.8-32.5-15.4-39.4-30.6-51.4 18.9-12 38.5-29.6 38.5-53.3 0-25.5-17.5-41.6-29.4-49.5-16.8-11-35.3-13.2-40.3-13.2H116.3A187.5 187.5 0 0 1 221.2 70.1l23.5 24.6a13.8 13.8 0 0 0 19.6 .4l26.3-25a187.5 187.5 0 0 1 128.4 91.4l-18 40.6A14 14 0 0 0 408 220.4l34.6 15.3a187.1 187.1 0 0 1 .4 32.5H423.7c-1.9 0-2.7 1.3-2.7 3.1v8.8C421 301 409.3 305.6 399 306.7zM240 60.2A15.2 15.2 0 0 1 255.2 45h.5A15.2 15.2 0 1 1 240 60.2zM436.8 214a15.2 15.2 0 1 1 0-30.5h.4a15.2 15.2 0 0 1 -.4 30.5z"),
            "python" => ("#3776AB", "M439.8 200.5c-7.7-30.9-22.3-54.2-53.4-54.2h-40.1v47.4c0 36.8-31.2 67.8-66.8 67.8H172.7c-29.2 0-53.4 25-53.4 54.3v101.8c0 29 25.2 46 53.4 54.3 33.8 9.9 66.3 11.7 106.8 0 26.9-7.8 53.4-23.5 53.4-54.3v-40.7H226.2v-13.6h160.2c31.1 0 42.6-21.7 53.4-54.2 11.2-33.5 10.7-65.7 0-108.6zM286.2 404c11.1 0 20.1 9.1 20.1 20.3 0 11.3-9 20.4-20.1 20.4-11 0-20.1-9.2-20.1-20.4.1-11.3 9.1-20.3 20.1-20.3zM167.8 248.1h106.8c29.7 0 53.4-24.5 53.4-54.3V91.9c0-29-24.4-50.7-53.4-55.6-35.8-5.9-74.7-5.6-106.8.1-45.2 8-53.4 24.7-53.4 55.6v40.7h106.9v13.6h-147c-31.1 0-58.3 18.7-66.8 54.2-9.8 40.7-10.2 66.1 0 108.6 7.6 31.6 25.7 54.2 56.8 54.2H101v-48.8c0-35.3 30.5-66.4 66.8-66.4zm-6.7-142.6c-11.1 0-20.1-9.1-20.1-20.3.1-11.3 9-20.4 20.1-20.4 11 0 20.1 9.2 20.1 20.4s-9 20.3-20.1 20.3z"),
            "javascript" => ("#F7DF1E", "M0 32v448h448V32H0zm243.8 349.4c0 43.6-25.6 63.5-62.9 63.5-33.7 0-53.2-17.4-63.2-38.5l34.3-20.7c6.6 11.7 12.6 21.6 27.1 21.6 13.8 0 22.6-5.4 22.6-26.5V237.7h42.1v143.7zm99.6 63.5c-39.1 0-64.4-18.6-76.7-43l34.3-19.8c9 14.7 20.8 25.6 41.5 25.6 17.4 0 28.6-8.7 28.6-20.8 0-14.4-11.4-19.5-30.7-28l-10.5-4.5c-30.4-12.9-50.5-29.2-50.5-63.5 0-31.6 24.1-55.6 61.6-55.6 26.8 0 46 9.3 59.8 33.7L368 290c-7.2-12.9-15-18-27.1-18-12.3 0-20.1 7.8-20.1 18 0 12.6 7.8 17.7 25.9 25.6l10.5 4.5c35.8 15.3 55.9 31 55.9 66.2 0 37.8-29.8 58.6-69.7 58.6z"),
            "typescript" => ("#3178C6", "M0 0h512v512H0V0zm281.6 312.4v50.7h-50.4v-50.7h50.4zm93.3-131.2v182h-50.4v-182h50.4zm-186.6 0v182h-50.4v-182h50.4z"),
            "java" => ("#007396", "M253.03 94.41c-14.86 13.21-23.32 31.23-23.32 50.03 0 37.68 40.98 73.36 40.98 73.36s-46.76-24.01-46.76-72.7c0-24.13 12.85-38.13 29.1-50.69zm70.31 251.95c12.85-8.35 32.79-15.61 32.79-15.61s-51.9 9.01-101.13 12.85c-60.25 4.68-125.14 5.34-159.25 1.5C49.02 337.44 253.03 298.3 253.03 298.3s-138.65 3.84-229.93 11.52C9.88 311.48 0 316.16 0 316.16c89.35 22.66 299.91 12.85 323.34 30.12zm-4.01-48.73c-19.54-14.86-93.44-28.38-93.44-28.38s48.9 1.67 94.1 8.35c45.2 6.68 89.35 21.54 89.35 30.79 0 9.25-10.01 14.2-26.95 14.2-38.13 0-43.47-10.1-63.06-24.96zm-8.35-40.05c-18.38-8.35-59.59-16.03-59.59-16.03s31.88 1.67 61.65 6.35c29.77 4.68 58.88 14.86 58.88 21.54s-6.68 10.01-17.71 10.01c-24.96 0-24.96-13.52-43.23-21.87zm-20.37-80.43s27.44 27.44 27.44 40.05c0 12.61-7.26 20.71-7.26 20.71s11.27-5.84 11.27-22.16c0-16.32-31.45-38.6-31.45-38.6zm-62.23-89.35c0 0-25.11 22.16-25.11 46.1 0 23.94 13.69 41.31 30.79 53.24-17.1-11.93-39.93-32.13-39.93-59.59 0-27.46 34.25-39.75 34.25-39.75zm146.16 250.11c6.68-4.18 17.05-7.85 17.05-7.85s-27.02 4.51-52.65 6.43c-31.37 2.34-65.15 2.67-82.92.75-27.61-2.84 131.97-22.49 131.97-22.49s-72.2 1.92-119.72 5.76c-6.93.75-12.11 3.01-12.11 3.01 46.52 11.35 156.17 6.43 168.38 15.1z"),
            "c++" => ("#00599C", "M302.107 258.262c2.401-4.159 3.893-8.845 3.893-13.053v-64.813c0-5.062-1.035-9.723-2.751-13.831l-148.944 91.697h147.802zm-160.646-92.436c-2.401 4.159-3.893 8.845-3.893 13.053v64.813c0 5.062 1.035 9.723 2.751 13.831l148.944-91.697h-147.802zm-21.647 102.436h147.802l-148.944-91.697c-2.401 4.159-3.893 8.845-3.893 13.053v64.813c0 5.062 1.035 9.723 2.751 13.831zm160.646-92.436h-147.802l148.944 91.697c2.401-4.159 3.893-8.845 3.893-13.053v-64.813c0-5.062-1.035-9.723-2.751-13.831z"),
            "c#" => ("#239120", "M82.637 225.014L0 174.228l82.637-50.786 82.637 50.786-82.637 50.786zm347.725 0l-82.637-50.786 82.637-50.786 82.637 50.786-82.637 50.786zM256 389.402l-82.637-50.786L256 287.83l82.637 50.786L256 389.402zm0-328.804l82.637 50.786L256 161.97l-82.637-50.786L256 60.598z"),
            "ruby" => ("#CC342D", "M419.8 168.9l-138.3 92.2L419.8 353.3v-184.4zm-327.6 0v184.4l138.3-92.2-138.3-92.2zm163.8 117.3l-163.8 109.2h327.6l-163.8-109.2zm0-42.9l163.8-109.2H92.2l163.8 109.2z"),
            "swift" => ("#FA7343", "M473.2 128.3c-19.8-19.8-43.5-35.4-69.5-46.2-27.4-11.4-56.4-17.1-86.2-17.1-29.8 0-58.9 5.8-86.2 17.1-26 10.8-49.7 26.4-69.5 46.2s-35.4 43.5-46.2 69.5c-11.4 27.4-17.1 56.4-17.1 86.2 0 29.8 5.8 58.9 17.1 86.2 10.8 26 26.4 49.7 46.2 69.5s43.5 35.4 69.5 46.2c27.4 11.4 56.4 17.1 86.2 17.1 29.8 0 58.9-5.8 86.2-17.1 26-10.8 49.7-26.4 69.5-46.2s35.4-43.5 46.2-69.5c11.4-27.4 17.1-56.4 17.1-86.2 0-29.8-5.8-58.9-17.1-86.2-10.8-26-26.4-49.7-46.2-69.5zm-20.5 225.4c-7.6 18.2-18.6 34.6-32.8 48.8s-30.6 25.2-48.8 32.8c-19 7.9-39.1 11.9-59.8 11.9s-40.9-4-59.8-11.9c-18.2-7.6-34.6-18.6-48.8-32.8s-25.2-30.6-32.8-48.8c-7.9-19-11.9-39.1-11.9-59.8s4-40.9 11.9-59.8c7.6-18.2 18.6-34.6 32.8-48.8s30.6-25.2 48.8-32.8c19-7.9 39.1-11.9 59.8-11.9s40.9 4 59.8 11.9c18.2 7.6 34.6 18.6 48.8 32.8s25.2 30.6 32.8 48.8c7.9 19 11.9 39.1 11.9 59.8s-4 40.9-11.9 59.8z"),
            "kotlin" => ("#7F52FF", "M256 32L32 256l224 224 224-224L256 32zm0 48.4L432.8 256 256 431.6 79.2 256 256 80.4z"),
            "php" => ("#777BB4", "M107.3 293.3c-11.8 0-21.3-3.2-28.5-9.5-7.2-6.3-10.8-15.1-10.8-26.2 0-11.2 3.6-19.9 10.8-26.2 7.2-6.3 16.7-9.5 28.5-9.5 8.5 0 15.7 1.4 21.6 4.3 5.9 2.9 10.3 6.9 13.2 12.1l-14.7 8.4c-1.8-3.2-4.1-5.6-7-7.2-2.9-1.6-6.4-2.4-10.5-2.4-6.1 0-10.8 1.8-14.2 5.4-3.4 3.6-5.1 8.7-5.1 15.2 0 6.5 1.7 11.5 5.1 15.2 3.4 3.6 8.2 5.4 14.2 5.4 4.1 0 7.6-0.8 10.5-2.4 2.9-1.6 5.2-4 7-7.2l14.7 8.4c-2.9 5.2-7.3 9.2-13.2 12.1-5.9 2.9-13.1 4.3-21.6 4.3zm83.7 0c-11.8 0-21.3-3.2-28.5-9.5-7.2-6.3-10.8-15.1-10.8-26.2 0-11.2 3.6-19.9 10.8-26.2 7.2-6.3 16.7-9.5 28.5-9.5 8.5 0 15.7 1.4 21.6 4.3 5.9 2.9 10.3 6.9 13.2 12.1l-14.7 8.4c-1.8-3.2-4.1-5.6-7-7.2-2.9-1.6-6.4-2.4-10.5-2.4-6.1 0-10.8 1.8-14.2 5.4-3.4 3.6-5.1 8.7-5.1 15.2 0 6.5 1.7 11.5 5.1 15.2 3.4 3.6 8.2 5.4 14.2 5.4 4.1 0 7.6-0.8 10.5-2.4 2.9-1.6 5.2-4 7-7.2l14.7 8.4c-2.9 5.2-7.3 9.2-13.2 12.1-5.9 2.9-13.1 4.3-21.6 4.3zm83.7 0c-11.8 0-21.3-3.2-28.5-9.5-7.2-6.3-10.8-15.1-10.8-26.2 0-11.2 3.6-19.9 10.8-26.2 7.2-6.3 16.7-9.5 28.5-9.5 8.5 0 15.7 1.4 21.6 4.3 5.9 2.9 10.3 6.9 13.2 12.1l-14.7 8.4c-1.8-3.2-4.1-5.6-7-7.2-2.9-1.6-6.4-2.4-10.5-2.4-6.1 0-10.8 1.8-14.2 5.4-3.4 3.6-5.1 8.7-5.1 15.2 0 6.5 1.7 11.5 5.1 15.2 3.4 3.6 8.2 5.4 14.2 5.4 4.1 0 7.6-0.8 10.5-2.4 2.9-1.6 5.2-4 7-7.2l14.7 8.4c-2.9 5.2-7.3 9.2-13.2 12.1-5.9 2.9-13.1 4.3-21.6 4.3z"),
            "scala" => ("#DC322F", "M256 416c141.4 0 256-60.6 256-135.3v-49.4c0 74.7-114.6 135.3-256 135.3S0 305.9 0 231.3v49.4C0 355.4 114.6 416 256 416zm0-123.3c141.4 0 256-60.6 256-135.3v-49.4c0 74.7-114.6 135.3-256 135.3S0 182.6 0 108v49.4c0 74.7 114.6 135.3 256 135.3z"),
            "r" => ("#276DC3", "M256 32C132.3 32 32 132.3 32 256s100.3 224 224 224 224-100.3 224-224S379.7 32 256 32zm128.5 300.8c-12.7 12.7-29.5 19.7-47.4 19.7H176.9c-17.9 0-34.7-7-47.4-19.7-12.7-12.7-19.7-29.5-19.7-47.4V176.9c0-17.9 7-34.7 19.7-47.4 12.7-12.7 29.5-19.7 47.4-19.7h160.2c17.9 0 34.7 7 47.4 19.7 12.7 12.7 19.7 29.5 19.7 47.4v108.5c0 17.9-7 34.7-19.7 47.4z"),
            "dart" => ("#00B4AB", "M378.6 78.9c-2.8-.1-5.6-.2-8.5-.2l-264.1 0 143.2-72C256.6 2.3 268 0 279.6 0c13.5 0 29.4 9.2 37 16.8l62 62zM107.3 96.5l262.8 0c16 0 25.4 1.4 35.4 9.3L512 212.2 512 421l-79.3 .7L107.3 96.5zM96.5 373l0-262.2L420.3 434.6l.7 77.4-212.2 0-98.1-98.2 0 0C99.4 402.5 96.5 398.5 96.5 373zM78.7 105.3l0 267.7c0 3.3 .1 6.3 .2 9.1l-62-62C6.5 309.3 0 294.3 0 279.6c0-6.8 3.9-17.5 6.7-23.6l72-150.7z"),
            "c" => ("#555555", "M329.1 142.9c-62.5-62.5-155.8-62.5-218.3 0s-62.5 163.8 0 226.3s155.8 62.5 218.3 0c12.5-12.5 32.8-12.5 45.3 0s12.5 32.8 0 45.3c-87.5 87.5-221.3 87.5-308.8 0s-87.5-229.3 0-316.8s221.3-87.5 308.8 0c12.5 12.5 12.5 32.8 0 45.3s-32.8 12.5-45.3 0z"),
            _ => return None,
        };
        Some((color, path))
    }

    fn create_language_icon(&self, x: i32, y: i32, language: &str) -> Option<Group> {
        self.get_language_icon(language).map(|(color, _)| {
            let icon_id = format!("lang-{}", language.to_lowercase());
            Group::new()
                .set(
                    "transform",
                    format!("translate({}, {}) scale(0.025)", x, y - 6),
                )
                .add(
                    svg::node::element::Use::new()
                        .set("href", format!("#{}", icon_id))
                        .set("fill", color),
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
        (longest_name as i32 * 8).clamp(200, 400)
    }

    fn create_language_defs(&self, languages: &[&str]) -> Definitions {
        let mut defs = Definitions::new();

        for lang in languages {
            let lang = lang.to_lowercase();
            if let Some((_, path)) = self.get_language_icon(&lang) {
                defs = defs.add(
                    Path::new()
                        .set("id", format!("lang-{}", lang))
                        .set("d", path)
                        .set("viewBox", "0 0 512 512"),
                );
            }
        }

        defs
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

        let languages = repos.iter().map(|x| x.language()).collect::<Vec<_>>();
        // Add definitions with all language icons
        document = document.add(self.create_language_defs(&languages));

        // Title and date on the same line
        document = document.add(self.create_title(10, 30, &format!("Repos Created by {author}")));

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
                repo.name(),
                &repo.html_url(),
            ));

            // Language
            x += col_widths[1];
            let text_x = x + 25;
            if let Some(lang_icon) =
                self.create_language_icon(x, y + row_height / 2, repo.language())
            {
                document = document.add(lang_icon);
            }
            document = document.add(self.create_text(
                text_x,
                y + row_height / 2,
                repo.language(),
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
        let name_width = name_width.clamp(200, 400);
        let col_widths = [50, name_width, 80, 120, 120, 100];
        let row_height = 40;
        let header_height = 50;
        let total_width = col_widths.iter().sum::<i32>();
        let total_height = header_height + (repos.len() as i32 + 2) * row_height;

        let mut document = Document::new()
            .set("width", total_width)
            .set("height", total_height)
            .set("style", "background-color: white");

        // Title and date on the same line
        document =
            document.add(self.create_title(10, 30, &format!("Repos {author} Contributed To")));

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
