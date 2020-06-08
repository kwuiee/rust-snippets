extern crate photon_rs;

use photon_rs::native::{open_image, save_image};

fn main() {
    // Open the image (a PhotonImage is returned)
    let mut img = open_image("data/lion.jpg");

    // Increment the red channel by 40
    photon_rs::channels::alter_red_channel(&mut img, 40);

    // Write file to filesystem.
    save_image(img, "raw_image.png");
}
