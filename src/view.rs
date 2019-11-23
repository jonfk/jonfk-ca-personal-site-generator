use typed_html::{
    dom::{DOMTree, Node},
    elements::FlowContent,
    html, text,
    types::LinkType,
    unsafe_text, OutputType,
};

pub fn base(title: &str, content: Box<dyn FlowContent<String>>) -> DOMTree<String> {
    html! {
    <html>
      <head>
        <title>{ text!(title) }</title>
        <meta charset="UTF-8"/>
        <meta name="author" content="Jonathan Fok kan" />

        <meta name="viewport" content="width=device-width, initial-scale=1"/>

        //<!-- Homepage CSS -->
        <link rel="stylesheet" href="/css/screen.css" type="text/css" media="screen, projection" />
        <link rel="stylesheet" href="/css/syntax.css" type="text/css" />

        <link href="https://fonts.googleapis.com/css?family=Alegreya+SC:700,400" rel="stylesheet" type="text/css"/>
        //<!-- <link href='https://fonts.googleapis.com/css?family=Montserrat:400,700' rel='stylesheet' type='text/css'> -->
        <link href="https://fonts.googleapis.com/css?family=PT+Sans:400,400italic,700" rel="stylesheet" type="text/css"/>
        <link href="https://fonts.googleapis.com/css?family=Ubuntu+Mono" rel="stylesheet" type="text/css"/>

        <link rel="stylesheet" href="https://maxcdn.bootstrapcdn.com/font-awesome/4.5.0/css/font-awesome.min.css"/>

        <link rel="icon" type="image/png" href="/img/favicon-16x16.png" sizes="16x16" />
        <link rel="icon" type="image/png" href="/img/favicon-32x32.png" sizes="32x32" />
        <link rel="icon" type="image/png" href="/img/favicon-96x96.png" sizes="96x96" />
        <link rel="icon" type="image/png" href="/img/favicon-128x128.png" sizes="128x128" />
      </head>
      <body>

        <div class="site">
          <header role="banner">
            <a href="/">"Jonfk"</a>
          </header>
          <nav>
            <ul>
              <li>
                <a class="first extra" href="/">"home"</a>
              </li>
              <li>
                <a class="extra" href="/favorites.html">"favorites"</a>
              </li>
              <li>
                <a class="extra" href="/about.html">"about"</a>
              </li>
            </ul>
          </nav>

          <main>
            { content }
          </main>

          <footer>
            "© 2016 Jonathan Fok kan."
            <div class="rss">
              <a href="/feed.xml"><i class="fa fa-rss"></i></a>
            </div>
          </footer>

        </div>

      </body>
    </html>
    }
}

use crate::PostData;

pub fn blog_post(post: &PostData, content_html: &str) -> DOMTree<String> {
    let blog_html = html! {
        <article id="post">

            <header>
            <h1>{ text!(&post.title) }</h1>
            <span>{ text!(&post.date.format("%Y-%m-%d").to_string()) }</span>
            </header>

            <section>
        { unsafe_text!(content_html) }
        </section>

        //     <aside id="related">
        //     <h3>"Related Posts"</h3>
        //     <ul class="posts">
        // {% for post in site.related_posts limit:3 %}
        // <li><span>{{ post.date | date_to_string }}</span> &raquo; <a href="{{ post.url }}">{{ post.title }}</a></li>
        // {% endfor %}
        // </ul>
        //     </aside>

            </article>
    };

    base(&post.title, blog_html)
}
