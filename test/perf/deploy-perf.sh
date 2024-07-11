#!/bin/bash
# Script to deploy multiple CCNP example pod for perf

set -e

DEFAULT_DOCKER_REPO=docker.io/library
DEFAULT_TAG=latest
WORK_DIR=$(cd "$(dirname "$0")" || exit; pwd)
TEMP_MANIFEST_FILE=/tmp/ccnp-example-perf-deployment.yaml
delete_deployment=false
number=1
registry=""
tag=""


usage() {
    cat << EOM
usage: $(basename "$0") [OPTION]...
    -n Number of pods to run parallel perf testing
    -r Image registry
    -g Image tag
    -d Delete perf example pods
EOM
    exit 1
}

process_args() {
    while getopts ":r:g:i:n:dh" option; do
        case "${option}" in
            r) registry=${OPTARG};;
            g) tag=${OPTARG};;
            d) delete_deployment=true;;
            n) number=${OPTARG};;
            h) usage;;
            *) echo "Invalid option: -${OPTARG}" >&2
            usage
            ;;
        esac
    done
}

deploy_perf() {
    echo "Deploy pods for performance testing "
    pushd "${WORK_DIR}/../.." || exit
    # replace registry and image tag according to user input
    cp deployment/kubernetes/manifests/ccnp-example-perf-deployment.yaml $TEMP_MANIFEST_FILE
    if [[ -n "$registry" ]]; then
            sed -i  "s#${DEFAULT_DOCKER_REPO}#${registry}#g" $TEMP_MANIFEST_FILE
    fi
    if [[ -n "$tag" ]];then
            sed -i "s#${DEFAULT_TAG}#${tag}#g" $TEMP_MANIFEST_FILE
    fi

    # Loop to deploy deployments
    for ((i=1; i<=number; i++)); do
    # Replace the placeholder in the template with the deployment name
    sed "s/PLACEHOLDER/$i/g" $TEMP_MANIFEST_FILE > /tmp/ccnp-perf-example-$i.yaml

    # Apply the deployment to Kubernetes
    kubectl apply -f /tmp/ccnp-perf-example-$i.yaml
    done

    echo "==> Checking ccnp-example deployment are ready"
    for i in {1..10}
    do
        pod_num=$(kubectl get po -n ccnp | grep perf-example | grep -i -c running)
        if [ "$pod_num" -eq "$number" ]
        then
            echo "Perf example pods are ready"
            break
        else
            sleep 2
            echo "Not ready yet ..."
        fi
    done
    popd || exit
}

# Delete all the perf deployment
delete_perf() {
    for (( i=1; i<=number; i++ ))
        do kubectl delete deployment ccnp-perf-example-$i -n ccnp;
        echo "Deleting deployment ccnp-perf-example-$i ..."
    done

    echo "==> Checking ccnp-example deployment are deleted"
    for i in {1..10}
    do
        pod_num=$(kubectl get deployment -n ccnp | grep -c perf-example)
        if [ "$pod_num" -eq 0 ]
        then
            echo "Perf example pods are deleted"
            break
        else
            sleep 2
            echo "Not all deleted yet ..."
        fi
    done
}

process_args "$@"

echo ""
echo "-------------------------"
echo "Number of pods for perf testing: ${number}"
echo "Image registry: ${registry}"
echo "Image tag: ${tag}"
echo "delete pods: ${delete_deployment}"
echo "-------------------------"
echo ""

if [[ $delete_deployment == true ]]; then
    delete_perf
fi

if [[ -n "$registry" && -n "$tag" ]]; then
    deploy_perf
else
    echo "Image registry and tag are not set."
fi
