// test_io.h
#ifndef TEST_IO_H
#define TEST_IO_H

#include <stdio.h>

// file op
FILE *open_file(const char *filename, const char *mode);
size_t read_file(FILE *file, void *buffer, size_t size);
size_t write_file(FILE *file, const void *buffer, size_t size);
int close_file(FILE *file);

// buffer op
struct Buffer
{
    void *data;
    size_t size;
    size_t capacity;
};

struct Buffer *create_buffer(size_t initial_capacity);
void delete_buffer(struct Buffer *buffer);

// op stream op
typedef struct
{
    FILE *stream;
    char *buffer;
    size_t buffer_size;
} StreamWrapper;

StreamWrapper *create_stream(const char *filename);
void destroy_stream(StreamWrapper *wrapper);

#endif // TEST_IO_H