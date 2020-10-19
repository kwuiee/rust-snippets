use robotstxt::DefaultMatcher;

fn main() {
    let mut matcher = DefaultMatcher::default();
    let robots_body = "user-agent: FooBot\n\
                       disallow: /\n";
    assert_eq!(
        false,
        matcher.one_agent_allowed_by_robots(robots_body, "FooBot", "https://foo.com/")
    );
}
