# Build Tool

This tool is used to build or rebuild the packages with some customized patches or configurations.
It also provides some additional patches for CCNP container measurement.

## Prerequisite
[Intel TDX 1.0 technology preview](https://ubuntu.com/blog/intel-tdx-1-0-preview-on-ubuntu-23-10) 
is available on Ubuntu 23.10, and [this Github repository](https://github.com/canonical/tdx) 
provides guidance and straightforward instructions on how to get started.
Please follow the instructions to create a guest image and set up the TDX environment.

Some additional patches are provided in [kernel/patches](kernel/patches) directory for CCNP container measurement,
here is the information about the patches:

| Patch Number | Comments |
| ------------ | -------- |
| 0001 ~ 0007  | Extend TDX RTMR for IMA measurement |
| 0008 ~ 0009  | Add new IMA template [ima-cgpath](https://patchwork.kernel.org/project/linux-integrity/patch/20221224162830.21554-1-enrico.bravi@polito.it/) |
| 0010 ~ 0016  | Support [ConfigFS TSM](https://lwn.net/Articles/945578/) |

## Build
Install the build dependencies and build the packages

```Shell
sudo ./build.sh
```

*Note: this build script is based on Ubuntu 23.10 TDX early preview kernel, please make sure this kernel has been installed.*

## Install

All the packages are built in `output` directory, please follow [cvm-image-rewriter plugin](../cvm-image-rewriter/plugins/06-install-tdx-guest-kernel/README.md) or install them by `apt`/`dpkg`

```Shell
sudo apt install -y ./output/*.deb
```

or

```Shell
sudo dpkg -i ./output/*.deb
```
