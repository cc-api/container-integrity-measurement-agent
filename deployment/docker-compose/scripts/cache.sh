#!/bin/bash

DIR=$(dirname "$(readlink -f "$0")")
# shellcheck disable=SC1091
. "$DIR"/scripts/comm.sh

check_cache_dir() {
    if [[ -d "$CIMA_CACHE_DIR" ]]; then
    	error "Cache Dir $CIMA_CACHE_DIR Exists. Please Back & Delete It"
    fi
}

create_cache_dir() {
    info "Cache Dir Being Created: $CIMA_CACHE_DIR"
    mkdir -p "$CIMA_CACHE_DIR"
    mkdir -p "$CIMA_CACHE_DIR/run/cima-eventlog"
    mkdir -p "$CIMA_CACHE_DIR/run/cima/uds"
    mkdir -p "$CIMA_CACHE_DIR/eventlog-entry-dir"
    mkdir -p "$CIMA_CACHE_DIR/eventlog-data-dir"
    mkdir -p "$COMPOSE_CACHE_DIR"

    chmod 777 -R "$CIMA_CACHE_DIR"
    ok "Cache Dir Created: $CIMA_CACHE_DIR"
}

remove_cache_dir() {
    info "Cache Dir Being Removed"
    if [[ -d "$CIMA_CACHE_DIR" ]]; then
    	rm -rf "$CIMA_CACHE_DIR"
    fi
    ok "Cache Dir Removed"
}
