#!/bin/bash

# If we have an argument, it's the directory of the libstoragemgmt source code, so use it
# instead of running against installed packages

# To run locally, do:
# podman pull fedora
# Run the following in the root of the golang source tree
# podman run --privileged --rm=false --tty=true --interactive=true -v \
#    `pwd`:/libstoragemgmt-rust:rw fedora \
#    /bin/bash -c "cd /libstoragemgmt-rust && pwd && tests/ci_test.sh"

if [ -z "$@" ]; then
    if [ "CHK$(rpm -E "%{?fedora}")" != "CHK" ];then
        dnf install libstoragemgmt python3-six cargo rust -y || exit 1
    elif [ "CHK$(rpm -E "%{?el8}")" != "CHK" ];then
        dnf install dnf-plugins-core -y || exit 1
	dnf config-manager --set-enabled ol8_codeready_builder -y || exit 1
        dnf install libstoragemgmt python3-six cargo rust -y || exit 1
    else
        echo "Unsupported distribution"
        exit 1
    fi

    # Start the service
    /usr/bin/lsmd || exit 1

    # Let the service get ready
    sleep 5 || exit 1

    # Make sure things are sane
    lsmcli list --type pools -u simc:// || exit 1
    lsmcli list --type plugins -u simc:// || exit 1

else

    pwd

    # If we are running against a source directory, we assume daemon already running for us.
    export LSMSRC="$1"

    if [ "CHK$(rpm -E "%{?fedora}")" != "CHK" ];then
        dnf install cargo rust -y || exit 1
    elif [ "CHK$(rpm -E "%{?el8}")" != "CHK" ];then
        dnf install dnf-plugins-core -y || exit 1
	dnf config-manager --set-enabled ol8_codeready_builder -y || exit 1
        dnf install cargo rust -y || exit 1
    else
        echo "Unable to run rust-lang test for this distro, skipping..."
        exit 0
    fi

fi

# Speed up tests
export LSM_SIM_TIME=0

cargo test -- --test-threads=1 || exit 1
