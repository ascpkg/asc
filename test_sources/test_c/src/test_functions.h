// test_functions.h
#ifndef TEST_FUNCTIONS_H
#define TEST_FUNCTIONS_H

// normal callbacks
typedef void (*simple_callback)(void);
typedef int (*data_callback)(const void *, size_t);

// complex callbacks
struct CallbackInfo
{
    void *user_data;
    simple_callback on_start;
    simple_callback on_end;
    data_callback on_data;
};

// event types
typedef enum
{
    EVENT_NONE,
    EVENT_MOUSE,
    EVENT_KEYBOARD,
    EVENT_SYSTEM
} EventType;

typedef struct
{
    EventType type;
    void *data;
} Event;

typedef void (*event_handler)(const Event *);

// register event callback
void register_event_handler(EventType type, event_handler handler);

#endif // TEST_FUNCTIONS_H