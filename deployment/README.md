# CCNP Deployment Guide

CCNP is designed for collecting confidential computing primitives in cloud native environments. It can run as DaemonSet in a Kubernetes cluster or containers in a Docker environment on confidential virtual machines, such as Intel TDX guest(TD). 

## Build CCNP Kernel

Run [build.sh](../tools/build/build.sh) to build kernel packages for CCNP. It's recommended to run the tool on TDX host mentioned in [Configuration](../README.md#configuration).

It will generate a `output` folder including kernel packages. The folder will be used in the next step.
```
$ cd tools/build
$ sudo ./build.sh
```

**NOTE:**
  - CCNP kernel patches are at [kernel](../tools/build/kernel/)
  - The tool should be run on a Ubuntu 23.10 TDX host with TDX early preview packages installed. Please refer to [here](https://github.com/canonical/tdx)


## Prepare TDX guest image

Run [cvm image rewriter](../tools/cvm-image-rewriter/README.md) to prepare a TDX guest image for CCNP deployment. The default user name is `tdx`. The password is `123456`.

It's recommended to run the tool on TDX host mentioned in [Configuration](../README.md#configuration).

A quick start is as below.

```
# Download Ubuntu 23.10 cloud image (Skip this step if you already has an initial guest image.)
$ wget https://cloud-images.ubuntu.com/mantic/current/mantic-server-cloudimg-amd64.img

# Set file path of the generated output folder above. Plugin 06 will install the kernel in the guest image.
$ export CVM_TDX_GUEST_REPO=<path to above output folder>

# Set image size
$ export GUEST_SIZE=<image size>G

# Run CVM image rewriter to configure a TDX guest image for CCNP
$ cd tools/cvm-image-rewriter
$ ./run.sh -i <mantic-server-cloudimg-amd64.img or your initial guest image>  -t <timeout in minutes, suggest to set to 15>
```

**NOTE:**
 - By default all the plugins will be executed. Generate a `NOT_RUN` file under the specific plugin folder if you want to skip it.
 - It's required to run [plugin](../tools/cvm-image-rewriter/plugins/) 06, 07, 08, 09 for CCNP.


## Create a TD

Start a TD using [qemu-test.sh](../tools/cvm-image-rewriter/qemu-test.sh) or [start-virt.sh](../tools/cvm-image-rewriter/start-virt.sh).

 - Use `qemu-test.sh`, please use `-q <vsock>` to make sure get quote works for the TD.
    ```
    $ sudo ./qemu-test.sh -i output.qcow2 -q vsock
    ```

- Use `start-virt.sh`. The Libvirt XML template is [tdx-libvirt-ubuntu-host.xml.template](../tools/cvm-image-rewriter/tdx-libvirt-ubuntu-host.xml.template). It uses `vsock` for getting quote.
    ```
    $ sudo ./start-virt.sh -i <guest image>
    ```

Check the kernel version. It should be CCNP kernel as below.

```
$ uname -ar | grep -i ccnp
Linux tdx-guest 6.5.0-1003-intel-opt #3.ccnp.1
```

If above output is empty, refer to [Build CCNP Kernel](#build-ccnp-kernel) to generate CCNP kernel packages. Then install the packages in the TD and make it as default kernel.

## Build CCNP images

Run script [build.sh](../container/build.sh) to generate CCNP images. It will generate 3 images and push them to user specific registry. Learn more details in the [README.md](../container/README.md).

**NOTE:**
  - The scripts need to run on a server with docker installed.
  - Run `docker login` before running the tool to make sure it can pull images.
  - Set proxy server in your environment if needed. See more details in [Configure Docker to use a proxy server](https://docs.docker.com/network/proxy/).

```
$ cd container
$ sudo ./build.sh -r <remote registry> -g <docker image tag>

e.g.

# Build images with tag 0.3 and push them to remote registry test-registry.intel.com
$ sudo ./build.sh -r test-registry.intel.com/test -g 0.3

# Build images only with tag 0.3
$ sudo ./build.sh -a build -g 0.3
```

After the script is executed successfully, it's supposed to see below docker images for CCNP.

```
$ sudo docker images
ccnp-example                    <your image tag>
ccnp-server                     <your image tag>
ccnp-webhook                    <your image tag>
```

## Setup QGS and PCCS on the Host

Intel Quote Generation Service(QGS) and Provisioning Certification Caching Service(PCCS) should be installed and configured on the host for getting TD Quote. Please refer to Section 4.3.2, 4.3.3 and 4.3.4 of [guide](https://www.intel.com/content/www/us/en/content-details/789198/whitepaper-linux-stacks-for-intel-trust-domain-extensions-1-5.html)
for QGS and PCCS installation.


## Deploy CCNP in Kubernetes

Below diagram illustrates CCNP deployment process in a Kubernetes cluster. If you want to install CCNP services as DamonSets in the Kubernetes cluster, please refer to [CCNP deployment in Kubernetes](./kubernetes/README.md).

![Deployment diagram](../docs/ccnp-deployment-k8s.png)


## Deploy CCNP in Docker

Below diagram illustrates CCNP deployment process using docker compose. If you want to setup CCNP services as docker containers, please refer to [CCNP deployment in Docker](./docker-compose/README.md).

![Deployment diagram](../docs/ccnp-deployment-docker.png)
