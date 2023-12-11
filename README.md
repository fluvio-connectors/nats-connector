# Fluvio Nats Connector
Fluvio Nats connector

## Source Connector
Reads record from Nats topic and writes to Fluvio topic.

### Configuration
| Option              | default  | type           | description                                                                                                    |
|:--------------------|:---------|:---------      |:---------------------------------------------------------------------------------------------------------------|
|                     |          |                |                                                                                                                |

#### Record Type Output

-

#### Payload Output Type

| Value  | Output                       |
|:-------|:-----------------------------|
| binary | Array of bytes               |
| json   | UTF-8 JSON Serialized String |

### Usage Example

This is an example of connector config file:

```yaml
```

Run connector locally using `cdk` tool (from root directory or any sub-directory):
```bash
fluvio install cdk

cdk deploy start --config config-example.yaml

cdk deploy list # to see the status
cdk deploy log my-nats-connector # to see connector's logs
```

Install nats Client such as
```bash
```

Insert records:
```bash
```

The produced record in Fluvio topic will be:
```json
```
### Transformations
Fluvio Nats Source Connector supports [Transformations](https://www.fluvio.io/docs/concepts/transformations-chain/).

Records can be modified before sending to Fluvio topic.



