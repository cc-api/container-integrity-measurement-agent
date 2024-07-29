#!/bin/bash

set -e

CIMA_VERSION_SUFFIX="+cima1"
BASE_KERNEL_VERSION="6.8.0-31-generic"
if [ -n "${TDX_SETUP_INTEL_KERNEL}" ]; then
  BASE_KERNEL_VERSION="6.8.0-1001-intel"
fi

CUR_DIR=$(dirname "$(readlink -f "$0")")
KERNEL_DIR=${CUR_DIR}/kernel
TMP_DIR=$(mktemp -d /tmp/cima_build.XXXXXX)
OUT_DIR=${CUR_DIR}/output

patch_kernel() {
    for p in "${KERNEL_DIR}"/patches/*
    do
        patch -p1 -F1 -s < "${p}"
    done
}

build_ubuntu_kernel() {
    # Add apt repository source
    add-apt-repository -s -y ppa:kobuk-team/tdx
    sed -i 's/^Types: deb$/Types: deb deb-src/' /etc/apt/sources.list.d/ubuntu.sources \
        /etc/apt/sources.list.d/kobuk-team-ubuntu-tdx-noble.sources
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
    sed -i "/CONFIG_TDX_GUEST_DRIVER/ r ${KERNEL_DIR}/ubuntu/annotations" \
        debian.master/config/annotations
    # Change kernel version in changelog
    if [ -f "debian.intel/changelog" ]; then
        CHANGELOG="debian.intel/changelog"
    else
        CHANGELOG="debian.master/changelog"
    fi

    # For generic kernel, the default version in changelog is linux (6.8.0-31.31),
    #   we want to change to cima version linux (6.8.0-31.31+cima1)
    # For intel kernel, the default version in changelog is linux-intel (6.8.0-1001.7)
    #   we want to change it to linux-intel (6.8.0-1001.7+cima1)
    LATEST_VERSION=$(sed -n '1 s/\(linux.*(.*\)) noble.*$/\1/p' ${CHANGELOG})
    CIMA_VERSION="${LATEST_VERSION}${CIMA_VERSION_SUFFIX})"
    sed "s/CIMA_VERSION/${CIMA_VERSION}/" \
        "${KERNEL_DIR}/ubuntu/changelog" > "${KERNEL_DIR}/ubuntu/changelog.tmp"
    sed -i "0 r ${KERNEL_DIR}/ubuntu/changelog.tmp" debian/changelog ${CHANGELOG}
    rm "${KERNEL_DIR}/ubuntu/changelog.tmp"

    dpkg-buildpackage -us -uc -ui -b
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
