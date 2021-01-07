use ramhorns::{Content, Template};

#[derive(Content)]
struct Post<'a> {
    title: &'a str,
    teaser: &'a str,
}

#[derive(Content)]
struct Blog<'a> {
    title: String,        // Strings are cool
    posts: Vec<Post<'a>>, // &'a [Post<'a>] would work too
}

fn main() {
    // Standard Mustache action here
    let source = "<h1>{{title}}</h1>\
              {{#posts}}<article><h2>{{title}}</h2><p>{{teaser}}</p></article>{{/posts}}\
              {{^posts}}<p>No posts yet :(</p>{{/posts}}";

    let tpl = Template::new(source).unwrap();

    let rendered = tpl.render(&Blog {
        title: "My Awesome Blog!".to_string(),
        posts: vec![
            Post {
                title: "How I tried Ramhorns and found love 💖",
                teaser: "This can happen to you too",
            },
            Post {
                title: "Rust is kinda awesome",
                teaser: "Yes, even the borrow checker! 🦀",
            },
        ],
    });

    assert_eq!(
        rendered,
        "<h1>My Awesome Blog!</h1>\
                      <article>\
                          <h2>How I tried Ramhorns and found love 💖</h2>\
                          <p>This can happen to you too</p>\
                      </article>\
                      <article>\
                          <h2>Rust is kinda awesome</h2>\
                          <p>Yes, even the borrow checker! 🦀</p>\
                      </article>"
    );
}
