use bollard::image::ListImagesOptions;
use bollard::Docker;

use std::default::Default;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let docker = Docker::connect_with_local_defaults()?;
    let images = &docker
        .list_images(Some(ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await?;

    for image in images {
        println!("-> {:?}", image);
    }

    Ok(())
}
