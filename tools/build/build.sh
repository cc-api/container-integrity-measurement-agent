#!/bin/bash

set -e

BASE_KERNEL_VERSION="6.5.0-1003-intel-opt"

CUR_DIR=$(dirname "$(readlink -f "$0")")
KERNEL_DIR=${CUR_DIR}/kernel
TMP_DIR=$(mktemp -d /tmp/ccnp_build.XXXXXX)
OUT_DIR=${CUR_DIR}/output

patch_kernel() {
    for p in "${KERNEL_DIR}"/patches/*
    do
        patch -p1 -F1 -s < "${p}"
    done
}

build_ubuntu_kernel() {
    # Add apt repository source
    add-apt-repository -s -y ppa:kobuk-team/tdx-release
    # Install the build dependencies
    DEBIAN_FRONTEND=noninteractive apt update && apt install -y devscripts && \
    apt build-dep -y linux-image-unsigned-"${BASE_KERNEL_VERSION}"
    # Download the source codes
    apt source linux-image-unsigned-"${BASE_KERNEL_VERSION}"
    # A workaround to fix build issue of DKMS
    mv /lib/modules/"$(uname -r)"/modules.dep /lib/modules/"$(uname -r)"/modules.dep.bk
    touch /lib/modules/"$(uname -r)"/modules.dep

    pushd linux-*/
    patch_kernel
    # Add new configs in the patch
    sed -i "/CONFIG_TDX_GUEST_DRIVER *note.*/ r ${KERNEL_DIR}/ubuntu/annotations" \
        debian.intel-opt/config/annotations
    # Change kernel version in changelog
    sed -i "0 r ${KERNEL_DIR}/ubuntu/changelog" debian/changelog debian.intel-opt/changelog

    debuild -uc -us -b
    popd

    mv ./*.deb "${OUT_DIR}"/
    # Restore the environemnt
    mv /lib/modules/"$(uname -r)"/modules.dep.bk /lib/modules/"$(uname -r)"/modules.dep
}

build_packages() {
    [[ -d ${OUT_DIR} ]] || mkdir "${OUT_DIR}"
    pushd "${TMP_DIR}"
    if grep -q "Ubuntu" /etc/os-release; then
        echo "Building Ubuntu packages..."
        build_ubuntu_kernel
    else
        echo "The system is not supported yet."
    fi
    popd
}

clean_up() {
    echo "Clean up build environment..."
    [[ ! -f /lib/modules/$(uname -r)/modules.dep.bk ]] || \
        mv /lib/modules/"$(uname -r)"/modules.dep.bk /lib/modules/"$(uname -r)"/modules.dep
    rm -rf "${TMP_DIR}"
}

trap clean_up EXIT
build_packages
