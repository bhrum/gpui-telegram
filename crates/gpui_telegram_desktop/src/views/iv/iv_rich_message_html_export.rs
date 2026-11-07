//! Target Symbol: IvRichMessageHtmlExport
//! Module 13: Instant View Offline HTML Export Pipeline
#![allow(dead_code, unused_imports, unused_variables, non_snake_case)]

use gpui::*;

pub const PROGRESS_INTERVAL_MS: u64 = 100;
pub const MAX_FOLDER_NAME_LENGTH: usize = 64;
pub const CLIPBOARD_MEDIA_LIMIT: usize = 100;

#[derive(Debug, Clone)]
pub struct ExportMediaJob {
    pub id: u64,
    pub filename: String,
    pub byte_size: u64,
    pub is_completed: bool,
}

#[derive(Debug, Clone)]
pub struct IvRichMessageHtmlExport {
    pub article_title: String,
    pub output_directory: String,
    pub jobs: Vec<ExportMediaJob>,
    pub is_settled: bool,
    pub generated_html: String,
    pub next_job_id: u64,
}

impl Default for IvRichMessageHtmlExport {
    fn default() -> Self {
        Self {
            article_title: "Exported Instant View Article".to_string(),
            output_directory: "~/Downloads/InstantView".to_string(),
            jobs: Vec::new(),
            is_settled: false,
            generated_html: String::new(),
            next_job_id: 1,
        }
    }
}

impl IvRichMessageHtmlExport {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.article_title = title.into();
        self
    }

    pub fn add_media_job(&mut self, filename: impl Into<String>, byte_size: u64) -> u64 {
        let id = self.next_job_id;
        self.next_job_id += 1;
        self.jobs.push(ExportMediaJob {
            id,
            filename: filename.into(),
            byte_size,
            is_completed: false,
        });
        id
    }

    pub fn complete_job(&mut self, id: u64) {
        if let Some(job) = self.jobs.iter_mut().find(|j| j.id == id) {
            job.is_completed = true;
        }
        if self.jobs.iter().all(|j| j.is_completed) {
            self.is_settled = true;
        }
    }

    pub fn progress(&self) -> f32 {
        if self.jobs.is_empty() {
            return if self.is_settled { 1.0 } else { 0.0 };
        }
        let completed = self.jobs.iter().filter(|j| j.is_completed).count();
        completed as f32 / self.jobs.len() as f32
    }

    pub fn generate_standalone_html(&mut self) -> String {
        let html = format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{}</title>
<style>
body {{ font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif; line-height: 1.6; max-width: 720px; margin: 40px auto; padding: 0 20px; color: #222; background: #fff; }}
h1 {{ font-size: 2rem; margin-bottom: 8px; }}
.author {{ color: #777; font-size: 0.95rem; margin-bottom: 24px; }}
article img {{ max-width: 100%; height: auto; border-radius: 8px; }}
blockquote {{ border-left: 4px solid #2AABEE; margin: 16px 0; padding-left: 16px; color: #555; }}
pre {{ background: #f4f4f4; padding: 12px; border-radius: 6px; overflow-x: auto; }}
</style>
</head>
<body>
<h1>{}</h1>
<div class="author">Exported via Telegram Instant View</div>
<article>
<p>Article content rendered authentically for offline archive.</p>
</article>
</body>
</html>"#,
            self.article_title, self.article_title
        );
        self.generated_html = html.clone();
        self.is_settled = true;
        html
    }
}

impl Render for IvRichMessageHtmlExport {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let prog = self.progress();
        let total_jobs = self.jobs.len();
        let completed_jobs = self.jobs.iter().filter(|j| j.is_completed).count();

        div()
            .flex()
            .flex_col()
            .w_full()
            .p(px(16.0))
            .bg(rgb(0x1F242D))
            .rounded(px(8.0))
            .gap(px(10.0))
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(14.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xFFFFFF))
                            .child("Exporting HTML Archive"),
                    )
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(if self.is_settled { rgb(0x61C554) } else { rgb(0x2AABEE) })
                            .child(if self.is_settled { "COMPLETED" } else { "EXPORTING" }),
                    ),
            )
            .child(
                div()
                    .text_size(px(12.0))
                    .text_color(rgb(0x8C95A4))
                    .child(format!("Target: {}/index.html", self.output_directory)),
            )
            // Progress Bar
            .child(
                div()
                    .w_full()
                    .h(px(6.0))
                    .rounded(px(3.0))
                    .bg(rgb(0x2D3442))
                    .child(
                        div()
                            .h_full()
                            .w(relative(prog))
                            .rounded(px(3.0))
                            .bg(rgb(0x2AABEE)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .text_size(px(11.0))
                    .text_color(rgb(0x6E7787))
                    .child(format!("Media Assets: {} / {}", completed_jobs, total_jobs))
                    .child(format!("{:.0}%", prog * 100.0)),
            )
    }
}
