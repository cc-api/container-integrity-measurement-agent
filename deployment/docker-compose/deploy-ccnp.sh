#!/bin/bash

set -e

TAG="latest"
CCNP_SERVER_IMAGE="ccnp-server"
REGISTRY=""

DIR=$(dirname "$(readlink -f "$0")")

#
# Display Usage information
#
usage() {

    cat <<EOM
Usage: $(basename "$0") [OPTION]...
    -r <registry prefix>    the prefix string for registry
    -g <tag>                container image tag
    -h                      show help info
EOM
}

process_args() {
    while getopts ":r:g:h" option; do
        case "$option" in
        r) REGISTRY=$OPTARG ;;
        g) TAG=$OPTARG ;;
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

    CCNP_SERVER_IMAGE="$CCNP_SERVER_IMAGE:$TAG"

    if [[ ${REGISTRY: -1} == "/" ]]; then
        REGISTRY="${REGISTRY%/}"
    fi
    if [[ $REGISTRY != "" ]]; then
        CCNP_SERVER_IMAGE="$REGISTRY/$CCNP_SERVER_IMAGE"
    fi
}

process_args "$@"

# shellcheck disable=SC1091
. "$DIR"/scripts/cache.sh
create_cache_dir

# shellcheck disable=SC1091
. "$DIR"/scripts/docker_compose.sh
create_composes "$CCNP_SERVER_IMAGE"

docker_compose_up
