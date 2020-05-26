#!/usr/bin/env python
import pika

connection = pika.BlockingConnection(pika.ConnectionParameters(host="localhost"))
channel = connection.channel()

channel.exchange_declare(exchange="rust-exchange", exchange_type="fanout")

result = channel.queue_declare(queue="rust-queue", exclusive=False)
queue_name = result.method.queue

channel.queue_bind(exchange="rust-exchange", queue=queue_name)

print(" [*] Waiting for logs. To exit press CTRL+C")


def callback(ch, method, properties, body):
    print("Received [x] %r" % body)


channel.basic_consume(queue=queue_name, on_message_callback=callback, auto_ack=True)

channel.start_consuming()
