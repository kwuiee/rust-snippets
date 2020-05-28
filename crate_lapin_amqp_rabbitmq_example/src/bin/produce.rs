extern crate async_std;
extern crate lapin;

use async_std::task;
use lapin::{
    options::*, publisher_confirm::Confirmation, types::FieldTable, BasicProperties, Connection,
    ConnectionProperties, ExchangeKind, Result,
};
use std::time::Duration;

#[async_std::main]
async fn main() -> Result<()> {
    let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());

    let conn = Connection::connect(
        &addr,
        ConnectionProperties::default().with_default_executor(8),
    )
    .await?;

    println!("CONNECTED");

    let chan = conn.create_channel().await?;

    let queue = chan
        .queue_declare(
            "rust-queue",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    println!("Declared queue {:?}", queue);

    let exchange = chan
        .exchange_declare(
            "rust-exchange",
            ExchangeKind::Fanout,
            ExchangeDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    println!("Declared exchange {:?}", exchange);

    chan.queue_bind(
        "rust-queue",
        "rust-exchange",
        "rust-binding",
        QueueBindOptions::default(),
        FieldTable::default(),
    )
    .await?;

    let payload = b"Hello from rust!";

    loop {
        println!("sending {:?}", payload);
        let confirm = chan
            .basic_publish(
                "rust-exchange",
                "rust-binding",
                BasicPublishOptions::default(),
                payload.to_vec(),
                BasicProperties::default(),
            )
            .await?
            .await?;
        assert_eq!(confirm, Confirmation::NotRequested);
        task::sleep(Duration::from_secs(3)).await;
    }
}
