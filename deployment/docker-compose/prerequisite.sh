#!/bin/bash

set -e

DIR=$(dirname "$(readlink -f "$0")")

# shellcheck disable=SC1091
. "$DIR"/scripts/cache.sh
check_cache_dir
ok "Cache Dir Clear"

# shellcheck disable=SC1091
. "$DIR"/scripts/device.sh
grant_dev_tdx
ok "Dev TDX Valid"

info "Make Sure Service QGS&PCCS is Avaliable to Get Quote"


function install_docker {
    echo "========= Install Docker ==========="
    # install GPG key
    install -m 0755 -d /etc/apt/keyrings
    rm -f /etc/apt/keyrings/docker.gpg
    curl -fsSL https://download.docker.com/linux/ubuntu/gpg | gpg --dearmor -o /etc/apt/keyrings/docker.gpg
    chmod a+r /etc/apt/keyrings/docker.gpg

    # install repo
    # shellcheck disable=SC1091
    echo \
    "deb [arch=\"$(dpkg --print-architecture)\" signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
    $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | \
    tee /etc/apt/sources.list.d/docker.list > /dev/null
    apt-get update > /dev/null

    # install docker
    apt-get install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin

    # start docker
    systemctl enable docker
    systemctl daemon-reload
    systemctl start docker
}

if command -v docker &> /dev/null; then
    echo "Skip: Docker has been installed."
else
    install_docker
fi

