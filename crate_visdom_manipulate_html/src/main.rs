extern crate visdom;

use std::error::Error;

use visdom::Vis;

fn main() -> Result<(), Box<dyn Error>> {
    let html = r##"
  <div class="second-child"></div>
  <div id="container">
    <div class="first-child"></div>
  </div>
"##;
    let root = Vis::load(html)?;
    let mut container = root.find("#container");
    let mut second_child = root.find(".second-child");
    // append the `second-child` element to the `container`
    container.append(&mut second_child);
    // then the code become to below
    /*
    <div id="container">
      <div class="first-child"></div>
      <div class="second-child"></div>
    </div>
    */
    // create new element by `Vis::load`
    let mut third_child = Vis::load(r##"<div class="third-child"></div>"##)?;
    container.append(&mut third_child);
    // then the code become to below
    /*
    <div id="container">
      <div class="first-child"></div>
      <div class="second-child"></div>
      <div class="third-child"></div>
    </div>
    */
    Ok(())
}
