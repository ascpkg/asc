#pragma once

// c
#include <stdint.h>

// c++
#include <algorithm>
#include <atomic>
#include <chrono>
#include <memory>
#include <vector>

// spdlog
#include <spdlog/spdlog.h>



#define LOAD_ATOMIC_RELAXED(offset) offset.load(std::memory_order_relaxed)
#define STORE_ATOMIC_RELAXED(offset, value) offset.store(value, std::memory_order_relaxed)


inline uint32_t roundup_pow_of_two(uint32_t n)
{
	n--;
	n |= n >> 1;
	n |= n >> 2;
	n |= n >> 4;
	n |= n >> 8;
	n |= n >> 16;
	n++;

	return n;
}


inline uint32_t rounddown_pow_of_two(uint32_t n)
{
	n |= n >> 1;
	n |= n >> 2;
	n |= n >> 4;
	n |= n >> 8;
	n |= n >> 16;

	return (n + 1) >> 1;
}


// lock free, yet thread-safe single-producer single-consumer buffer
template<typename T>
class lock_free_spsc
{
public:
	lock_free_spsc()
		: m_stopping(false)
		, m_input_offset(0)
		, m_output_offset(0)
		, m_last_full_log_time(std::chrono::steady_clock::now())
		, m_last_full_log_repeat_times(0)
		, m_last_empty_log_time(std::chrono::steady_clock::now())
		, m_last_empty_log_repeat_times(0)
	{
	}

	lock_free_spsc(uint32_t buffer_size)
		: m_stopping(false)
		, m_input_offset(0)
		, m_output_offset(0)
	{
		reset(buffer_size);
	}

	~lock_free_spsc()
	{
		reset(0);
	}

	void stopping()
	{
		m_stopping = true;
	}

	void reset(uint32_t buffer_size)
	{
		m_input_offset = 0;
		m_output_offset = 0;

		m_last_full_log_repeat_times = 0;
		m_last_empty_log_repeat_times = 0;

		if (0 == buffer_size) {
			m_stopping = true;
			m_ring_buffer.clear();
		}
		else {
			m_stopping = false;

			m_last_full_log_time = std::chrono::steady_clock::now() - std::chrono::seconds(10);
			m_last_empty_log_time = std::chrono::steady_clock::now() - std::chrono::seconds(10);

			if (buffer_size & (buffer_size - 1)) {
				buffer_size = roundup_pow_of_two(buffer_size);
			}

			if (buffer_size > 0) {
				m_ring_buffer.resize(buffer_size, 0);
			}
		}
	}

	void clear()
	{
		get_all();
	}

	uint32_t buffer_size()
	{
		return (uint32_t)m_ring_buffer.size();
	}

	bool is_buffer_null()
	{
		return m_ring_buffer.empty() && 0 == m_ring_buffer.capacity();
	}

	bool is_buffer_empty()
	{
		return 0 == available_data_size();
	}

	bool is_buffer_full()
	{
		return (uint32_t)m_ring_buffer.size() == available_data_size();
	}

	uint32_t available_data_size()
	{
		return LOAD_ATOMIC_RELAXED(m_input_offset) - LOAD_ATOMIC_RELAXED(m_output_offset);
	}

	uint32_t available_space_size()
	{
		return (uint32_t)m_ring_buffer.size() - LOAD_ATOMIC_RELAXED(m_input_offset) + LOAD_ATOMIC_RELAXED(m_output_offset);
	}

	uint32_t put(const T item)
	{
		T buff[] = { item };
		return put(buff, 1);
	}

	uint32_t put(const std::vector<T> &input_buffer)
	{
		return put(input_buffer.data(), (uint32_t)input_buffer.size());
	}

	uint32_t put_if_not_full(const T *input_buffer, uint32_t length)
	{
		uint32_t offset = 0;
		while (!m_stopping && offset < length) {
			uint32_t c = put(input_buffer + offset, length - offset);
			if (0 == c) {
				break;
			}

			offset += c;
			if (!m_stopping && offset < length) {
				std::this_thread::sleep_for(std::chrono::milliseconds(5));
			}
			else {
				break;
			}
		}
		return offset;
	}

	uint32_t put(const T *input_buffer, uint32_t length)
	{
		uint32_t buffer_size = (uint32_t)m_ring_buffer.size();
		length = std::min(length, buffer_size - LOAD_ATOMIC_RELAXED(m_input_offset) + LOAD_ATOMIC_RELAXED(m_output_offset));
		if (length <= 0) {
			m_last_full_log_repeat_times++;
			auto now = std::chrono::steady_clock::now();
			auto ms = std::chrono::duration_cast<std::chrono::milliseconds>(now - m_last_full_log_time).count();
			if (ms > 10000) {
				m_last_full_log_time = now;
				SPDLOG_WARN(
					"no space, n: {}, p: {}, s({}) - w({}) + o({}) = {}\n",
					m_last_full_log_repeat_times, fmt::ptr(this), m_ring_buffer.size(), LOAD_ATOMIC_RELAXED(m_input_offset), LOAD_ATOMIC_RELAXED(m_output_offset), available_space_size()
				);
				m_last_full_log_repeat_times = 0;
			}
			return 0;
		}

		// ensure that we sample the input offset before we start putting bytes into the buffer
		std::atomic_thread_fence(std::memory_order_acquire);

		uint32_t write_offset = LOAD_ATOMIC_RELAXED(m_input_offset) & (buffer_size - 1);

		// first put the data starting from in to buffer end
		uint32_t first_part = std::min(length, buffer_size - write_offset);
		std::memcpy(m_ring_buffer.data() + write_offset, input_buffer, sizeof(T) * first_part);

		// then put the rest (if any) at the beginning of the buffer
		std::memcpy(m_ring_buffer.data(), input_buffer + first_part, sizeof(T) * (length - first_part));

		// ensure that we add the bytes to the buffer before we update the input offset
		std::atomic_thread_fence(std::memory_order_release);

		STORE_ATOMIC_RELAXED(m_input_offset, LOAD_ATOMIC_RELAXED(m_input_offset) + length);

		return length;
	}

	uint32_t peek(T &item)
	{
		uint32_t length = std::min(1u, LOAD_ATOMIC_RELAXED(m_input_offset) - LOAD_ATOMIC_RELAXED(m_output_offset));
		if (length <= 0) {
			return 0;
		}

		uint32_t read_offset = LOAD_ATOMIC_RELAXED(m_output_offset) & ((uint32_t)m_ring_buffer.size() - 1);
		item = m_ring_buffer[read_offset];

		return length;
	}

	uint32_t get(T &item)
	{
		T buf[] = { item };
		auto count = get(buf, 1);
		item = buf[0];
		return count;
	}

	uint32_t get(std::vector<T> &output_buffer)
	{
		return get(output_buffer.data(), (uint32_t)output_buffer.size());
	}

	std::vector<T> get_all()
	{
		std::vector<T> result(available_data_size());
		if (result.empty()) {
			return result;
		}

		get(result);
		return result;
	}

	uint32_t get_if_not_empty(T *output_buffer, uint32_t length)
	{
		uint32_t offset = 0;
		while (!m_stopping && 0 == offset) {
			uint32_t c = get(output_buffer + offset, length - offset);
			if (c == 0 && offset > 0) {
				break;
			}

			offset += c;
			if (!m_stopping && 0 == offset) {
				std::this_thread::sleep_for(std::chrono::milliseconds(5));
			}
			else {
				break;
			}
		}
		return offset;
	}

	uint32_t get(T *output_buffer, uint32_t length)
	{
		length = std::min(length, LOAD_ATOMIC_RELAXED(m_input_offset) - LOAD_ATOMIC_RELAXED(m_output_offset));
		if (length <= 0) {
			m_last_empty_log_repeat_times++;
			auto now = std::chrono::steady_clock::now();
			auto ms = std::chrono::duration_cast<std::chrono::milliseconds>(now - m_last_empty_log_time).count();
			if (ms > 10000) {
				m_last_empty_log_time = now;
				SPDLOG_WARN(
					"no data, n: {}, p: {}, s({}) - w({}) + o({}) = {}\n",
					m_last_empty_log_repeat_times, fmt::ptr(this), m_ring_buffer.size(), LOAD_ATOMIC_RELAXED(m_input_offset), LOAD_ATOMIC_RELAXED(m_output_offset), available_data_size()
				);
				m_last_empty_log_repeat_times = 0;
			}

			return 0;
		}

		// ensure that we sample the output offset before we start removing bytes from the buffer
		std::atomic_thread_fence(std::memory_order_acquire);

		uint32_t buffer_size = (uint32_t)m_ring_buffer.size();

		uint32_t read_offset = LOAD_ATOMIC_RELAXED(m_output_offset) & (buffer_size - 1);

		// first get the data from out until the end of the buffer
		uint32_t first_part = std::min(length, buffer_size - read_offset);
		std::memcpy(output_buffer, m_ring_buffer.data() + read_offset, sizeof(T) * first_part);

		// then get the rest (if any) from the beginning of the buffer
		std::memcpy(output_buffer + first_part, m_ring_buffer.data(), sizeof(T) * (length - first_part));

		// ensure that we remove the bytes from the buffer before we update the output offset
		std::atomic_thread_fence(std::memory_order_release);

		STORE_ATOMIC_RELAXED(m_output_offset, LOAD_ATOMIC_RELAXED(m_output_offset) + length);

		return length;
	}


private:
	bool m_stopping;
	std::vector<T> m_ring_buffer;  // the buffer holding the data
	std::atomic<uint32_t> m_input_offset;  // data is added at offset: m_input_offset % (size - 1)
	std::atomic<uint32_t> m_output_offset;  // data is extracted from offset: m_output_offset % (size - 1)
	// last full log time
	std::chrono::steady_clock::time_point m_last_full_log_time;
	// log full log repeat times
	uint32_t m_last_full_log_repeat_times;
	// last empty log time
	std::chrono::steady_clock::time_point m_last_empty_log_time;
	// log empty log repeat times
	uint32_t m_last_empty_log_repeat_times;
};

