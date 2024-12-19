#pragma once

// c++
#include <chrono>
#include <map>
#include <string>

// project
#include "spsc.hpp"

// libmpv
struct mpv_handle;
struct mpv_event_log_message;



class MpvWrapper {
public:
	MpvWrapper(uint32_t buffer_size = 4 * 1024 * 1024);
	virtual ~MpvWrapper();

	// start player
	bool start(
		int64_t container_wid, std::string video_url = "",
		std::string profile = "low-latency", std::string vo = "gpu",
		std::string hwdec = "auto", std::string gpu_api = "auto",
		std::string gpu_context = "auto", std::string log_level = "v"
	);
	// stop player
	void stop();

	// break infinite loop
	void stopping();

	// validate spsc
	bool is_buffer_null();

	// is buffer empty
	bool is_buffer_empty();

	// write av stream to spsc
	bool write(const uint8_t *buf, uint32_t length);

	// read av stream from spsc
	int64_t read(char *buf, uint64_t nbytes);

	// play
	bool play();
	// pause
	bool pause();

	// play one frame
	bool step();

	// is mute
	bool get_mute_state();
	// set mute or unmute
	void set_mute_state(const bool state);

	// get volume
	int get_volume();
	// set volume
	void set_volume(const int v);

	// get video resolution
	bool get_resolution(int64_t &width, int64_t &height);

	// get speed
	double get_speed();
	// set speed
	bool set_speed(double v);

	// get bitrate
	int get_bitrate();
	
	// get fps
	int get_fps();

	// take screenshot from video
	bool screenshot(std::string &path);


protected:
	// wrap mpv_create to create mpv handle ctx
	bool create_handle();

	// wrap mpv_initialize to init mpv handle ctx
	bool initialize_handle();

	// wrap mpv_stream_cb_add_ro to register custom stream protocol
	bool register_stream_callbacks();

	// wrap mpv_request_log_messages to set log level
	bool set_log_level(std::string min_level);

	// wrap mpv_command to call mpv command
	bool call_command(std::vector<std::string> args);

	// wrap mpv_set_option to set mpv option
	bool set_option(std::string key, bool value);
	bool set_option(std::string key, int64_t value);
	bool set_option(std::string key, double value);
	bool set_option(std::string key, std::string value);

	// wrap mpv_get_option to get mpv properity
	bool get_property(std::string key, bool &value);
	bool get_property(std::string key, int64_t &value);
	bool get_property(std::string key, double &value);
	bool get_property(std::string key, std::string &value);

	// wrap mpv_set_property to set mpv properity
	bool set_property(std::string key, bool value);
	bool set_property(std::string key, int64_t value);
	bool set_property(std::string key, double value);
	bool set_property(std::string key, std::string value);

	// show/hide container window
	void set_container_window_visiable(bool state);

	// restart when the codec was changed
	bool restart_when_codec_changed(struct mpv_event_log_message *msg);

	// get decoded resolution
	bool get_decoded_resolution(struct mpv_event_log_message *msg);

	// estimate bitrate
	void estimate_bitrate(uint32_t length);

	// fast speed to reduce latency
	void reduce_latency();

	// poll events
	static void poll_events(void *ptr);


private:
	// id
	uint32_t m_id;
	// auto-incrementing index
	static std::atomic<uint16_t> s_index;
	// flag to break infinite loop
	bool m_stopping;
	// is restarting
	std::atomic<bool> m_is_restarting;
	// mpv handle ctx
	mpv_handle *m_mpv_context;
	// event thread
	std::thread *m_event_thread;
	// input size in one second
	uint32_t m_input_size_2s;
	// last bitrate update time
	std::chrono::steady_clock::time_point m_last_bitrate_update_time;
	// estimated bitrate
	uint32_t m_estimated_bitrate;
	// min bitrate according to resolution
	uint32_t m_min_bitrate;
	// estimate speed
	double m_estimated_speed;
	// video width
	uint32_t m_width;
	// video height
	uint32_t m_height;
	// player's parent window id
	int64_t m_container_wid;
	// video to play
	std::string m_video_url;
	// mpv profile option
	std::string m_profile;
	// mpv vo option
	std::string m_vo;
	// mpv hwdec option
	std::string m_hwdec;
	// mpv gpu-api option
	std::string m_gpu_api;
	// mpv gpu-context option
	std::string m_gpu_context;
	// mpv log level
	std::string m_log_level;
	// spsc size
	uint32_t m_buffer_size;
	// spsc
	lock_free_spsc<uint8_t> m_spsc;
};

