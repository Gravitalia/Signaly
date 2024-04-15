<img src="https://avatars.githubusercontent.com/u/81774317?s=200&v=4" width="40" />

# Signaly

> Signaly aggregates reports and sanctions over time so that it can react conscientiously to limit damage to other users.

Signaly does not add latency to other services. Everything is processed asynchronously. It issues automatic sanctions asynchronously; services are free to process them or not.

## Feature highlights
- Support multiple message broker ([Apache Kafka](https://kafka.apache.org/) & [RabbitMQ](https://www.rabbitmq.com/))
- Support telemetry ([Prometheus](https://prometheus.io/), [Jaeger](https://www.jaegertracing.io/) and Grafana [Loki](https://grafana.com/oss/loki/))

## Getting started

See our [quick starting guide](https://github.com/Gravitalia/Signaly/blob/master/docs/quick_start.md) to find out how to properly set up Signaly. Also look at the [attributes required in messages](https://github.com/Gravitalia/Signaly/blob/master/docs/report_instructions.md).

## License

This project is Licensed under [Mozilla Public License, Version 2.0](https://github.com/Gravitalia/Signaly/blob/master/LICENSE).
