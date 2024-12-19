// c++
#include <fstream>
#include <vector>

// spdlog
#include <spdlog/spdlog.h>

// project
#include "config.h"
#include "version.h"
#include "mpv_wrapper.hpp"



size_t find_sequence(const std::vector<uint8_t> &data, const std::vector<uint8_t> &sequence, size_t start_pos) {
    for (size_t i = start_pos; i <= data.size() - sequence.size(); ++i) {
        if (std::equal(sequence.begin(), sequence.end(), data.begin() + i)) {
            return i;
        }
    }
    return std::string::npos;
}


bool find_ps_headers(FILE *input_stream) {
    // headers to search
    const std::vector<uint8_t> PS_HEADER_01BA = { 0x00, 0x00, 0x01, 0xBA };
    const std::vector<uint8_t> PS_SYSTEM_HEADER_01BB = { 0x00, 0x00, 0x01, 0xBB };

    std::vector<uint8_t> read_buffer(14);  // store read bytes
    std::vector<uint8_t> extension_buffer; // store ps header extension bytes
    std::vector<uint8_t> previous_buffer; // header in multiple buffers

    size_t offset = 0;
    while (true) {
        // read bytes from file
        auto bytes_read = fread(reinterpret_cast<char *>(read_buffer.data()), sizeof(uint8_t), read_buffer.size(), input_stream);
        if (bytes_read == 0) {
            break;
        }
        offset += bytes_read;

        // combine bytes
        std::vector<uint8_t> combined_buffer = previous_buffer;
        combined_buffer.insert(combined_buffer.end(), read_buffer.begin(), read_buffer.begin() + bytes_read);

        // search ps header
        size_t pos = 0;
        while ((pos = find_sequence(combined_buffer, PS_HEADER_01BA, pos)) != std::string::npos) {
            // ps header was found
            size_t ps_header_extension_size_pos = pos + PS_HEADER_01BA.size() + 9;  // ps header extension bytes offset
            if (ps_header_extension_size_pos >= combined_buffer.size()) {
                // ps header extension bytes offset was not available
                break;
            }

            uint8_t ps_header_extension_size = combined_buffer[ps_header_extension_size_pos] & 0x07;
            size_t ps_header_extension_start_pos = ps_header_extension_size_pos + 1;
            size_t ps_header_extension_end_pos = ps_header_extension_start_pos + ps_header_extension_size;

            if (ps_header_extension_size > 0 && ps_header_extension_end_pos <= combined_buffer.size()) {
                extension_buffer.assign(combined_buffer.begin() + ps_header_extension_start_pos, combined_buffer.begin() + ps_header_extension_end_pos);
            }

            // search ps system header
            size_t ps_bb_pos = ps_header_extension_end_pos;
            if (ps_bb_pos + 4 <= combined_buffer.size() && std::equal(PS_SYSTEM_HEADER_01BB.begin(), PS_SYSTEM_HEADER_01BB.end(), combined_buffer.begin() + ps_bb_pos)) {
                // ps system header was found
                return true;
            }

            // update pos
            pos = ps_header_extension_end_pos;
        }

        if (pos == std::string::npos) {
            // not found, save bytes expept header
            size_t save_size = std::min(combined_buffer.size(), PS_HEADER_01BA.size());
            previous_buffer.assign(combined_buffer.end() - save_size, combined_buffer.end());
        }
        else {
            // found, save bytes from header offset
            size_t save_size = combined_buffer.size() - pos;
            previous_buffer.assign(combined_buffer.end() - save_size, combined_buffer.end());
        }
    }

    return false;
}


int main(int argc, char **argv) {
    MpvWrapper wrapper;    
    if(!wrapper.start(0, "")) {
        SPDLOG_INFO("wrapper.start error");
        return -1;
    }

    const auto count = 1400;
    std::vector<uint8_t> buf;
    buf.resize(count);

    //FILE *fp = fopen("D:/asc/a.h264", "rb");
    FILE *fp = fopen("D:/asc/192.168.91.140Video3-430_2024-12-19-10-01-56.h264", "rb");

    find_ps_headers(fp);
    //while(true) {
    //    auto r = fread(buf.data(), sizeof(uint8_t), count, fp);
    //    if(0 == r) {
    //        break;
    //    }
    //    SPDLOG_INFO("feed {} bytes", r);
    //    wrapper.write(buf.data(), r);
    //}

    //while (!wrapper.is_buffer_empty()) {
    //    std::this_thread::sleep_for(std::chrono::seconds(1));
    //}
    //std::this_thread::sleep_for(std::chrono::seconds(3));

    fclose(fp);
    wrapper.stop();
}
