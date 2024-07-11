# CCNP Performance Test

CCNP performance tests focus on the latency of calling CCNP SDK key APIs: `get_cc_eventlog`, `get_cc_report` and `get_cc_measurement`.
It will simulate requests from multiple pods in parallel and calculate average time of all the requests.

Below are the steps for you to build and run the performance test.

## Prerequisites

Please make sure you have CCNP deployed in a K8S cluster, and ccnp-example image has been built.
Please refer to [here](../../deployment/README.md) for image building and CCNP deployment.

## Run Tests

### Deploy pods for performance testing

```bash
# Deploy ccnp-example pods
$ sudo ./deploy-perf.sh -r <ccnp-example image registry> -g <ccnp-example image tag> -n <number of pods>
```

### Run Tests

The script will run tests in parallel. The log will be saved in files with prefix `perf_output` under current directory.

```bash
# Test for get event log
$ sudo ./perf-para.sh -n <number of pods> -e

# Test for get measurement
$ sudo ./perf-para.sh -n <number of pods> -m

# Test for get quote
$ sudo ./perf-para.sh -n <number of pods> -r
```

Run below script to calculate average time of a request.

```bash
$ sudo ./average.sh -f perf_output_quote
```

### Clear

Run below command to delete the pods for performance testing.

```bash
$ sudo ./deploy-per.sh -n <number of pods> -d
```