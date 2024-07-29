# Container Integrity Measurement Agent SDK for Python

The Container Integrity Measurement Agent (CIMA) project is the solution targeted on simplifying the use of Trusted Execution Environment (TEE) in cloud-native environment. Currently, there are 2 parts included in CIMA, the services and the SDK.

- Service is designed to hide the complexity of different TEE platforms and provides common interfaces and scalability for cloud-native environment.
- SDK is to simplify the use of the service interface for development, it covers communication to the service and parses the results from the services.

The service supports attestation, measurement fetching and event log collecting of various platforms including Intel Trusted Domain Extensions (TDX), Trusted Platform Modules (TPM) and AMD SEV-SNP. More platforms will be supported later.

Attestation is a common process within TEE platform and TPM to verify if the software binaries were properly instantiated on a trusted platform. Third parties can leverage the attestation process to identify the trustworthiness of the platform (by checking the measurements or event logs) as well as the software running on it, in order to decide whether they shall put their confidential information/workload onto the platform.

CIMA, as the overall framework for attestation, measurement and event log fetching, provides user with both customer-facing SDK and overall framework. By leveraging this SDK, user can easily retrieve different kinds of measurements or evidence such as event logs. Working along with different verification services (such as Amber) and configurable policies, user can validate the trustworthiness of the  platform and make further decision.

[Source code][source_code]
| [Package (PyPI)][cima_pypi]
| [API reference documentation][api_doc]

## Getting started

### Prerequisites
In order to work properly, user need to have the backend services ready on the TEE or TPM enabled platform first. Please refer to each deployment guide reside in the [service](../../service/) folder to install the backend services.

### Install the package
User can install the CIMA client library for Python with PyPI:

```
pip install cima
```

To install from source code, user can use the following command:

```
pip install -e .
```

## Key concepts and usage
There are three major functionalities provided in this SDK:

* [CC report fetching](#cc-report)
* [Measurement fetching](#measurement)
* [Event log fetching](#event-log)

### CC Report

Using this SDK, user could fetch the report from different platforms, the service detect the platform automatically and return the report.

#### Example usage of the SDK

The interface input of CC report is `nonce` and `user_data`, both of them are optional and will be measured in the report.
Here are the example usages of the SDK:

* Fetch report without any inputs
```python
from cima import CimaSdk

CimaSdk.inst().get_cc_report().dump()

```

* Fetch report with a `nonce`
```python
import base64
import secrets
from cima import CimaSdk

nonce = base64.b64encode(secrets.token_urlsafe().encode())
CimaSdk.inst().get_cc_report(nonce=nonce).dump()

```

* Fetch report with a `nonce` and `user_data`
```python
import base64
import secrets
from cima import CimaSdk

nonce = base64.b64encode(secrets.token_urlsafe().encode())
user_data = base64.b64encode(b'This data should be measured.')
CimaSdk.inst().get_cc_report(nonce=nonce, data=user_data).dump()

```

### Measurement

Using this SDK, user could fetch various measurements from different perspective and categories.
Basic support on measurement focus on the platform measurements, including TEE report, values within TDX RTMR registers or values reside in TPM PCR registers.
There's also advanced support to provide measurement for a certain workload or container. The feature is still developing in progress.

#### Example usage of the SDK

Here are the example usages for measurement SDK:

* Fetch TEE measurement base on platform
```python
from cima import CimaSdk

for i in [0, 1, 3]:
    m = CimaSdk.inst().get_cc_measurement([i, 12])
    print("IMR index: %d, hash: %s"%(i, m.hash.hex()))

```

### Event log

Using this SDK, user can fetch the event logs to assist the attestation/verification process. It also enables two different categories of event logs - for the platform or for a single workload/container.
From platform perspective, it can support different Trusted Execution Environment and TPM. This sdk can also do fetching on certain number of event logs.

#### Example usage of the SDK

Here are the example usages of the SDK:

* Fetch event log of platform and check the information inside
```python
from cima import CimaSdk

evt = CimaSdk.inst().get_cc_eventlog()
for e in evt:
    e.dump()

```

* Replay the event logs
```python
from cima import CimaSdk

evt = CimaSdk.inst().get_cc_eventlog()
replay = CimaSdk.inst().replay_cc_eventlog(evt)
for r in replay:
    print("Replay IMR[%d]: %s"%(r, replay[r][12].hex()))
    m = CimaSdk.inst().get_cc_measurement([r, 12])
    print("Read IMR[%d]: %s"%(r, m.hash.hex()))
    if m.hash != replay[r][12]:
        print("Replay IMR value does not match real IMR.")
    else:
        print("Verify event log replay value successfully.")
```

## End-to-end examples

TBA.

## Troubleshooting

Troubleshooting information for the CIMA SDK can be found here.

## Next steps
For more information about the Container Integrity Measurement Agent, please see our documentation page.

## Contributing
This project welcomes contributions and suggestions. Most contributions require you to agree to a Contributor License Agreement (CLA) declaring that you have the right to, and actually do, grant us the rights to use your contribution. For details, visit the Contributor License Agreement site.

When you submit a pull request, a CLA-bot will automatically determine whether you need to provide a CLA and decorate the PR appropriately (e.g., label, comment). Simply follow the instructions provided by the bot. You will only need to do this once across all repos using our CLA.

See [CONTRIBUTING.md](../../CONTRIBUTING.md) for details on building, testing, and contributing to these libraries.

## Provide Feedback
If you encounter any bugs or have suggestions, please file an issue in the Issues section of the project.

<!-- LINKS -->
[source_code]: https://github.com/cc-api/container-integrity-measurement-agent/tree/main/sdk/python3
[cima_pypi]: https://pypi.org/project/cima/
[api_doc]: https://github.com/cc-api/evidence-api?tab=readme-ov-file#3-apis
