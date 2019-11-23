#![recursion_limit = "1000"]
pub mod view;

use anyhow::Error;
use chrono::NaiveDate;
use rodin::{ContentGenerator, ContentParser, File, Generator, Page, Rodin, SiteVariables};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use typed_html::{html, text};

pub fn run() {
    let rodin: Rodin<PageData, PageMetadata> = Rodin::new(".", "../pages_target")
        .register_parser(Box::new(BlogPostParser {}))
        .register_generator(Generator::Content(Box::new(BlogPostGenerator {})));

    rodin.build().expect("failed to build rodin");
}

#[derive(Clone, Debug)]
pub enum PageData {
    Post(PostData),
}

#[derive(Clone, Debug)]
pub struct PostData {
    pub title: String,
    pub date: NaiveDate,
    pub content: String,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PostFrontMatter {
    pub title: String,
    pub tags: String,
}

#[derive(Clone, Debug)]
pub struct PageMetadata {}

pub struct BlogPostParser {}

impl ContentParser<PageData> for BlogPostParser {
    fn supports(&self, file: &File) -> bool {
        file.path.starts_with("_posts")
    }
    fn parse(&self, file: &File) -> Result<PageData, Box<dyn std::error::Error>> {
        let filename = file
            .path
            .file_name()
            .expect("failed getting filename")
            .to_str()
            .expect("failed getting str from osstr");

        let date_str: String = filename.chars().take(10).collect();
        let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").expect("failed to parse date");

        let content_and_front_matter = file
            .read_front_matter_and_contents("---")
            .expect("failed getting front matter");

        let post_frontmatter: PostFrontMatter =
            serde_yaml::from_str(&content_and_front_matter.front_matter)
                .expect("failed parsing frontmatter yaml");

        let post_data = PostData {
            title: post_frontmatter.title,
            date: date,
            content: content_and_front_matter.contents,
            tags: post_frontmatter
                .tags
                .split_whitespace()
                .map(|s| s.to_owned())
                .collect(),
        };

        Ok(PageData::Post(post_data))
    }
}

pub struct BlogPostGenerator {}

impl ContentGenerator<PageData, PageMetadata> for BlogPostGenerator {
    fn supports(&self, content: &PageData) -> bool {
        match content {
            PageData::Post(_) => true,
            _ => false,
        }
    }

    fn generate(
        &self,
        content: &PageData,
        site_variables: &SiteVariables<PageData>,
    ) -> Result<Vec<Page<PageMetadata>>, Box<dyn std::error::Error>> {
        use pulldown_cmark::{html, Options, Parser};
        use view::blog_post;

        match content {
            PageData::Post(post_data) => {
                let parser = Parser::new_ext(&post_data.content, Options::all());
                let mut html_output = String::new();
                html::push_html(&mut html_output, parser);

                let page = Page {
                    path: Path::new(&format!(
                        "posts/{}-{}.html",
                        post_data.date.format("%Y-%m-%d"),
                        post_data.title.replace(" ", "_")
                    ))
                    .to_owned(),
                    contents: blog_post(post_data, &html_output).to_string(),
                    metadata: PageMetadata {},
                };

                Ok(vec![page])
            }

            _ => unreachable!(),
        }
    }
}
