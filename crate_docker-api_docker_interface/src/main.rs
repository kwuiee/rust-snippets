extern crate docker_api;
extern crate tokio;

#[tokio::main]
async fn main() {
    let docker = docker_api::Docker::new("tcp://127.0.0.1:80").unwrap();

    match docker.images().list(&Default::default()).await {
        Ok(images) => {
            for image in images {
                println!("{:?}", image.repo_tags);
            }
        }
        Err(e) => eprintln!("Something bad happened! {}", e),
    }
}
