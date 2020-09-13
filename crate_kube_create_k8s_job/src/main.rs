use futures::{StreamExt, TryStreamExt};
use k8s_openapi::api::batch::v1::Job;
use kube::api::{Api, ListParams, Meta, PostParams, WatchEvent};
use kube::Client;

#[tokio::main]
async fn main() -> Result<(), kube::Error> {
    // Read the environment to find config for kube client.
    // Note that this tries an in-cluster configuration first,
    // then falls back on a kubeconfig file.
    let client = Client::try_default().await?;

    // Get a strongly typed handle to the Kubernetes API for interacting
    // with pods in the "default" namespace.
    let jobs: Api<Job> = Api::namespaced(client, "default");

    // Create a job from JSON
    let name = uuid::Uuid::new_v4().to_simple().to_string();
    let job = serde_json::from_value(serde_json::json!({
        "apiVersion": "batch/v1",
        "kind": "Job",
        "metadata": {
            "name": &name,
        },
        "spec": {
            "backoffLimit": 4,
            "template": {
                "spec": {
                    "containers": [
                        {
                            "name": "my-container",
                            "image": "docker/whalesay:latest",
                            "command": ["cowsay"],
                            "args": ["hello world"]
                        },
                    ],
                    "restartPolicy": "Never",
                }
            }
        }
    }))?;

    // Create the job
    let job = jobs.create(&PostParams::default(), &job).await?;

    // Start a watch call for jobs matching our name
    let lp = ListParams::default()
        .fields(&format!("metadata.name={}", &name))
        .timeout(10);
    let mut stream = jobs.watch(&lp, "0").await?.boxed();

    // Observe the jobs phase for 10 seconds
    while let Some(status) = stream.try_next().await? {
        match status {
            WatchEvent::Added(o) => println!("Added {}", Meta::name(&o)),
            WatchEvent::Modified(o) => {
                let s = o.status.as_ref().expect("status exists on job");
                let failed = s.failed.unwrap_or(0);
                let succeeded = s.succeeded.unwrap_or(0);
                println!(
                    "Modified: {} with failed {} succeeded {}",
                    Meta::name(&o),
                    failed,
                    succeeded
                );
            }
            WatchEvent::Deleted(o) => println!("Deleted {}", Meta::name(&o)),
            WatchEvent::Error(e) => println!("Error {}", e),
            _ => {}
        }
    }

    Ok(())
}
