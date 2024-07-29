#!/bin/bash

set -e

DIR=$(dirname "$(readlink -f "$0")")
# shellcheck disable=SC1091
. "$DIR"/scripts/device.sh
CONFIG_DIR="$DIR"/configs 	  

EXAMPLE_IMAGE=cima-example
TAG=latest
REGISTRY=""
DEV_TDX="/dev/tdx_guest"
DELETE_CTR=false

#
# Display Usage information
#
usage() {
    cat <<EOM
Usage: $(basename "$0") [OPTION]...
    -r <registry prefix>    the prefix string for registry
    -g <tag>                container image tag
    -d                      delete example container
    -h                      show help info
EOM
}

process_args() {
    while getopts ":r:g:dh" option; do
        case "$option" in
        r) REGISTRY=$OPTARG ;;
        g) TAG=$OPTARG ;;
	    d) DELETE_CTR=true ;;
        h)
            usage
            exit 0
            ;;
        *)
            echo "Invalid option '-$OPTARG'"
            usage
            exit 1
            ;;
        esac
    done

    EXAMPLE_IMAGE="$EXAMPLE_IMAGE:$TAG"

    if [[ "${REGISTRY: -1}" == "/" ]]; then
        REGISTRY="${REGISTRY%/}"
    fi
    if [[ "$REGISTRY" != "" ]]; then
        EXAMPLE_IMAGE="$REGISTRY/$EXAMPLE_IMAGE"
    fi

    DEV_TDX=$(check_dev_tdx)
}

delete_example_ctr() {
    if [[ "$DELETE_CTR" == "false" ]]; then
        return
    fi

    info "Example Container Being Deleted"
    docker compose -f "$COMPOSE_CACHE_DIR"/cima-node-measurement-example.yaml down
    ok "Example Container Deleted"
}

validate_on_container() {
    info "Execute example Container cima-example"
    ctr_id=$(docker ps | grep cima-example-ctr | awk '{print $1}')    
    if [[ "$ctr_id" == "" ]]; then
   	info "Example Container is NOT Avaliable. Deploying Example Container"
        sed "s@\#EXAMPLE_IMAGE@$EXAMPLE_IMAGE@g" "$CONFIG_DIR"/cima-example.yaml.template \
                    > "$COMPOSE_CACHE_DIR"/cima-example.yaml
        sed -i "s@\#DEV_TDX@$DEV_TDX@g" "$COMPOSE_CACHE_DIR"/cima-example.yaml
        docker compose -f "$COMPOSE_CACHE_DIR"/cima-example.yaml up -d
    fi

    ctr_id=$(docker ps | grep cima-example-ctr | awk '{print $1}')
    if [[ "$ctr_id" == "" ]]; then
       error "Fail to deploy Example Container"
    fi

    ok "Example Container Avaliable. Compose file: $COMPOSE_CACHE_DIR/cima-example.yaml"
    ok "=============== Get Measurement ==============="
    docker exec -it "$ctr_id" python3 py_sdk_example.py -m > "$CIMA_CACHE_DIR"/example.log
    ok "Measurement is saved in file $CIMA_CACHE_DIR/example.log"

    ok "=============== Get Event Logs ==============="
    docker exec -it "$ctr_id" python3 py_sdk_example.py -e >> "$CIMA_CACHE_DIR"/example.log
    ok "Eventlog is saved in file $CIMA_CACHE_DIR/example.log"

    ok "=============== Get CC Report ==============="
    docker exec -it "$ctr_id" python3 py_sdk_example.py -r >> "$CIMA_CACHE_DIR"/example.log
    ok "CC Report is saved in file $CIMA_CACHE_DIR/example.log"

    ok "=============== Verify Event Logs ==============="
    docker exec -it "$ctr_id" python3 py_sdk_example.py -v >> "$CIMA_CACHE_DIR"/example.log
    ok "Eventlog is verified in file $CIMA_CACHE_DIR/example.log"
}

process_args "$@"

validate_on_container
delete_example_ctr

