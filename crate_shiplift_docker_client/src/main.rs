use futures::StreamExt;
use shiplift::Docker;

#[tokio::main]
async fn main() {
    let docker = Docker::new();

    println!("Listing images.");
    match docker.images().list(&Default::default()).await {
        Ok(images) => {
            for image in images {
                println!("{:?}", image.repo_tags);
            }
        }
        Err(e) => eprintln!("Something bad happened! {}", e),
    }

    println!("Listening for docker events");
    while let Some(event_result) = docker.events(&Default::default()).next().await {
        match event_result {
            Ok(event) => println!("event -> {:?}", event),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
