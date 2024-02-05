#!/bin/bash
set -o errexit

CLUSTER_NAME=my-cluster
REG_NAME=kind-registry
DEVICE_PLUGIN=localhost:5001/ccnp-device-plugin:latest
QUOTE=localhost:5001/ccnp-quote-server:latest
MEASUREMENT=localhost:5001/ccnp-measurement-server:latest
EVENTLOG=localhost:5001/ccnp-eventlog-server:latest
TEST_NODE=localhost:5001/ccnp-test-node:latest

kind delete cluster --name $CLUSTER_NAME
rm /run/ccnp/uds/*
docker stop $REG_NAME
docker rm $REG_NAME
sleep 1m
docker rmi ${DEVICE_PLUGIN} ${QUOTE} ${MEASUREMENT} ${EVENTLOG} ${TEST_NODE}
