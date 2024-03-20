#!/bin/bash

set -e

CUR_DIR=$(dirname "$(readlink -f "$0")")
KERNEL_PATCHES_DIR=${CUR_DIR}/kernel
TMP_DIR=$(mktemp -d /tmp/ccnp_build.XXXXXX)
OUT_DIR=${CUR_DIR}/output

KERNEL_CONFIG_ANNOTATIONS=$(cat << EOF

CONFIG_TSM_REPORTS                              policy<{'amd64': 'm'}>
CONFIG_TSM_REPORTS                              note<'Required for ConfigFS TSM support'>

CONFIG_IMA_CGPATH_TEMPLATE                      policy<{'amd64': 'n'}>
CONFIG_IMA_CGPATH_TEMPLATE                      note<'CGPATH for CCNP container measurement'>

CONFIG_IMA_DEP_CGN_TEMPLATE                     policy<{'amd64': 'n'}>
CONFIG_IMA_DEP_CGN_TEMPLATE                     note<'CGN for CCNP container measurement'>
EOF
)

patch_kernel() {
    for p in "${KERNEL_PATCHES_DIR}"/*
    do
        patch -p1 -F1 -s < "${p}"
    done
}

build_ubuntu_kernel() {
    # Add apt repository source
    add-apt-repository -s -y ppa:kobuk-team/tdx-release
    # Install the build dependencies
    DEBIAN_FRONTEND=noninteractive apt update && apt install -y devscripts && \
    apt build-dep -y linux-image-unsigned-"$(uname -r)"
    # Download the source codes
    apt source linux-image-unsigned-"$(uname -r)"
    # A workaround to fix build issue of DKMS
    mv /lib/modules/"$(uname -r)"/modules.dep /lib/modules/"$(uname -r)"/modules.dep.bk
    touch /lib/modules/"$(uname -r)"/modules.dep

    pushd linux-*/
    patch_kernel
    # Add new configs in the patch
    echo "${KERNEL_CONFIG_ANNOTATIONS}" | sed -i "/CONFIG_TDX_GUEST_DRIVER *note.*/ r /dev/stdin" \
        debian.intel-opt/config/annotations

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
