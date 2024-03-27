# Confidential VM Customization Tool

This tool is plugin-based and used to customize the confidential VM guest to meet user-specific requirements and CCNP deployment requirements.

It provides below plugins with different functions supported.

| Name | Descriptions | Required for CCNP deployment |
| ---- | ------------ | ------------ |
| 01-resize-image | Resize the input qcow2 image | N |
| 02-motd-welcome | Customize the login welcome message | N |
| 03-netplan | Customize the netplan.yaml | N |
| 04-user-authkey | Add auth key for user login instead of password | N |
| 05-readonly-data | Fix some file permission to ready-only | N |
| 06-install-tdx-guest-kernel | Install user-specified TDX guest kernel | Y |
| 07-device-permission | Set the permission for device node | Y |
| 08-ccnp-uds-directory-permission | Set the permission for CCNP UDS directory | Y |
| 09-ccnp-vsock-port | Prepare a VM sockets port for CCNP | Y |
| 60-initrd-update | Update the initrd image | N |
| 97-sample | Plugin customization example | N |
| 98-ima-enable-simple | Enable IMA (Integrity Measurement Architecture) feature | N |


## How to Run the tool

### Prerequisite

1. This tool has been tested on `Ubuntu 22.04`, `Ubuntu 23.10` and `Debian 10`. It is recommend to run the tool on the TDX host prepared following [Configuration](../../README.md/#configuration).

2. This tool can run on bare metal or within a virtual machine with steps described in the section [Run in Nested VM](#run-in-nested-vm-optional).

3. Please install the following packages on Ubuntu/Debian host.

    ```
    sudo apt install qemu-utils guestfs-tools virtinst genisoimage libvirt-daemon-system libvirt-daemon
    ```
    If `guestfs-tools` is not available in your distribution, you may need to install some additional packages on Debian.

    ```
    sudo apt-get install guestfsd libguestfs-tools
    ```

4. Ensure current login user is in the group of libvirt.

    ```
    sudo usermod -aG libvirt $USER
    ```

5. Ensure read permission on `/boot/vmlinuz-$(uname-r)`.

    ```
    sudo chmod o+r /boot/vmlinuz-*
    ```

6. The version of cloud-init is required > 23.0, so if the host distro could not
provide such cloud-init tool, you have to install it manually. For example, on a
debian 10 system, the version of default cloud-init is 20.0. Please do following
steps:
    ```
    wget http://ftp.cn.debian.org/debian/pool/main/c/cloud-init/cloud-init_23.3.1-1_all.deb
    sudo dpkg -i cloud-init_23.3.1-1_all.deb
    ```

7. If it is running with `libvirt/virt-daemon` hypervisor, then:

  - In file `/etc/libvirt/qemu.conf`, make sure `user` and `group` is `root` or
    current user.
  - If need customize the connection URL, you can specify via `-s` like `-s /var/run/libvirt/libvirt-sock`,
    please make sure the current user belongs to the libvirt group via the following commands:
    ```
    sudo usermod -aG libvirt $USER
    sudo systemctl daemon-reload
    sudo systemctl restart libvirtd
    ```

8. Please start the net `default` for libvirt via:

    ```
    virsh net-start default
    ```

### Run the tool

The tool supports parameters as below.
```
$ ./run.sh -h
Usage: run.sh [OPTION]...
Required
  -i <guest image>          Specify initial guest image file
Optional
  -t <number of minutes>    Specify the timeout of rewriting, 3 minutes default,
                            If enabling IMA, recommend timeout >6 minutes
  -s <connection socket>    Default connection URI is qemu:///system,
                            if install libvirt, you can specify to "/var/run/libvirt/libvirt-sock"
                            then the corresponding URI is "qemu+unix:///system?socket=/var/run/libvirt/libvirt-sock"
  -n                        Silent running for virt-install with no output
  -h                        Show usage
```

Run below commands to generate an `output.qcow2` under current directory. The default user name is `tdx`. The password is `123456`.

If you want to skip some plugins, create a file named "NOT_RUN" in the directory of the plugin.

```
# E.g. Skip plugin 98
$ touch plugins/98-ima-enable-simple/NOT_RUN

# Run the tool with an initial guest image and set timeout as 10 minutes.
$ ./run.sh -i <initial guest image> -t 10
```

**NOTE:**
  - All plugins need to be executed in numerical order.
  - Plugin 06, 07, 08 and 09 are required for CCNP deployment.
  - Plugin 60 requires copying or generating all files to the root directory first. When users customize plugins, please ensure that the plugin number with this requirement is placed before 60.
  - Plugin 98 needs to be executed after all other plugins have completed. The number of the user-customized plugin must be before 98.


After the tool is executed successfully, the output will be as below.

```
SUCCESS: Complete cloud-init...
...
SUCCESS: Success to create guest image tools/cvm-image-rewriter/output.qcow2...
...
SUCCESS: Complete.
```


### Boot a VM

After  is running successfully, you can boot a VM using the generated `output.qcow2` using `qemu-test.sh` or `start-virt.sh`.

- Boot TD or normal VM using `qemu-test.sh`.

  ```
  # Boot a TD
  $ sudo ./qemu-test.sh -i output.qcow2 -b grub -q vsock

  # Boot a normal VM
  $ sudo ./qemu-test.sh -i output.qcow2 -b grub -t efi
  ```

- Boot TD using `start-virt.sh`.

  ```
  $ sudo ./start-virt.sh -h
  Usage: start-virt.sh [OPTION]...
    -i <guest image file>     Default is tdx-guest-ubuntu22.04.qcow2 under current directory
    -n <guest name>           Name of TD guest
    -t <template file>        Default is ./tdx-libvirt-ubuntu-host.xml.template
    -f                        Force recreate
    -v <vcpu number>          VM vCPU number
    -m <memory size in GB>    VM memory size in GB
    -h                        Show this help
  ```

  For example:
  ```
  # Boot a TD with specified name and CPU/memory
  $ sudo ./qemu-test.sh -i output.qcow2 -n <libvirt domain name> -v <vCPU number> -m <memory size in GiB>
  ```

### Run in Nested VM (Optional)

This tool can also be run in a guest VM on the host, in case that users need to prepare a clean host environment.  

1. Enable Nested Virtualization

Given that some plugins will consume more time in a low-performance guest VM, it is recommended to enable nested virtualization feature on the host.

First, check if the nested virtualization is enabled. If the file `/sys/module/kvm_intel/parameters/nested` show `Y` or `1`, it indicates that the feature is enabled. 

```
cat /sys/module/kvm_intel/parameters/nested
```

If the feature is not enabled, create the file ` /etc/modprobe.d/kvm.conf`, appending `options kvm_intel nested=1` to it and reboot the host.

```
echo "options kvm_intel nested=1" > /etc/modprobe.d/kvm.conf
```

2. Launch the guest VM

When we launch the guest VM, it is recommended to allocate more than `8G` memory for the guest VM, because this tool will occupy at least `4G` memory. And more CPU cores will improve the guest VM performance, typically the number of CPU cores is at least `4`.

3. Install dependencies

At last, install dependencies in the guest VM before running this tools.

It is an example for a basic Ubuntu 22.04 guest VM.

```
sudo apt install qemu-utils libguestfs-tools virtinst genisoimage cloud-init qemu-kvm libvirt-daemon-system
```
