# https://github.com/dockur/windows

# Skip the download and use a local iso file instead
# --volume /home/capric/runner/tiny10x6423h2.iso:/custom.iso

# Open 'File Explorer' and click on the 'Network' section, you will see a computer called host.lan. Double-click it and it will show a folder called Data
# --volume /home/capric/runner/data:/data

# The folder will be copied to C:\OEM during installation and the containing install.bat will be executed during the last step
# --volume /home/capric/runner/x86_64_windows/oem:/oem

# To change the storage location
# --volume /home/capric/runner/x86_64_windows/storage:/storage

# Windows remote desktop port
# --publish 3389:3389

# QEMU noVNC port
# --publish 8006:8006

# System memory size
# --env RAM_SIZE="4G"

# System cpu cores
# --env CPU_CORES="4"

# System disk size
# --env DISK_SIZE="16G"

# Http proxy
# --env HTTP_PROXY=$HTTP_PROXY

# Https proxy
# --env HTTPS_PROXY=$HTTPS_PROXY

# No proxy
# --env NO_PROXY="localhost,127.0.0.1"

# Windows user name
# --env USERNAME=builder

# Windows user password
# --env PASSWORD=redliub

RUNNER_DIR=/mnt/d/runner
RUNNER_DATA_DIR=$RUNNER_DIR/data
RUNNER_ISO_PATH=$RUNNER_DIR/tiny10x6423h2.iso
RUNNER_OS_DIR=$RUNNER_DIR/x86_64_windows
RUNNER_OS_OEM_DIR=$RUNNER_OS_DIR/oem
RUNNER_OS_OEM_STORAGE=$RUNNER_OS_DIR/storage

PORT_MSTSC=3389:3389
PORT_NOVNC=8006:8006

RAM_SIZE="4G"
CPU_CORES="4"
DISK_SIZE="20G"

USERNAME="builder"
PASSWORD="redliub"

sudo docker run                                 \
    --tty                                       \
    --interactive                               \
    --volume $RUNNER_ISO_PATH:/custom.iso       \
    --volume $RUNNER_DATA_DIR:/data             \
    --volume $RUNNER_OS_OEM_DIR:/oem            \
    --volume $RUNNER_OS_OEM_STORAGE:/storage    \
    --publish $PORT_MSTSC                       \
    --publish $PORT_NOVNC                       \
    --env RAM_SIZE=$RAM_SIZE                    \
    --env CPU_CORES=$CPU_CORES                  \
    --env DISK_SIZE=$DISK_SIZE                  \
    --env HTTP_PROXY=$HTTP_PROXY                \
    --env HTTPS_PROXY=$HTTPS_PROXY              \
    --env NO_PROXY="localhost,127.0.0.1"        \
    --env USERNAME=$USERNAME                    \
    --env PASSWORD=$PASSWORD                    \
    --device=/dev/kvm                           \
    --device=/dev/net/tun                       \
    --cap-add NET_ADMIN                         \
    --stop-timeout 120                          \
    dockurr/windows
