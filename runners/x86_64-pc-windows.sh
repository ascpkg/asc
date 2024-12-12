# https://github.com/dockur/windows

sudo docker run                                                   \
    --tty                                                         \
    --interactive                                                 \
    --volume /mnt/c/tiny10x6423h2.iso:/custom.iso                 \
    --volume /home/capric/runner/x86_64_windows/oem:/oem          \
    --volume /home/capric/runner/x86_64_windows/data:/data        \
    --volume /home/capric/runner/x86_64_windows/storage:/storage  \
    --publish 3389:3389                                           \
    --publish 8006:8006                                           \
    --publish 8000:8000                                           \
    --env RAM_SIZE="4G"                                           \
    --env CPU_CORES="4"                                           \
    --env HTTP_PROXY=$HTTP_PROXY                                  \
    --env HTTPS_PROXY=$HTTPS_PROXY                                \
    --env NO_PROXY="localhost,127.0.0.1"                          \
    --env USERNAME=builder                                        \
    --env PASSWORD=redliub                                        \
    --device=/dev/kvm                                             \
    --device=/dev/net/tun                                         \
    --cap-add NET_ADMIN                                           \
    --stop-timeout 120                                            \
    dockurr/windows
