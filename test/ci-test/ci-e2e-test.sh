#!/bin/bash

# This is a CI test script. The script will run in cima-example pod.
# It will check SDK of python, golang and rust.

set -o errexit

PY_WORK_DIR='/run/cima/ci-test/py-test'

for i in {1..3}
do
    POD_NAME=$(kubectl get po -n cima | grep cima-example | grep Running | awk '{ print $1 }')
    if [[ -z "$POD_NAME" ]]
    then
        sleep 2
        echo "Retrying $i time ..."
    else
        break
    fi
done

if [ -z "$POD_NAME" ]; then
    echo "Error: CIMA example pod is not Running."
    exit 1
fi


# Run python tests
echo "--------> Run python test........."
kubectl exec -it "$POD_NAME" -n cima -- pytest -v ${PY_WORK_DIR}

# Run go tests
echo "--------> Run go test........."
kubectl exec -it "$POD_NAME" -n cima -- ./go-sdk-example

# Run rust tests
echo "--------> Run rust test........."
kubectl exec -it "$POD_NAME" -n cima -- ./rust-sdk-example
