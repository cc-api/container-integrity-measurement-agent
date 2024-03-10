# Performance Test

We have these KPIs for performance test.

| KPI​                                        | HIB/LIB​ | Unit​ | Comment​                                              |
| ------------------------------------------- | -------- | ----- | ----------------------------------------------------- |
| CCNP service get measurement throughput​    | HIB​     | ops​  | Service Throughput​                                   |
| CCNP service get measurement response time​ | LIB​     | ms​   | Service Response time​                                |
| CCNP service get eventlog throughput​       | HIB​     | ops​  | Service Throughput​                                   |
| CCNP service get eventlog response time​    | LIB​     | ms​   | Service Response time​                                |
| CCNP service get quote throughput​          | HIB​     | ops​  | Service Throughput​                                   |
| CCNP service get quote response time​       | LIB​     | ms​   | Service Response time​                                |
| CCNP initialization time​                   | LIB​     | s​    | CCNP device plugin, DaemonSet and service readiness.​ |

*Note: we use the CCNP SDK to access the CCNP service because it's convenient to prepare the request data (e.g. container ID, etc.)​

Below are the steps for you to build and run the performance test.

## Prerequisites

To run the test, you need a K8S cluster with CCNP enabled (CCNP Device Plugin and CCNP Service deployed and ready).

## Build

```bash
# Make sure you are on the repo's top dir
cd <the-dir-of-confidential-cloud-native-primitives>

# Run doker build
docker build --build-arg http_proxy=$http_proxy --build-arg https_proxy=$https_proxy --build-arg no_proxy=$no_proxy -t ccnp-perf:latest -f container/ccnp-perf/Dockerfile .

# View build result
docker image ls | grep ccnp-perf

# Save the docker image for later use
docker save ccnp-perf:latest > ccnp-perf_latest.tar
```

## Deploy

```bash
# Load the docker image for K8S using containerd.
# You need to run this on the node where you want to deploy the ccnp-perf test
ctr -n=k8s.io image import ccnp-perf_latest.tar

# Make sure you are on the repo's top dir
cd <the-dir-of-confidential-cloud-native-primitives>

# Deploy ccnp-perf test
kubectl apply -f deployment/kubernetes/manifests/ccnp-perf-deployment.yaml
```

## Test

```bash
# Get the pod name of ccnp-perf
kubectl get pod | grep ccnp-perf

# Run all perf test on the specified pod name got from above command
kubectl exec -ti <ccnp-perf-pod-name> -- python3 -m pytest --log-cli-level=INFO --verbose ccnp_perf.py
```

Sample test output looks like this:

```bash
root@ccnp-perf-0:~/ccnp/confidential-cloud-native-primitives# kubectl exec -ti ccnp-perf-7f8798bf85-8s6zg -- python3 -m pytest --log-cli-level=INFO --verbose
 ccnp_perf.py
==================================================================== test session starts ====================================================================
platform linux -- Python 3.12.2, pytest-8.1.1, pluggy-1.4.0 -- /usr/local/bin/python3
cachedir: .pytest_cache
rootdir: /run/ccnp
collected 7 items

ccnp_perf.py::test_svc_get_cc_measurement_throughput
----------------------------------------------------------------------- live log call -----------------------------------------------------------------------
INFO     ccnp_perf:ccnp_perf.py:191 Perf test average throughput is: 70.75 ops (operations per second)
PASSED                                                                                                                                                [ 14%]
ccnp_perf.py::test_svc_get_cc_measurement_response
----------------------------------------------------------------------- live log call -----------------------------------------------------------------------
INFO     ccnp_perf:ccnp_perf.py:213 Perf test average response time is: 25.89662575 ms (milliseconds)
PASSED                                                                                                                                                [ 28%]
ccnp_perf.py::test_svc_get_cc_eventlog_throughput
----------------------------------------------------------------------- live log call -----------------------------------------------------------------------
INFO     ccnp_perf:ccnp_perf.py:191 Perf test average throughput is: 57.8 ops (operations per second)
PASSED                                                                                                                                                [ 42%]
ccnp_perf.py::test_svc_get_cc_eventlog_response
----------------------------------------------------------------------- live log call -----------------------------------------------------------------------
INFO     ccnp_perf:ccnp_perf.py:213 Perf test average response time is: 76.130223 ms (milliseconds)
PASSED                                                                                                                                                [ 57%]
ccnp_perf.py::test_svc_get_cc_report_throughput
----------------------------------------------------------------------- live log call -----------------------------------------------------------------------
INFO     ccnp_perf:ccnp_perf.py:191 Perf test average throughput is: 54.9 ops (operations per second)
PASSED                                                                                                                                                [ 71%]
ccnp_perf.py::test_svc_get_cc_report_response
----------------------------------------------------------------------- live log call -----------------------------------------------------------------------
INFO     ccnp_perf:ccnp_perf.py:213 Perf test average response time is: 29.38618825 ms (milliseconds)
PASSED                                                                                                                                                [ 85%]
ccnp_perf.py::test_ccnp_init PASSED                                                                                                                   [100%]

=============================================================== 7 passed in 66.95s (0:01:06) ================================================================
root@ccnp-perf-0:~/ccnp/confidential-cloud-native-primitives#
```
