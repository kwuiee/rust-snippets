extern crate soup;

use soup::prelude::*;
use soup::Soup;

fn main() {
    const THREE_SISTERS: &'static str = r#"
<html><head><title>The Dormouse's story</title></head>
<body>
<p class="title"><b>The Dormouse's story</b></p>

<p class="story">Once upon a time there were three little sisters; and their names were
<a href="http://example.com/elsie" class="sister" id="link1">Elsie</a>,
<a href="http://example.com/lacie" class="sister" id="link2">Lacie</a> and
<a href="http://example.com/tillie" class="sister" id="link3">Tillie</a>;
and they lived at the bottom of a well.</p>

<p class="story">...</p>
"#;
    let soup = Soup::new(THREE_SISTERS);

    let title = soup.tag("title").find().expect("Couldn't find tag 'title'");
    assert_eq!(title.display(), "<title>The Dormouse's story</title>");
    assert_eq!(title.name(), "title");
    assert_eq!(title.text(), "The Dormouse's story".to_string());
    assert_eq!(
        title
            .parent()
            .expect("Couldn't find parent of 'title'")
            .name(),
        "head"
    );

    let p = soup.tag("p").find().expect("Couldn't find tag 'p'");
    assert_eq!(
        p.display(),
        r#"<p class="title"><b>The Dormouse's story</b></p>"#
    );
    assert_eq!(p.get("class"), Some("title".to_string())); // .find returns only the first 'a' tag
    let a = soup.tag("a").find().expect("Couldn't find tag 'a'");
    assert_eq!(
        a.display(),
        r#"<a class="sister" href="http://example.com/elsie" id="link1">Elsie</a>"#
    );
    // but .find_all will return _all_ of them:
    let a_s = soup.tag("a").find_all();
    assert_eq!(
        a_s.map(|a| a.display()).collect::<Vec<_>>().join("\n"),
        r#"<a class="sister" href="http://example.com/elsie" id="link1">Elsie</a>
<a class="sister" href="http://example.com/lacie" id="link2">Lacie</a>
<a class="sister" href="http://example.com/tillie" id="link3">Tillie</a>"#
    );
}
