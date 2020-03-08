extern crate crabquery;

use crabquery::Document;

fn main() {
    let doc = Document::from(
        "<div class='container'>
       <a class='link button' id='linkmain'>
         <span>text hi there</span>
       </a>
     </div>",
    );

    let sel = doc.select("div.container > a.button.link[id=\"linkmain\"]");
    let el = sel.first().unwrap();

    assert_eq!(el.attr("id").unwrap(), "linkmain");

    let sel = doc.select("div > a > span");
    let el = sel.first().unwrap();

    assert_eq!(el.text().unwrap(), "text hi there");
}
