# Produced messages

## Introduction

Messages are the responses sent via Apache Kafka or RabbitMQ to inform decentralised services of the sanctions automatically taken. Messages can be ignored by the services, but they **SHOULD** be treated.

## Specification

[Apache Kafka](https://kafka.apache.org/) or [RabbitMQ](https://www.rabbitmq.com/) is required. The protocol you choose will depend on your Signaly configuration.

It uses [CloudEvents](https://cloudevents.io/) specifications. It uses [JSON](https://www.json.org/) to send event messages.

## Message attributes

Message **MUST** fit with [CloudEvents core specifications, Version 1.0.2](https://github.com/cloudevents/spec/blob/v1.0.2/cloudevents/spec.md).

Following attributes will be placed in the `data` field provided by CloudEvents:

**to**
* Type: `string`
* Description: unique identifier of user **OR** content sanctioned.
* Constraints:
  * **MUST** be a non-empty string.
  * **MUST** be identical for all services.

**reason**
* Type: `string`
* Description: reason for the sanction.
* Constraints:
  * **MUST** fit with predetermined reasons: `Copyright`, `Defamation`, `Hate`, `Harassment`, `Nudity`, `Spam`, `Violence` or `Other` *(when reports are too varied)*.

**sanction**
* Type: `string`
* Description: sanction taken against user.
* Constraints:
  * **MUST** be a non-empty string.
  * **MUST** fit with predetermined sanctions: `Suspension` *(specific to the accounts)*, `Removal` *(specific to the content)*.

## Message example
The following example shows a message containing a sanction for `Nudity` to content `111111111`:
```json
{
    "specversion" : "1.0",
    "type" : "com.gravitalia.sanction",
    "source" : "/sanction/toomanyreports",
    "id" : "12d9f651-8123-4739-96d0-e39ed0b69d62",
    "time" : "2024-01-01T10:31:00Z",
    "datacontenttype" : "application/json",
    "data" : {
        "to": "111111111",
        "reason": "Other",
        "sanction": "Removal"
    }
}
```
