#!/bin/bash

DOCKER_REPO=docker.io/library
NFD_NAME=node-feature-discovery
NFD_NS=node-feature-discovery
NFD_URL=https://kubernetes-sigs.github.io/node-feature-discovery/charts
WORK_DIR=$(cd "$(dirname "$0")" || exit; pwd)
tag=latest
delete_force=false


function usage {
    cat << EOM
usage: $(basename "$0") [OPTION]...
    -r <registry prefix> the prefix string for registry
    -g <tag> container image tag
    -d Delete existing CCNP and install new CCNP
EOM
    exit 1
}

function process_args {
while getopts ":r:g:hd" option; do
        case "${option}" in
            r) registry=${OPTARG};;
            g) tag=${OPTARG};;
            d) delete_force=true;;
            h) usage;;
            *) echo "Invalid option: -${OPTARG}" >&2
               usage
               ;;
        esac
    done

    if [[ -z "$registry" ]]; then
        echo "Error: Please specify your docker registry via -r <registry prefix>."
            exit 1
    fi
}

function check_env {
    if ! command -v helm &> /dev/null
    then
        echo "Helm could not be found. Please install Helm."
        exit
    fi
    if ! command -v kubectl &> /dev/null
    then
        echo "Kubectl could not be found. Please install K8S."
        exit
    fi
}

function delete_ccnp {
    pushd "${WORK_DIR}/../../.." || exit

    echo "-----------Delete ccnp device plugin and NFD..."
    helm uninstall $NFD_NAME --namespace $NFD_NS
    helm uninstall ccnp-device-plugin

    echo "-----------Delete ccnp server..."
    kubectl delete -f deployment/kubernetes/manifests/ccnp-server-deployment.yaml

    echo "-----------Delete ccnp namespace..."
    kubectl delete -f deployment/kubernetes/manifests/namespace.yaml
    popd || exit
}

function deploy_ccnp {
    pushd "${WORK_DIR}/../../.." || exit

    # Generate temporary yaml files for deployment
    mkdir -p temp_manifests
    cp deployment/kubernetes/manifests/* temp_manifests/

    mkdir temp_manifests/ccnp-device-plugin
    cp -r device-plugin/ccnp-device-plugin/deploy/helm/ccnp-device-plugin/* temp_manifests/ccnp-device-plugin/

    # If private repo is used, modify the images' names in the yaml files
    if [[ -n "$registry" ]]; then
        sed -i  "s#${DOCKER_REPO}#${registry}#g" temp_manifests/*.yaml
        sed -i  "s#${DOCKER_REPO}#${registry}#g" temp_manifests/ccnp-device-plugin/values.yaml
    fi

    if [[ "$tag" != "latest" ]]; then
        sed -i  "s#latest#${tag}#g" temp_manifests/*.yaml
        sed -i  "s#latest#${tag}#g" temp_manifests/ccnp-device-plugin/values.yaml
    fi

    # Deploy CCNP Dependencies
    helm repo add nfd $NFD_URL
    helm repo update
    helm install $NFD_NAME  nfd/node-feature-discovery --namespace $NFD_NS --create-namespace

    kubectl apply -f  device-plugin/ccnp-device-plugin/deploy/node-feature-rules.yaml
    helm install ccnp-device-plugin  temp_manifests/ccnp-device-plugin

    # Deploy CCNP services
    echo "-----------Deploy ccnp namespace..."
    kubectl create -f temp_manifests/namespace.yaml

    echo "-----------Deploy ccnp server..."
    kubectl create -f temp_manifests/ccnp-server-deployment.yaml

    rm -rf temp_manifests
    popd || exit
}

function check_ccnp_deployment {
    # Check CCNP device plugin pod
    echo "-----------Checking ccnp device plugin pod..."
    for i in {1..10}
    do
        DEVICE_POD=$(kubectl get po -n kube-system | grep device | grep Running | awk '{ print $1 }')
        if [[ -z "$DEVICE_POD" ]]
        then
            sleep 3
            echo "Retrying $i time ..."
        else
            break
        fi
    done

    if [ -z "$DEVICE_POD" ]; then
        echo "Error: CCNP device plugin pod is not Running."
        exit 1
    fi
    echo "CCNP device plugin pod $DEVICE_POD is Running."

    # Check CCNP server pod
    echo "-----------Checking ccnp server pod..."
    for i in {1..10}
    do
        CCNP_SERVER_POD=$(kubectl get po -n ccnp | grep ccnp-server | grep Running | awk '{ print $1 }')
        if [[ -z "$CCNP_SERVER_POD" ]]
        then
            sleep 3
            echo "Retrying $i time ..."
        else
            break
        fi
    done

    if [ -z "$CCNP_SERVER_POD" ]; then
        echo "Error: CCNP server pod is not Running."
        exit 1
    fi
    echo "CCNP server pod $CCNP_SERVER_POD is Running."
}

check_env
process_args "$@"

echo ""
echo "-------------------------"
echo "tag: ${tag}"
echo "registry: ${registry}"
echo "delete_force: ${delete_force}"
echo "-------------------------"
echo ""

if [[ $delete_force == true ]]; then
    delete_ccnp
fi

deploy_ccnp
check_ccnp_deployment
