use vial::prelude::*;

routes! {
    GET "/" => |_| "Hello, world!";
    GET "/echo" => echo;
    POST "/echo" => post;
}

fn echo(_: Request) -> &'static str {
    "<form method='POST'>
        <input type='text' name='echo'/>
        <input type='submit'/>
    </form>"
}

fn post(req: Request) -> String {
    format!(
        "<h1>{}</h1>",
        req.form("echo").unwrap_or("You didn't say anything!")
    )
}

fn main() {
    vial::run!("localhost:2333").unwrap();
}
