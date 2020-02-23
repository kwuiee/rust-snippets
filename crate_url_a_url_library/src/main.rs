extern crate url;

use std::error::Error;
use url::{Host, Position, Url};

fn main() -> Result<(), Box<dyn Error>> {
    let issue_list_url =
        Url::parse("https://github.com/rust-lang/rust/issues?labels=E-easy&state=open")?;

    assert!(issue_list_url.scheme() == "https");
    assert!(issue_list_url.username() == "");
    assert!(issue_list_url.password() == None);
    assert!(issue_list_url.host_str() == Some("github.com"));
    assert!(issue_list_url.host() == Some(Host::Domain("github.com")));
    assert!(issue_list_url.port() == None);
    assert!(issue_list_url.path() == "/rust-lang/rust/issues");
    assert!(
        issue_list_url
            .path_segments()
            .map(|c| c.collect::<Vec<_>>())
            == Some(vec!["rust-lang", "rust", "issues"])
    );
    assert!(issue_list_url.query() == Some("labels=E-easy&state=open"));
    assert!(
        &issue_list_url[Position::BeforePath..]
            == "/rust-lang/rust/issues?labels=E-easy&state=open"
    );
    assert!(issue_list_url.fragment() == None);
    assert!(!issue_list_url.cannot_be_a_base());
    let this_document = Url::parse("http://servo.github.io/rust-url/url/index.html")?;
    let css_url = this_document.join("../main.css")?;
    assert_eq!(css_url.as_str(), "http://servo.github.io/rust-url/main.css");
    Ok(())
}
