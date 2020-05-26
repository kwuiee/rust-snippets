#!/usr/bin/env python
import time
import pika
import sys

connection = pika.BlockingConnection(pika.ConnectionParameters(host="localhost"))
channel = connection.channel()

channel.exchange_delete(exchange="rust-exchange")
channel.exchange_declare(exchange="rust-exchange", exchange_type="fanout")

message = "hello from python"

while True:
    channel.basic_publish(exchange="rust-exchange", routing_key="rust-binding", body=message)
    print(" [x] Sent %r" % message)
    time.sleep(3)
connection.close()
