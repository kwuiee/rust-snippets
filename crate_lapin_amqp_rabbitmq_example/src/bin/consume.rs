extern crate async_std;
extern crate lapin;

use futures_util::stream::StreamExt;
use lapin::{
    options::*, types::FieldTable, Connection, ConnectionProperties, ExchangeKind, Result,
};

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

    let mut consumer = chan
        .basic_consume(
            "rust-queue",
            "rust-binding",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await?;

    println!("will consume");
    while let Some(delivery) = consumer.next().await {
        let delivery = delivery.expect("error in consumer");
        chan.basic_ack(delivery.delivery_tag, BasicAckOptions::default())
            .await
            .expect("ack");
        println!("Received {:?}", delivery);
    }
    Ok(())
}
