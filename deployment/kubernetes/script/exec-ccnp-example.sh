#!/bin/bash
# Script to execute CCNP example pod

set -e

MEASUREMENT=false
EVENTLOG=false
CC_REPORT=false
VERIFY=false

usage() { echo "Usage: $0 [-m get measurement] [-e get event logs] [-r get cc report] [-v verify event logs]"; exit 1; }
while getopts ":mervh" option; do
        case "${option}" in
            m) MEASUREMENT=true;;
            e) EVENTLOG=true;;
            r) CC_REPORT=true;;
            v) VERIFY=true;;
            h) usage;;
            *) echo "Invalid option: -${OPTARG}" >&2
               usage
               ;;
        esac
    done

echo "Exeute the script to get measurement, event log and CC report"

POD_NAME=$(kubectl get po -n ccnp | grep -i ccnp-example | grep Running | awk '{ print $1 }')

if [[ -z "$POD_NAME" ]]; then
    echo "No ccnp-example pod with status running! Please check your deployment."
    exit 1
fi

if [ $MEASUREMENT == true ]; then
    echo "==> Get Measurements"
    kubectl exec -it "$POD_NAME" -n ccnp -- python3 ccnp_example.py -m
fi

if [ $EVENTLOG == true ]; then
    echo "==> Get Event logs"
    kubectl exec -it "$POD_NAME" -n ccnp -- python3 ccnp_example.py -e
fi

if [ $CC_REPORT == true ]; then
    echo "==> Get CC_REPORT"
    kubectl exec -it "$POD_NAME" -n ccnp -- python3 ccnp_example.py -r
fi

if [ $VERIFY == true ]; then
    echo "==> Verify event logs"
    kubectl exec -it "$POD_NAME" -n ccnp -- python3 ccnp_example.py -v
fi
