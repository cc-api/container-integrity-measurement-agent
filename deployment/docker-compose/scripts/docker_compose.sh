#!/bin/bash

DIR=$(dirname "$(readlink -f "$0")")
# shellcheck disable=SC1091
. "$DIR"/scripts/device.sh

CONFIG_DIR="$DIR/configs"

create_composes() {
    CCNP_SERVER_IMAGE=$1

    DEV_TDX=$(check_dev_tdx)

    sed "s@\#CCNP_SERVER_IMAGE@$CCNP_SERVER_IMAGE@g" "$CONFIG_DIR"/ccnp-compose.yaml.template \
        > "$COMPOSE_CACHE_DIR"/ccnp-compose.yaml
    
    sed -i "s@\#DEV_TDX@$DEV_TDX@g" "$COMPOSE_CACHE_DIR"/ccnp-compose.yaml

}

docker_compose_up() {
    if ! [ -d "$COMPOSE_CACHE_DIR" ]; then
        error "Compose Cache Dir not Exist: $COMPOSE_CACHE_DIR"
    fi

    CONFIGS=("$COMPOSE_CACHE_DIR"/*)
    for config in "${CONFIGS[@]}"
    do
        info "Compose $config Being Deployed"
    	docker compose -f "$config" up -d 
        ok "Compose $config Deployed"
    done
}

docker_compose_down() {
    if ! [ -d "$COMPOSE_CACHE_DIR" ]; then
        error "Compose Cache Dir not Exist: $COMPOSE_CACHE_DIR"
    fi
    
    CONFIGS=("$COMPOSE_CACHE_DIR"/*)
    for config in "${CONFIGS[@]}"
    do
        name_line=$(head -1 "$config")
        name="${name_line#name:}"
        # shellcheck disable=SC2086
        compose=$(docker compose ls | grep $name || true)
        if [[ "$compose" == "" ]]; then
            continue
        fi
        info "Compose $config Being Down"
            docker compose -f "$config"  down
        ok "Compose $config Down"
    done
}
