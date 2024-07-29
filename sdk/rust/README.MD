# Container Integrity Measurement Agent SDK for Rust

The Container Integrity Measurement Agent (CIMA) project is the solution targeted on simplifying the use of Trusted Execution Environment (TEE) in cloud-native environment. Currently, there are 2 parts included in CIMA, the services and the SDK.

- Service is designed to hide the complexity of different TEE platforms and provides common interfaces and scalability for cloud-native environment.
- SDK is to simplify the use of the service interface for development, it covers communication to the service and parses the results from the services.

The service supports attestation, measurement fetching and event log collecting of various platforms including Intel Trusted Domain Extensions (TDX), Trusted Platform Modules (TPM) and AMD SEV-SNP. More platforms will be supported later.

Attestation is a common process within TEE platform and TPM to verify if the software binaries were properly instantiated on a trusted platform. Third parties can leverage the attestation process to identify the trustworthiness of the platform (by checking the measurements or event logs) as well as the software running on it, in order to decide whether they shall put their confidential information/workload onto the platform.

CIMA, as the overall framework for attestation, measurement and event log fetching, provides user with both customer-facing SDK and overall framework. By leveraging this SDK, user can easily retrieve different kinds of measurements or evidence such as event logs. Working along with different verification services (such as Amber) and configurable policies, user can validate the trustworthiness of the  platform and make further decision.

[Source code][source_code]
| [API reference documentation][api_doc]

## Getting started

### Prerequisites
In order to work properly, user need to have the backend services ready on the TEE or TPM enabled platform first. Please refer to each deployment guide reside in the [service](../../service/) folder to install the backend services.

### Use the package
User can include CIMA client library for Rust in the `Cargo.toml` of their rust project:

```
cima = { git="https://github.com/cc-api/container-integrity-measurement-agent"}
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

* Fetch report with a `nonce` and `user_data`
```rust
use cctrusted_base::api::*;
use cctrusted_base::api_data::*;
use cctrusted_base::cc_type::TeeType;
use cctrusted_base::tdx::quote::TdxQuote;
use cima::sdk::API;
use log::*;
use rand::Rng;

fn get_cc_report() {
     /***
     * Note: in real user case, the nonce should come from attestation server
     * side to prevent replay attack and the data should be generate by API caller
     * according to user define spec
     */
    let nonce = base64::encode(rand::thread_rng().gen::<[u8; 32]>());
    let data = base64::encode(rand::thread_rng().gen::<[u8; 32]>());

    // retrieve cc report with API "get_cc_report"
    info!("call cc trusted API [get_cc_report] to retrieve cc report with nonce and data!");
    let report = match API::get_cc_report(Some(nonce), Some(data), ExtraArgs {}) {
        Ok(q) => q,
        Err(e) => {
            info!("error getting cc report: {:?}", e);
            std::process::exit(-1);
        }
    };

    info!("length of the cc report: {}", report.cc_report.len());

    // dump the cc report with API "dump_cc_report"
    //info!("call cc trusted API [dump_cc_report] to dump cc report!");
    //API::dump_cc_report(&report.cc_report);

    // parse the cc report with API "parse_cc_report"
    if report.cc_type == TeeType::TDX {
        let tdx_quote: TdxQuote = match CcReport::parse_cc_report(report.cc_report) {
            Ok(q) => q,
            Err(e) => {
                info!("error parse tdx quote: {:?}", e);
                std::process::exit(-1);
            }
        };
        info!(
            "version = {}, report_data = {}",
            tdx_quote.header.version,
            base64::encode(tdx_quote.body.report_data)
        );

        // show data of the struct TdxQuoteHeader
        info!("call struct show function to show data of the struct TdxQuoteHeader!");
        tdx_quote.header.show();
    }

    // retrieve cc report with API "get_cc_report"
    info!("call cc trusted API [get_cc_report] to retrieve cc report with no nonce and data!");
    let report1 = match API::get_cc_report(None, None, ExtraArgs {}) {
        Ok(q) => q,
        Err(e) => {
            info!("error getting cc report: {:?}", e);
            std::process::exit(-1);
        }
    };

    info!("length of the cc report: {}", report1.cc_report.len());

    // parse the cc report with API "parse_cc_report"
    if report1.cc_type == TeeType::TDX {
        let tdx_quote: TdxQuote = match CcReport::parse_cc_report(report1.cc_report) {
            Ok(q) => q,
            Err(e) => {
                info!("error parse tdx quote: {:?}", e);
                std::process::exit(-1);
            }
        };
        info!(
            "version = {}, report_data = {}",
            tdx_quote.header.version,
            base64::encode(tdx_quote.body.report_data)
        );

        // show data of the struct TdxQuoteHeader
        info!("call struct show function to show data of the struct TdxQuoteHeader!");
        tdx_quote.header.show();
    }
}

```


### Measurement

Using this SDK, user could fetch various measurements from different perspective and categories.
Basic support on measurement focus on the platform measurements, including TEE report, values within TDX RTMR registers or values reside in TPM PCR registers.
There's also advanced support to provide measurement for a certain workload or container. The feature is still developing in progress.

#### Example usage of the SDK

Here are the example usages for measurement SDK:

* Fetch TEE measurement of the current pod(Fetch measurements of all IMR index)
```rust
use cctrusted_base::api::*;
use cctrusted_base::api_data::*;
use cctrusted_base::tcg::TcgAlgorithmRegistry;
use cima::sdk::API;
use log::*;

fn get_cc_measurement() {
    // get default algorithm with API "get_default_algorithm"
    info!("call cc trusted API [get_default_algorithm] to get supported algorithm!");
    let defalt_algo = match API::get_default_algorithm() {
        Ok(algorithm) => {
            info!("supported algorithm: {}", algorithm.algo_id_str);
            algorithm
        }
        Err(e) => {
            error!("error get algorithm: {:?}", e);
            std::process::exit(-1);
        }
    };

    // get number of measurement registers
    info!("call cc trusted API [get_measurement_count] to get number of measurement registers!");
    let _count = match API::get_measurement_count() {
        Ok(count) => {
            info!("measurement registers count: {}", count);
            count
        }
        Err(e) => {
            error!("error get measurement count: {:?}", e);
            std::process::exit(-1);
        }
    };

    // retrive and show measurement registers
    info!("call cc trusted API [get_cc_measurement] to get measurement register content!");
    for index in [0, 1, 3] {
        let tcg_digest = match API::get_cc_measurement(index, defalt_algo.algo_id) {
            Ok(tcg_digest) => tcg_digest,
            Err(e) => {
                error!("error get measurement: {:?}", e);
                std::process::exit(-1);
            }
        };
        info!(
            "show index = {}, algo = {:?}, hash = {:02X?}",
            index,
            tcg_digest.get_algorithm_id_str(),
            tcg_digest.get_hash()
        );
    }
}

```

### Event log

Using this SDK, user can fetch the event logs to assist the attestation/verification process. It also enables two different categories of event logs - for the platform or for a single workload/container.
From platform perspective, it can support different Trusted Execution Environment and TPM. This sdk can also do fetching on certain number of event logs.

#### Example usage of the SDK

Here are the example usages of the SDK:

* Fetch event log of platform and check the information inside, then replay event logs
```rust
use cctrusted_base::api::*;
use cctrusted_base::api_data::*;
use cima::sdk::API;
use log::*;

fn get_cc_eventlog() {
    // retrieve cc eventlog with API "get_cc_eventlog"
    info!("call cc trusted API [get_cc_eventlog] to get container related eventlog without count!");
    let eventlogs1 = match API::get_cc_eventlog(Some(0), None) {
        Ok(q) => q,
        Err(e) => {
            error!("error getting eventlog: {:?}", e);
            std::process::exit(-1);
        }
    };

    info!("container event log count: {}", eventlogs1.len());

    // retrieve cc eventlog with API "get_cc_eventlog"
    info!("call cc trusted API [get_cc_eventlog] to get container related eventlog with count number!");
    let eventlogs = match API::get_cc_eventlog(Some(0), Some(101)) {
        Ok(q) => q,
        Err(e) => {
            error!("error getting eventlog: {:?}", e);
            std::process::exit(-1);
        }
    };

    info!("event log count: {}", eventlogs.len());
    for eventlog in &eventlogs {
        eventlog.show();
    }

    // replay cc eventlog with API "replay_cc_eventlog"
    info!("call cc trusted API [replay_cc_eventlog] to replay container related eventlog!");
    let replay_results = match API::replay_cc_eventlog(eventlogs) {
        Ok(q) => q,
        Err(e) => {
            error!("error replay eventlog: {:?}", e);
            std::process::exit(-1);
        }
    };

    // show replay results
    for replay_result in replay_results {
        replay_result.show();
    }
}

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
[source_code]: https://github.com/cc-api/container-integrity-measurement-agent/tree/main/sdk/rust
[api_doc]: https://github.com/cc-api/cc-trusted-api?tab=readme-ov-file#3-apis
