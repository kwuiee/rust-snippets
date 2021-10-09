use expectrl::{repl::spawn_python, Regex};

fn main() {
    let mut p = spawn_python().unwrap();

    p.execute("import platform").unwrap();
    p.send_line("platform.node()").unwrap();

    let found = p.expect(Regex(r"'.*'")).unwrap();

    println!("Platform {}", String::from_utf8_lossy(found.first()));
}
