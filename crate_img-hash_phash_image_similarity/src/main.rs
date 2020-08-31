extern crate image;
extern crate img_hash;

use img_hash::HasherConfig;

fn main() {
    let image1 = image::open("1.png").unwrap();
    let image2 = image::open("2.png").unwrap();

    let hasher = HasherConfig::new().to_hasher();

    let hash1 = hasher.hash_image(&image1);
    let hash2 = hasher.hash_image(&image2);

    println!("Image1 hash: {}", hash1.to_base64());
    println!("Image2 hash: {}", hash2.to_base64());

    println!(
        "Hamming Distance, image 1 to image 2: {}",
        hash1.dist(&hash2)
    );
}
