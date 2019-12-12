#![recursion_limit = "1000"]
pub mod cli;
pub mod homepage;
pub mod html_pages;
pub mod posts;
pub mod view;

use crate::{
    homepage::HomepageGenerator,
    html_pages::{HtmlPageData, HtmlPageGenerator, HtmlPageParser},
    posts::{BlogPostGenerator, BlogPostParser, PostData},
};

use anyhow::Error;
use chrono::NaiveDate;
use rodin::Rodin;
use structopt::StructOpt;

pub fn run() -> Result<(), Error> {
    let opt = cli::Opt::from_args();

    let rodin: Rodin<PageData, PageMetadata, Error> = Rodin::new(opt.input, opt.output)
        .rm_target_dir_on_build(!opt.disable_rm_target)
        // Register parsers and generators
        .register_parser(Box::new(BlogPostParser {}))
        .register_content_generator(Box::new(BlogPostGenerator {}))
        .register_parser(Box::new(HtmlPageParser {}))
        .register_content_generator(Box::new(HtmlPageGenerator {}))
        .register_onetime_generator(Box::new(HomepageGenerator {}))
        // static assets
        .add_static_assets(&vec!["static"]);

    rodin.build()?;
    Ok(())
}

#[derive(Clone, Debug)]
pub enum PageData {
    Post(PostData),
    HtmlPage(HtmlPageData),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PageType {
    BlogPost,
    HtmlPage,
    Homepage,
}

#[derive(Clone, Debug)]
pub struct PageMetadata {
    title: String,
    date: NaiveDate,
    page_type: PageType,
}
