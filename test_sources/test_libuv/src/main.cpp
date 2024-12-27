// c
#include <stdint.h>
#include <time.h>

// c++
#include <chrono>
#include <string>
#include <thread>
#include <vector>

// libuv
#include <uv.h>

// spdlog
#include <spdlog/spdlog.h>

// project
#include "config.h"
#include "version.h"


class UdpSock
{
public:
    UdpSock() {}
    virtual ~UdpSock() {}

    bool setup(std::string ip, int port, uv_loop_t *uv_loop, uv_udp_recv_cb recv_cb) {
        do {
            if (nullptr == uv_loop) {
                SPDLOG_ERROR("nullptr == uv_loop");
                break;
            }

            int error_code = uv_udp_init(uv_loop, &m_udp_socket);
            if (error_code < 0) {
                SPDLOG_ERROR("uv_udp_init error, error_code: {}, error_str: {}", error_code, uv_strerror(error_code));
                break;
            }

            error_code = uv_ip4_addr(ip.c_str(), port, &m_addr);
            if (error_code < 0) {
                SPDLOG_ERROR("uv_ip4_addr error, error_code: {}, error_str: {}", error_code, uv_strerror(error_code));
                break;
            }

            if (recv_cb != nullptr) {
                m_recv_cb = recv_cb;

                error_code = uv_udp_bind(&m_udp_socket, (const struct sockaddr *)&m_addr, 0);
                if (error_code < 0) {
                    SPDLOG_ERROR("uv_udp_bind error, error_code: {}, error_str: {}", error_code, uv_strerror(error_code));
                    break;
                }

                error_code = uv_udp_recv_start(&m_udp_socket, alloc_cb, m_recv_cb);
                if (error_code < 0) {
                    SPDLOG_ERROR("uv_udp_recv_start error, error_code: {}, error_str: {}", error_code, uv_strerror(error_code));
                    break;
                }

                error_code = uv_signal_init(uv_loop, &m_signal_handler);
                if (error_code < 0) {
                    SPDLOG_ERROR("uv_signal_init error, error_code: {}, error_str: {}", error_code, uv_strerror(error_code));
                    break;
                }
                m_signal_handler.data = &m_udp_socket;
                error_code = uv_signal_start(&m_signal_handler, teardown, SIGINT);  // Catch SIGINT signal (Ctrl+C)
                if (error_code < 0) {
                    SPDLOG_ERROR("uv_signal_start error, error_code: {}, error_str: {}", error_code, uv_strerror(error_code));
                    break;
                }
            }

            return true;

        } while (false);

        return false;
    }

    void stop() {
        raise(SIGINT);
    }

    static void teardown(uv_signal_t *handle, int signum) {
        if (handle->loop != nullptr) {
            if(handle->data != nullptr) {
                uv_udp_recv_stop((uv_udp_t *)handle->data);
            }
            uv_walk(handle->loop, close_handle, nullptr);
            uv_stop(handle->loop);
            uv_loop_close(handle->loop);
        }
    }

    static bool run_forever(uv_loop_t *uv_loop) {
        int error_code = uv_run(uv_loop, UV_RUN_DEFAULT);
        if (error_code < 0) {
            SPDLOG_ERROR("uv_run error, error_code: {}, error_str: {}", error_code, uv_strerror(error_code));
            return false;
        }

        return true;
    }

    static void close_handle(uv_handle_t *handle, void *arg) {
        if (!uv_is_closing(handle)) {
            uv_close(handle, nullptr);
        }
    }

    void send(std::vector<uint8_t> &data) {
        uint8_t *dup_data = new uint8_t[data.size()];
        memcpy(dup_data, data.data(), data.size());

        uv_buf_t send_buf = uv_buf_init((char *)dup_data, (unsigned int)data.size());

        uv_udp_send_t *send_req = new uv_udp_send_t;
        send_req->data = dup_data;

        int error_code = uv_udp_send(send_req, &m_udp_socket, &send_buf, 1, (const struct sockaddr *)&m_addr, send_cb);
        if (error_code < 0) {
            SPDLOG_ERROR("uv_udp_send error, error_code: {}, error_str: {}", error_code, uv_strerror(error_code));
        }
    }

protected:
    static void alloc_cb(uv_handle_t *handle, size_t suggested_size, uv_buf_t *buf) {
        char *store = new char[suggested_size];
        buf->base = store;
        buf->len = suggested_size;
    }

    static void send_cb(uv_udp_send_t *req, int status) {
        if (status < 0) {
            SPDLOG_ERROR("send_cb error, req: {}, error_code: {}, error_str: {}", fmt::ptr(req), status, uv_strerror(status));
        }

        uint8_t *dup_data = (uint8_t *)req->data;
        delete[] dup_data;
        delete req;
    }

private:
    uv_udp_t m_udp_socket;

    uv_signal_t m_signal_handler;

    uv_udp_recv_cb m_recv_cb;

    struct sockaddr_in m_addr;
};


typedef struct {   
    uint16_t year;
    uint8_t mon;
    uint8_t day;
    uint8_t hour;
    uint8_t min;
    uint8_t sec;
    uint8_t reserve;
} time_info_t;


void on_udp_recv(
    uv_udp_t *handle,
    ssize_t nread,
    const uv_buf_t *buf,
    const struct sockaddr *addr,
    unsigned flags
) {
    static int count = 0;
    if (nread > 0) {
        char sender_ip[INET6_ADDRSTRLEN];
        memset(sender_ip, 0, sizeof(char) * INET6_ADDRSTRLEN);
        if (addr->sa_family == AF_INET) { // IPv4
            struct sockaddr_in *addr_in = (struct sockaddr_in *)addr;
            uv_ip4_name(addr_in, sender_ip, sizeof(sender_ip));
        }
        else if (addr->sa_family == AF_INET6) { // IPv6
            struct sockaddr_in6 *addr_in6 = (struct sockaddr_in6 *)addr;
            uv_ip6_name(addr_in6, sender_ip, sizeof(sender_ip));
        }
        SPDLOG_INFO("recv from: {}", sender_ip);

        std::vector<uint8_t> req;
        req.resize(nread);
        std::memcpy(req.data(), buf->base, nread);

        size_t expect = sizeof(time_info_t);
        if(nread != expect) {
            SPDLOG_WARN("unexpect req, expect: {}, nread: {}", nread, expect, nread);
        }

        time_info_t time_info;
        memset(&time_info, 0, expect);
        memcpy(&time_info, req.data(), nread > expect ? expect : nread);

        SPDLOG_INFO("[{}] recv {:04d}-{:02d}-{:02d} {:02d}:{:02d}:{:02d}", count, time_info.year, time_info.mon, time_info.day, time_info.hour, time_info.min, time_info.sec);

        count++;
    }
    else if (-1 == nread) {
        uv_close((uv_handle_t *)handle, nullptr);
    }

    if (buf != nullptr && buf->base != nullptr) {
        delete[] buf->base;
    }
}


int main(int argc, char **argv) {
    uv_loop_t *uv_loop = uv_default_loop();
    if (nullptr == uv_loop) {
        return -1;
    }

    UdpSock udp_send_sock;
    if (!udp_send_sock.setup(/*"192.168.91.181"*/"127.0.0.1", 20000, uv_loop, nullptr)) {
        return false;
    }
    
    UdpSock udp_recv_sock;
    if (!udp_recv_sock.setup("0.0.0.0", 20000, uv_loop, on_udp_recv)) {
       return false;
    }

    std::thread sender_thread([&udp_send_sock]{
        for(int i = 0; i < 10; i++) {
            time_t time_now = time(NULL);
            struct tm *local_now = localtime(&time_now);

            time_info_t time_info;
            time_info.year = local_now->tm_year + 1900; // years since 1900
            time_info.mon = local_now->tm_mon + 1; // months since 0
            time_info.day = local_now->tm_mday;
            time_info.hour = local_now->tm_hour;
            time_info.min = local_now->tm_min;
            time_info.sec = local_now->tm_sec;

            std::vector<uint8_t> req(sizeof(time_info_t));
            memcpy(req.data(), &time_info, sizeof(time_info_t));

            udp_send_sock.send(req);
            SPDLOG_INFO("[{}] send {:04d}-{:02d}-{:02d} {:02d}:{:02d}:{:02d}", i, time_info.year, time_info.mon, time_info.day, time_info.hour, time_info.min, time_info.sec);

            std::this_thread::sleep_for(std::chrono::seconds(1));
        }
    });

    std::thread uv_loop_thread([uv_loop]() {
        UdpSock::run_forever(uv_loop);
    });
    
    sender_thread.join();
    udp_recv_sock.stop();
    uv_loop_thread.join();
}
