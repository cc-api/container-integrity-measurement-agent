#!/bin/bash

CURR_DIR=$(dirname "$(readlink -f "$0")")
TOP_DIR="${CURR_DIR}/../../../"
SCRIPTS_DIR="${TOP_DIR}/scripts"
# shellcheck disable=SC1091
source "${SCRIPTS_DIR}/common.sh"
ARTIFACTS_GUEST=/srv

# check environment variable 'CVM_TDX_GUEST_REPO'
if [[ -z "$CVM_TDX_GUEST_REPO" ]]; then
    warn "SKIP: TDX guest repo is not defined via environment variable 'CVM_TDX_GUEST_REPO' "
    exit 0
fi

info "TDX Guest Repo is at ${CVM_TDX_GUEST_REPO}..."

# check if the repo exists
if [[ ! -d "$CVM_TDX_GUEST_REPO" ]]; then
    warn "SKIP: TDX guest local repo CVM_TDX_GUEST_REPO does not exist."
    exit 0
fi

# Check if it is a valid TDX repo
if ! compgen -G "$CVM_TDX_GUEST_REPO/linux-image-*intel-opt*.deb"; then
    warn "SKIP: $CVM_TDX_GUEST_REPO is invalid."
    exit 0
fi

info "TDX guest local repo $CVM_TDX_GUEST_REPO check passed"

# Copy TDX local repo from host to guest
virt-copy-in -a "${GUEST_IMG}" "$CVM_TDX_GUEST_REPO" "$ARTIFACTS_GUEST"
ok "TDX guest local repo $CVM_TDX_GUEST_REPO copied to guest $ARTIFACTS_GUEST"

# Generate cloud-config
mkdir -p "${CURR_DIR}/../cloud-init/x-shellscript/"
cat > "${CURR_DIR}/../cloud-init/x-shellscript/07-install-tdx-guest-kernel.sh" << EOL
#!/bin/bash

PACKAGE_DIR=""$ARTIFACTS_GUEST"/$(basename "$CVM_TDX_GUEST_REPO")/"
pushd \$PACKAGE_DIR || exit 0
DEBIAN_FRONTEND=noninteractive apt install ./linux-image-unsigned-*.deb \
./linux-modules-*.deb ./linux-headers-*.deb \
./linux-intel-opt-headers-*.deb --allow-downgrades -y 
popd || exit 0
EOL

ok "Cloud config cloud-init/x-shellscript/07-install-tdx-guest-kernel.sh generated"
