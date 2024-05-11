# Deploy QGS service on Docker

QGS (Quote Generation Service) implementation comes from
[DCAP](https://github.com/intel/SGXDataCenterAttestationPrimitives/tree/master/QuoteGeneration/quote_wrapper/qgs).
Currently, the package of QGS only support several distros. Using docker to deploy the QGS service can be an alternative for some unsupported distros.

## 1. QGS Service Usage Guide

### 1.1 Build QGS container image

Build QGS container image using [build.sh](../build.sh).
It will push the image to a registry specified in the command.
```bash
$ cd container
$ sudo ./build.sh -c qgs -r <your registry> -g <image tag> -q
```

### 1.2 Start QGS Service

```bash
docker run -d --privileged --name qgs --restart always --net host <your registry>
```
- Check if QGS service works

```console
$ docker ps
CONTAINER ID   IMAGE      COMMAND                 CREATED         STATUS         PORTS      NAMES
90a3777d813e   qgs        "/opt/intel/tdx-qgs/…"  9 minutes ago   Up 9 minutes              qgs
```
