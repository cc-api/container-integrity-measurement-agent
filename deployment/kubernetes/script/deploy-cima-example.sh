#!/bin/bash
# Script to deploy CIMA example pod

set -e

DEFAULT_DOCKER_REPO=docker.io/library
DEFAULT_TAG=latest
WORK_DIR=$(cd "$(dirname "$0")" || exit; pwd)
TEMP_MANIFEST_FILE=/tmp/cima-example-deployment.yaml
DELETE_DEPLOYMENT=false


usage() { echo "Usage: $0 [-r <registry-prefix>] [-g <image-tag>] [-d delete existing deployment] [-m get measurement] [-e get event logs] [-q get cc report] [-v verify event logs]"; exit 1; }
while getopts ":r:g:i:dmervh" option; do
        case "${option}" in
            r) registry=${OPTARG};;
            g) tag=${OPTARG};;
            d) DELETE_DEPLOYMENT=true;;
            h) usage;;
            *) echo "Invalid option: -${OPTARG}" >&2
               usage
               ;;
        esac
    done

echo "Deploy CIMA example for container measurement in Kubernetes"
pushd "${WORK_DIR}/../../.." || exit
# replace registry and image tag according to user input
cp deployment/kubernetes/manifests/cima-example-deployment.yaml $TEMP_MANIFEST_FILE
if [[ -n "$registry" ]]; then
	sed -i  "s#${DEFAULT_DOCKER_REPO}#${registry}#g" $TEMP_MANIFEST_FILE
fi
if [[ -n "$tag" ]];then
	sed -i "s#${DEFAULT_TAG}#${tag}#g" $TEMP_MANIFEST_FILE
fi

# Delete old pod if it exists
OLD_POD_NAME=$(kubectl get po -n cima | grep cima-example | grep Running | awk '{ print $1 }')

if [[ $DELETE_DEPLOYMENT == true ]] && [[ -n "$OLD_POD_NAME" ]]; then
    echo "==> Cleaning up cima-example deployment"
    kubectl delete deployment cima-example -n cima
fi

echo "==> Creating cima-example deployment"
kubectl apply -f $TEMP_MANIFEST_FILE
for i in {1..10}
do
    POD_NAME=$(kubectl get po -n cima | grep cima-example | grep Running | awk '{ print $1 }')
    if [[ -z "$POD_NAME" ]]
    then
        sleep 3
        echo "Retrying $i time ..."
    else
        break
    fi
done

if [[ -z "$POD_NAME" ]]; then
    echo "No cima-example pod with status running! Please check your deployment."
    exit 1
fi
echo "CIMA example pod $POD_NAME is Running."

popd || exit
