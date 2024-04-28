#!/bin/bash

DOCKER_REPO=docker.io/library
NFD_NAME=node-feature-discovery
NFD_NS=node-feature-discovery
NFD_URL=https://kubernetes-sigs.github.io/node-feature-discovery/charts
CERT_MANAGER_URL=https://github.com/cert-manager/cert-manager/releases/download/v1.14.5/cert-manager.yaml
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

    echo "-----------Delete ccnp webhook and server..."
    kubectl delete -f deployment/kubernetes/manifests/ccnp-webhook-deployment.yaml
    kubectl delete -f deployment/kubernetes/manifests/ccnp-server-deployment.yaml

    echo "-----------Delete ccnp namespace..."
    kubectl delete -f deployment/kubernetes/manifests/namespace.yaml

    echo "-----------Delete NFD, cert-manager..."
    helm uninstall $NFD_NAME --namespace $NFD_NS
    kubectl delete -f $CERT_MANAGER_URL

    popd || exit
}

function deploy_ccnp {
    pushd "${WORK_DIR}/../../.." || exit

    # Generate temporary yaml files for deployment
    mkdir -p temp_manifests
    cp -r deployment/kubernetes/manifests/* temp_manifests/

    # If private repo is used, modify the images' names in the yaml files
    if [[ -n "$registry" ]]; then
        sed -i  "s#${DOCKER_REPO}#${registry}#g" temp_manifests/*.yaml
    fi

    if [[ "$tag" != "latest" ]]; then
        sed -i  "s#latest#${tag}#g" temp_manifests/*.yaml
    fi

    # Deploy CCNP Dependencies
    helm repo add nfd $NFD_URL
    helm repo update
    helm install $NFD_NAME  nfd/node-feature-discovery --namespace $NFD_NS --create-namespace
    kubectl create -f $CERT_MANAGER_URL

    # Check the cert manager
    curl -fsSL -o cmctl  https://github.com/cert-manager/cmctl/releases/download/v2.0.0/cmctl_linux_amd64
    chmod +x cmctl
    while :
    do
        CMCTL=$(./cmctl check api | grep "is ready")
        if [[ -z "$CMCTL" ]]
        then
            echo "cert-manager is not ready, try again..."
            sleep 5
        else
            break
        fi
    done
    rm cmctl

    # Deploy CCNP webhook
    echo "-----------Deploy ccnp namespace..."
    kubectl create -f temp_manifests/namespace.yaml
    kubectl create -f temp_manifests/ccnp-webhook-deployment.yaml

    # Deploy CCNP services
    echo "-----------Deploy ccnp server..."
    kubectl create -f temp_manifests/ccnp-server-deployment.yaml

    rm -rf temp_manifests
    popd || exit
}

function check_ccnp_deployment {
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
