# Container Integrity Measurement Agent (CIMA)

![CI Check License](https://github.com/cc-api/container-integrity-measurement-agent/actions/workflows/pr-license-python.yaml/badge.svg)
![CI Check Spelling](https://github.com/cc-api/container-integrity-measurement-agent/actions/workflows/pr-doclint.yaml/badge.svg)
![CI Check Python](https://github.com/cc-api/container-integrity-measurement-agent/actions/workflows/pr-pylint.yaml/badge.svg)
![CI Check Shell](https://github.com/cc-api/container-integrity-measurement-agent/actions/workflows/pr-shell-check.yaml/badge.svg)
![CI Check Rust](https://github.com/cc-api/container-integrity-measurement-agent/actions/workflows/pr-check-rust.yaml/badge.svg)
![CI Check Golang](https://github.com/cc-api/container-integrity-measurement-agent/actions/workflows/pr-golang-check.yaml/badge.svg)
![CI Check Container](https://github.com/cc-api/container-integrity-measurement-agent/actions/workflows/pr-container-check.yaml/badge.svg)
[![OpenSSF Best Practices](https://www.bestpractices.dev/projects/8325/badge)](https://www.bestpractices.dev/projects/8325)

## Introduction

Confidential Computing technologies provide an isolated encryption runtime
environment to protect data-in-use based on hardware Trusted Execution Environment (TEE).
It requires a full chain integrity measurement on the launch-time or runtime environment
to guarantee "consistent behavior in an expected way" of confidential
computing environment for tenant's zero-trust use case.

CIMA aims to help users establish a chain of trust for cloud-native workloads by providing
container level evidence, including container measurements,
event logs, and confidential computing (CC) reports.

Find out more in [CIMA Design and Architecture](https://cc-api.github.io/container-integrity-measurement-agent/) and [Container Measurement Design](docs/container-measurement-design.md).


## How to Install CIMA

### Configuration

CIMA support to run on Intel® TDX guest. Thus, you will need TDX host and guest for CIMA deployment and usage. Please see below recommended configuration. 

|  CPU | Host OS  | Host packages  | Guest OS  | Guest packages  | Attestation packages | CIMA Tag |
|---|---|---|---|---|---|---|
|  Intel 4th Gen (only TDX SKUs) and 5th Gen Xeon Scalable Processors | Ubuntu 23.10| TDX early preview referring to [here](https://github.com/canonical/tdx?tab=readme-ov-file#4-setup-tdx-host) | Ubuntu 23.10 | Build a guest image for CIMA using [CVM image rewriter](https://github.com/cc-api/container-integrity-measurement-agent/tree/v0.4.0/deployment#prepare-tdx-guest-image) | Setup remote attestation on host referring to [here](https://github.com/canonical/tdx?tab=readme-ov-file#8-setup-remote-attestation-on-host-and-td-guest)| [v0.4.0](https://github.com/cc-api/container-integrity-measurement-agent/releases/tag/v0.4.0)
|  Intel 4th Gen (only TDX SKUs) and 5th Gen Xeon Scalable Processors | Ubuntu 24.04| TDX early preview referring to [here](https://github.com/canonical/tdx/tree/2.0?tab=readme-ov-file#setup-tdx-host) | Ubuntu 24.04 | Build a guest image for CIMA using [CVM image rewriter](https://github.com/cc-api/container-integrity-measurement-agent/tree/v0.5.0/deployment#prepare-tdx-guest-image) | Setup remote attestation on host referring to [here](https://github.com/cc-api/container-integrity-measurement-agent/blob/v0.5.0/container/pccs/README.md) and [here](https://github.com/cc-api/container-integrity-measurement-agent/blob/v0.5.0/container/qgs/README.md)| [v0.5.0](https://github.com/cc-api/container-integrity-measurement-agent/releases/tag/v0.5.0)



### CIMA Service Deployment in Confidential VM

CIMA will run as a DaemonSet in a Kubernetes cluster or as a container in a docker environment on a single confidential VM (CVM).
Refer to [CIMA deployment guide](deployment/README.md) and choose a deployment model.

### CIMA SDK Usage

If you want to integrate CIMA SDK in the workload to get measurement and event logs, refer to [py_sdk_example.py](/sdk/python3/example/py_sdk_example.py). It is an example of using CIMA Python SDK. There are also Golang SDK and Rust SDK. Please see more details in [CIMA SDK](https://cc-api.github.io/container-integrity-measurement-agent/sdk.html).


## Contributing

This project welcomes contributions and suggestions. Most contributions require
you to agree to a Contributor License Agreement (CLA) declaring that you have the
right to, and actually do, grant us the rights to use your contribution. For details,
contact the maintainers of the project.

When you submit a pull request, a CLA-bot will automatically determine whether you
need to provide a CLA and decorate the PR appropriately (e.g., label, comment).
Simply follow the instructions provided by the bot. You will only need to do this
once across all repos using our CLA.

See [CONTRIBUTING.md](CONTRIBUTING.md) for details on building, testing, and contributing
to these libraries.

## Provide Feedback

If you encounter any bugs or have suggestions, please file an issue in the Issues
section of the project.

_Note: This is pre-production software. As such, it may be substantially modified as updated versions are made available._

## Reference

[CIMA Design and Architecture](https://cc-api.github.io/container-integrity-measurement-agent/)

[Container Measurement Design](docs/container-measurement-design.md)

[Trusted Computing](https://en.wikipedia.org/wiki/Trusted_Computing)

[TCG PC Client Platform TPM Profile Specification](https://trustedcomputinggroup.org/resource/pc-client-platform-tpm-profile-ptp-specification/)

[TCG PC Client Platform Firmware Profile Specification](https://trustedcomputinggroup.org/resource/pc-client-specific-platform-firmware-profile-specification/)



## Contributors

<!-- spell-checker: disable -->

<!-- readme: contributors -start -->
<table>
<tr>
    <td align="center">
        <a href="https://github.com/Ruoyu-y">
            <img src="https://avatars.githubusercontent.com/u/70305231?v=4" width="100;" alt="Ruoyu-y"/>
            <br />
            <sub><b>Ruoyu Ying</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/hairongchen">
            <img src="https://avatars.githubusercontent.com/u/105473940?v=4" width="100;" alt="hairongchen"/>
            <br />
            <sub><b>Hairongchen</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/kenplusplus">
            <img src="https://avatars.githubusercontent.com/u/31843217?v=4" width="100;" alt="kenplusplus"/>
            <br />
            <sub><b>Lu Ken</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/ruomengh">
            <img src="https://avatars.githubusercontent.com/u/90233733?v=4" width="100;" alt="ruomengh"/>
            <br />
            <sub><b>Ruomeng Hao</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/hjh189">
            <img src="https://avatars.githubusercontent.com/u/88485603?v=4" width="100;" alt="hjh189"/>
            <br />
            <sub><b>Jiahao  Huang</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/HaokunX-intel">
            <img src="https://avatars.githubusercontent.com/u/108452001?v=4" width="100;" alt="HaokunX-intel"/>
            <br />
            <sub><b>Haokun Xing</b></sub>
        </a>
    </td></tr>
<tr>
    <td align="center">
        <a href="https://github.com/hwang37">
            <img src="https://avatars.githubusercontent.com/u/36193324?v=4" width="100;" alt="hwang37"/>
            <br />
            <sub><b>Wang, Hongbo</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/dongx1x">
            <img src="https://avatars.githubusercontent.com/u/34326010?v=4" width="100;" alt="dongx1x"/>
            <br />
            <sub><b>Xiaocheng Dong</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/LeiZhou-97">
            <img src="https://avatars.githubusercontent.com/u/102779531?v=4" width="100;" alt="LeiZhou-97"/>
            <br />
            <sub><b>LeiZhou</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/Yanbo0101">
            <img src="https://avatars.githubusercontent.com/u/110962880?v=4" width="100;" alt="Yanbo0101"/>
            <br />
            <sub><b>Yanbo Xu</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/jialeif">
            <img src="https://avatars.githubusercontent.com/u/88661406?v=4" width="100;" alt="jialeif"/>
            <br />
            <sub><b>Jialei Feng</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/jiere">
            <img src="https://avatars.githubusercontent.com/u/6448681?v=4" width="100;" alt="jiere"/>
            <br />
            <sub><b>Jie Ren</b></sub>
        </a>
    </td></tr>
<tr>
    <td align="center">
        <a href="https://github.com/wenhuizhang">
            <img src="https://avatars.githubusercontent.com/u/2313277?v=4" width="100;" alt="wenhuizhang"/>
            <br />
            <sub><b>Wenhui Zhang</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/rdower">
            <img src="https://avatars.githubusercontent.com/u/15023397?v=4" width="100;" alt="rdower"/>
            <br />
            <sub><b>Robert Dower</b></sub>
        </a>
    </td>
    <td align="center">
        <a href="https://github.com/zhlsunshine">
            <img src="https://avatars.githubusercontent.com/u/4101246?v=4" width="100;" alt="zhlsunshine"/>
            <br />
            <sub><b>Steve Zhang</b></sub>
        </a>
    </td></tr>
</table>
<!-- readme: contributors -end -->

<!-- spell-checker: enable -->
