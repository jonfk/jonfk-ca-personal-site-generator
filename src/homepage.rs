use crate::{view, PageData, PageMetadata, PageType};

use anyhow::Error;
use chrono::{NaiveDate, Utc};
use rodin::{OneTimeGenerator, Page, SiteVariables};
use std::path::Path;
use typed_html::{html, text};

pub struct HomepageGenerator {}

impl OneTimeGenerator<PageData, PageMetadata> for HomepageGenerator {
    type Err = Error;

    fn generate(
        &self,
        site_variables: &SiteVariables<PageData, PageMetadata>,
    ) -> Result<Vec<Page<PageMetadata>>, Self::Err> {
        let posts: Vec<_> = site_variables
            .pages()
            .filter_map(|page| {
                if page.metadata.page_type == PageType::BlogPost {
                    Some(Post {
                        title: page.metadata.title.clone(),
                        relative_url: page.relative_url().unwrap(),
                        date: page.metadata.date.clone(),
                    })
                } else {
                    None
                }
            })
            .collect();
        let now = Utc::now().naive_utc();
        let page = Page {
            path: Path::new("index.html").to_owned(),
            contents: view(posts),
            metadata: PageMetadata {
                title: "jonfk.ca".to_owned(),
                date: now.date(),
                page_type: PageType::Homepage,
            },
        };

        Ok(vec![page])
    }
}

struct Post {
    title: String,
    relative_url: String,
    date: NaiveDate,
}

fn view(posts: Vec<Post>) -> String {
    let body_html = html! {
        <div class="home">
            <section>
            <h1>"Posts"</h1>
            <ul class="posts">
        {
            posts.iter().map(|post| {
                html!(<li><span>{ text!(post.date.format("%d %b %Y").to_string()) }</span>" » "<a href={ &post.relative_url }>{ text!(&post.title) }</a></li>)
            })
        }
            </ul>
            </section>

            <section>
            <h1>"Projects"</h1>
            <ul class="posts">
            <li><a href="/wilks.html">"Wilks Calculator"</a>"A web app to calculate your Wilks score"</li>
            </ul>
            </section>
            </div>
    };
    view::base("homepage", body_html).to_string()
}
