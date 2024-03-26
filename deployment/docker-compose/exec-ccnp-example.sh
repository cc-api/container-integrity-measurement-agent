#!/bin/bash

set -e

DIR=$(dirname "$(readlink -f "$0")")
# shellcheck disable=SC1091
. "$DIR"/scripts/device.sh
CONFIG_DIR="$DIR"/configs 	  

EXAMPLE_IMAGE=ccnp-example
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
    docker compose -f "$COMPOSE_CACHE_DIR"/ccnp-node-measurement-example.yaml down
    ok "Example Container Deleted"
}

validate_on_container() {
    info "Execute example Container ccnp-example"
    ctr_id=$(docker ps | grep ccnp-example-ctr | awk '{print $1}')    
    if [[ "$ctr_id" == "" ]]; then
   	info "Example Container No Avaliable. Attempt Deploy It"
        sed "s@\#EXAMPLE_IMAGE@$EXAMPLE_IMAGE@g" "$CONFIG_DIR"/ccnp-example.yaml.template \
                    > "$COMPOSE_CACHE_DIR"/ccnp-example.yaml
        sed -i "s@\#DEV_TDX@$DEV_TDX@g" "$COMPOSE_CACHE_DIR"/ccnp-example.yaml
        docker compose -f "$COMPOSE_CACHE_DIR"/ccnp-example.yaml up -d
    fi

    ctr_id=$(docker ps | grep ccnp-example-ctr | awk '{print $1}')
    if [[ "$ctr_id" == "" ]]; then
       error "Example Container Deploy Failed"
    fi

    ok "Example Container Avaliable. Compose file: $COMPOSE_CACHE_DIR/ccnp-example.yaml"
    ok "=============== Get Measurement ==============="
    docker exec -it "$ctr_id" python3 ccnp_example.py -m > "$CCNP_CACHE_DIR"/example.log
    ok "Measurement Log Saved in File $CCNP_CACHE_DIR/example.log"

    ok "=============== Get Event Logs ==============="
    docker exec -it "$ctr_id" python3 ccnp_example.py -e >> "$CCNP_CACHE_DIR"/example.log
    ok "Eventlog Saved in File $CCNP_CACHE_DIR/example.log"

    ok "=============== Get CC Report ==============="
    docker exec -it "$ctr_id" python3 ccnp_example.py -r >> "$CCNP_CACHE_DIR"/example.log
    ok "Eventlog Saved in File $CCNP_CACHE_DIR/example.log"

    ok "=============== Verify Event Logs ==============="
    docker exec -it "$ctr_id" python3 ccnp_example.py -v >> "$CCNP_CACHE_DIR"/example.log
    ok "Eventlog Saved in File $CCNP_CACHE_DIR/example.log"
}

process_args "$@"

validate_on_container
delete_example_ctr

