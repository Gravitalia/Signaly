# Report instructions

## Introduction

Report instructions aims to facilitate requests to the Signaly service. This means that it defines how to contact Signaly to aggregate additional reports on separate services, given that Gravitalia is based on microservices and an event-driven architecture.

## Specification

[Apache Kafka](https://kafka.apache.org/) or [RabbitMQ](https://www.rabbitmq.com/) is required. The protocol you choose will depend on your Signaly configuration.

It uses [CloudEvents](https://cloudevents.io/) specifications. It uses [JSON](https://www.json.org/) to send event messages.

## Message attributes requirements

Message **MUST** fit with [CloudEvents core specifications, Version 1.0.2](https://github.com/cloudevents/spec/blob/v1.0.2/cloudevents/spec.md).

CloudEvents `source` field **MUST** be the emitter unique identifier (vanity).

The following attributes **MUST** be placed in the `data` field provided by CloudEvents:

**from**
* Type: `string`
* Description: unique identifier (vanity) of the user who sanctioned or reported the incident.
* Constraints:
  * **MUST** be a non-empty string.
  * **MUST** be identical for all services.

**to**
* Type: `string`
* Description: unique identifier (vanity) of the user affected by the sanction or report.
* Constraints:
  * **MUST** be a non-empty string.
  * **MUST** be identical for all services.

**type**
* Type: `string`
* Description: 
* Constraints:
  * OPTIONAL: if not set it uses `Report`.
  * **MUST** fit with: `Report` or `Sanction`.

**reason**
* Type: `string`
* Description: reason for the sanction or warning to be recorded. This reason can be used to better identify specific behaviour and target an automatic sanction.
* Constraints:
  * OPTIONAL.
  * **SHOULD** fit with predetermined reasons: `Copyright`, `Defamation`, `Hate`, `Harassment`, `Nudity`, `Spam` or `Violence`.

**sanction**
* Type: `string`
* Description: sanction taken against user.
* Constraints:
  * OPTIONAL.
  * **MUST** fit with predetermined sanctions: `Suspension` *(specific to the accounts)*, `Removal` *(specific to the content)*.

## Message example
The following example shows a message containing a report of `Nudity` from user `x` to user `y`:
```json
{
    "specversion" : "1.0",
    "type" : "com.gravitalia.report.add",
    "source" : "https://www.gravitalia.com/x",
    "id" : "aee5c274-a2d2-4e20-99d8-e63c8947813e",
    "time" : "2024-01-01T10:31:00Z",
    "datacontenttype" : "application/json",
    "data" : {
        "from": "x",
        "to": "y",
        "reason": "Nudity"
    }
}
```
