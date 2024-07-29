# Build Tool

This tool is used to build or rebuild the packages with some customized patches or configurations.
It also provides some additional patches for CIMA container measurement.

## Prerequisite
Intel TDX 1.0 technology preview is available, and [this Github repository](https://github.com/canonical/tdx/tree/noble-24.04) 
provides guidance and straightforward instructions on how to get started for Ubuntu 24.04.
Please follow the instructions to create a guest image and set up the TDX environment.

Ubuntu 24.04 is targeted as the default base for this build tool, and the default kernel version is
v6.8.0, some additional patches are provided in [kernel/patches](kernel/patches) directory for
CIMA container measurement, here is the information about the patches:

| Patch Number | Comments |
| ------------ | -------- |
| 0000         | Extend TDX RTMR |
| 0001 ~ 0007  | Extend TDX RTMR for IMA measurement |
| 0008 ~ 0009  | Add new IMA template [ima-cgpath](https://patchwork.kernel.org/project/linux-integrity/patch/20221224162830.21554-1-enrico.bravi@polito.it/) |

## Build
Install the build dependencies and build the packages. It is recommend to run the tool on the TDX
host prepared following [Configuration](../../README.md/#configuration).

```Shell
sudo ./build.sh
```

Same as TDX early preview kernel, the packages are based on the Ubuntu generic kernel by default,
the intel kernel can be selected by using the environment variable TDX_SETUP_INTEL_KERNEL.

```Shell
sudo TDX_SETUP_INTEL_KERNEL=1 ./build.sh
```

*Note: this build script is based on Ubuntu 24.04 TDX early preview kernel, please make sure this kernel has been installed.*

## Install

All the packages are built in `output` directory, please follow
[cvm-image-rewriter plugin](../cvm-image-rewriter/plugins/06-install-tdx-guest-kernel/README.md)
or install them by `apt`/`dpkg`

```Shell
sudo apt install -y ./output/*.deb
```

or

```Shell
sudo dpkg -i ./output/*.deb
```
