#!/bin/bash

# This is a CI test script. The script will run in ccnp-example pod.
# It will check SDK of python, golang and rust.

set -o errexit

PY_WORK_DIR='test/py-test'
GO_WORK_DIR='test/go-test/test-cases'

for i in {1..3}
do
    POD_NAME=$(kubectl get po | grep ccnp-example | grep Running | awk '{ print $1 }')
    if [[ -z "$POD_NAME" ]]
    then
        sleep 2
        echo "Retrying $i time ..."
    else
        break
    fi
done

if [ -z "$POD_NAME" ]; then
    echo "Error: CCNP example pod is not Running."
    exit 1
fi


# Run python test cases
if $PY_FLAG
then
    kubectl exec -it "$POD_NAME" -- pytest -v ${PY_WORK_DIR}
fi

# Run golang test cases
if $GO_FLAG
then
    kubectl exec -it "$POD_NAME" -- bash  -c "pushd  ${GO_WORK_DIR};go test -v;popd "
fi

# Run rust test cases
if $GO_FLAG
then
    kubectl exec -it "$POD_NAME" -- bash  -c "pushd  ${GO_WORK_DIR};go test -v;popd "
fi
