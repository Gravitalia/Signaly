# Quick start

## Introduction

This quick start shows how to simply deploy a Signaly instance. Check out our [deployement guide](https://github.com/Gravitalia/Signaly/blob/master/docs/deployement_guide.md) to learn how to bring Signaly into Microsoft Azure via [Terraform](https://www.terraform.io/).

## Deploy instance with [Apache Kafka](https://kafka.apache.org/)

> You'll need to install [Docker](https://www.docker.com/).

1. Create a `docker-compose.yaml` file.
   Write

```yaml
services:
  signaly:
    image: ghcr.io/gravitalia/signaly:latest
    platform: linux/amd64
    container_name: signaly
    depends_on:
      - cassandra
      - kafka
    environment:
      TOPIC: compliance
      CASSANDRA_HOSTS: cassandra:9042 # use a comma (,) to add multiple hosts.
      KAFKA_BROKERS: localhost:9092 # use a comma (,) to add multiple brokers.

  cassandra:
    image: cassandra:latest
    restart: always
    container_name: cassandra
    ports:
      - 9042:9042
    volumes:
      - ./data/cassandra:/var/lib/cassandra

  zookeeper:
    image: zookeeper
    container_name: zookeeper
    ports:
      - 2181:2181

  kafka:
    image: confluentinc/cp-kafka
    container_name: kafka
    depends_on:
      - zookeeper
    ports:
      - 9092:9092
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock
   command: sh -c "((sleep 5 && kafka-topics --bootstrap-server kafka:9092 --create --if-not-exists --replication-factor 1 --partitions 3 --topic compliance)&) && /etc/confluent/docker/run ">
    environment:
      KAFKA_ADVERTISED_LISTENERS: INSIDE://kafka:9092,OUTSIDE://localhost:9093
      KAFKA_LISTENER_SECURITY_PROTOCOL_MAP: INSIDE:PLAINTEXT,OUTSIDE:PLAINTEXT
      KAFKA_LISTENERS: INSIDE://0.0.0.0:9092,OUTSIDE://0.0.0.0:9093
      KAFKA_INTER_BROKER_LISTENER_NAME: INSIDE
      KAFKA_ZOOKEEPER_CONNECT: zookeeper:2181,
      KAFKA_OFFSETS_TOPIC_REPLICATION_FACTOR: 1
```

2. Execute `docker-compose up`.

## To go further...

See how to [deploy](https://github.com/Gravitalia/Signaly/blob/master/docs/deployement_guide.md) Signaly on Microsoft Azure.
You can also opt to add healthchecks on each container or add multiple brokers.
