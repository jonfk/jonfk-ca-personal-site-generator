use crate::{view, PageData, PageMetadata};

use anyhow::Error;
use chrono::NaiveDate;
use rodin::{
    fs_util::format_title_filename, ContentGenerator, ContentParser, File, Page, SiteVariables,
};
use serde::Deserialize;
use std::{path::Path, time::SystemTime};
use typed_html::{html, unsafe_text};

#[derive(Clone, Debug)]
pub struct HtmlPageData {
    pub title: String,
    pub content: String,
    pub last_modified: NaiveDate,
}

#[derive(Clone, Debug, Deserialize)]
pub struct HtmlPageFrontMatter {
    pub title: String,
}

pub struct HtmlPageParser {}

impl ContentParser<PageData> for HtmlPageParser {
    fn supports(&self, file: &File) -> bool {
        file.path.starts_with("pages")
    }
    fn parse(&self, file: &File) -> Result<PageData, Box<dyn std::error::Error>> {
        let content_and_front_matter = file
            .read_front_matter_and_contents("---")
            .expect("failed getting front matter");

        let frontmatter: HtmlPageFrontMatter =
            serde_yaml::from_str(&content_and_front_matter.front_matter)
                .expect("failed parsing frontmatter yaml");

        let data = HtmlPageData {
            title: frontmatter.title,
            content: content_and_front_matter.contents,
            last_modified: systemtime_to_naive_date(file.metadata.last_modified),
        };

        Ok(PageData::HtmlPage(data))
    }
}

pub struct HtmlPageGenerator {}

impl ContentGenerator<PageData, PageMetadata> for HtmlPageGenerator {
    fn supports(&self, content: &PageData) -> bool {
        match content {
            PageData::HtmlPage(_) => true,
            _ => false,
        }
    }

    fn generate(
        &self,
        content: &PageData,
        _site_variables: &SiteVariables<PageData>,
    ) -> Result<Vec<Page<PageMetadata>>, Box<dyn std::error::Error>> {
        match content {
            PageData::HtmlPage(data) => {
                let page = Page {
                    path: Path::new(&format!("{}.html", format_title_filename(&data.title)))
                        .to_owned(),
                    contents: view::base(
                        &data.title,
                        html!(<div>{ unsafe_text!(&data.content) }</div>),
                    )
                    .to_string(),
                    metadata: PageMetadata {},
                };

                Ok(vec![page])
            }

            _ => unreachable!(),
        }
    }
}

fn systemtime_to_naive_date(systemtime: SystemTime) -> NaiveDate {
    use chrono::NaiveDateTime;
    use std::time::UNIX_EPOCH;

    let duration = systemtime.duration_since(UNIX_EPOCH).unwrap();
    NaiveDateTime::from_timestamp(duration.as_secs() as i64, duration.subsec_nanos()).date()
}
