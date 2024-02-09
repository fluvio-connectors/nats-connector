# Fluvio Nats Connector
Fluvio Nats connector

## Source Connector
Reads record from Nats topic and writes to Fluvio topic.

### Configuration
| Option              | default  | type           | description                                                                                                    |
|:--------------------|:---------|:---------      |:---------------------------------------------------------------------------------------------------------------|
| host                | -        | String         | Nats broker host                                                                                               |
| subject             | -        | String         | Nats subject to relay events from into fluvio                                                                  |

### Usage Example

See [config-example.yaml](config-example.yaml) for an example reflecting the above.

Run connector locally using `cdk` tool (from root directory or any sub-directory):
```bash
fluvio install cdk

cdk deploy start --config config-example.yaml

cdk deploy list # to see the status
cdk deploy log my-nats-connector # to see connector's logs
```

Install nats Client such as [natscli](https://github.com/nats-io/natscli)
```bash
curl -sf https://binaries.nats.dev/nats-io/natscli/nats@latest | sh
```

Insert records:
```bash
nats pub my.subject.tofluvio "Hello World"
```

The produced record in Fluvio topic will be:
```json
{"nats_subject":"my.subject.tofluvio","nats_reply":null,"nats_data":[72,101,108,108,111,32,87,111,114,108,100]}
```

### Transformations
Fluvio Nats Source Connector supports [Transformations](https://www.fluvio.io/docs/concepts/transformations-chain/).

Records can be modified before sending to Fluvio topic.



